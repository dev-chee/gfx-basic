use crate::{error::EngineError, event::Event, Application};
use gfx_hal::{
    adapter::Adapter,
    command::{
        ClearColor, ClearValue, CommandBuffer, CommandBufferFlags, Level as CmdLevel,
        SubpassContents,
    },
    device::Device,
    format::{ChannelType, Format},
    image,
    pass::{Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDesc},
    pool::{CommandPool, CommandPoolCreateFlags},
    prelude::CommandQueue,
    prelude::PhysicalDevice,
    prelude::QueueFamily,
    pso::{
        BlendState, ColorBlendDesc, ColorMask, EntryPoint, Face, GraphicsPipelineDesc,
        InputAssemblerDesc, Primitive, PrimitiveAssemblerDesc, Rasterizer, Rect, Specialization,
        Viewport,
    },
    queue::Submission,
    window::{Extent2D, PresentationSurface, Surface, SwapchainConfig},
    Instance,
};
use shaderc::Compiler;
use std::borrow::Borrow;
use winit::window::Window;

pub use shaderc::ShaderKind;

#[derive(Debug)]
struct SwapchainStat {
    should_configure_swapchain: bool,
    surface_extent: Extent2D,
    surface_color_format: Format,
}

impl SwapchainStat {
    fn configure<B: gfx_hal::Backend>(
        &mut self,
        adapter: &Adapter<B>,
        surface: &mut B::Surface,
        device: &B::Device,
    ) {
        if self.should_configure_swapchain {
            let caps = surface.capabilities(&adapter.physical_device);

            let mut swapchain_config =
                SwapchainConfig::from_caps(&caps, self.surface_color_format, self.surface_extent);

            if caps.image_count.contains(&3) {
                swapchain_config.image_count = 3;
            }

            self.surface_extent = swapchain_config.extent;

            unsafe {
                surface
                    .configure_swapchain(device, swapchain_config)
                    .expect("Failed to configure swapchain");
            };

            self.should_configure_swapchain = false;
        }
    }
}

#[derive(Debug)]
pub struct Engine<B: gfx_hal::Backend> {
    pub window: Window,
    instance: B::Instance,
    adapter: gfx_hal::adapter::Adapter<B>,
    device: B::Device,
    surface: B::Surface,
    render_passes: Vec<B::RenderPass>,
    pipeline_layouts: Vec<B::PipelineLayout>,
    pipelines: Vec<B::GraphicsPipeline>,
    queue_group: gfx_hal::queue::QueueGroup<B>,
    command_pool: B::CommandPool,
    submission_complete_fence: B::Fence,
    rendering_complete_semaphore: B::Semaphore,
    swapchain_stat: SwapchainStat,
}

impl Engine<gfx_backend::Backend> {
    pub fn new<A: Application>(window: Window, app: &mut A) -> Result<Self, EngineError> {
        Self::build(window, app)
    }
}

impl Engine<gfx_backend::Backend> {
    pub fn start<A: Application>(&mut self, app: &mut A) {
        app.start(self);
    }

    pub fn update<A: Application>(&mut self, app: &mut A) {
        app.update(self);
        self.window.request_redraw();
    }

    pub fn render<A: Application>(&mut self, app: &mut A) {
        self.on_render();
        app.render(self);
    }

    pub fn late_update<A: Application>(&mut self, app: &mut A) {
        app.late_update(self);
    }

    pub fn exit<A: Application>(mut self, mut app: A) {
        app.exit(&mut self);
        self.destroy();
    }

    pub fn event<A: Application>(&mut self, app: &mut A, event: &mut Event) {
        if self.on_event(event) {
            return;
        }
        app.event(self, event);
    }
}

impl<B: gfx_hal::Backend> Engine<B> {
    pub fn build<A: Application>(window: Window, app: &mut A) -> Result<Self, EngineError> {
        // create instance
        let instance = {
            match B::Instance::create(app.name(), app.version()) {
                Ok(instance) => instance,
                Err(err) => return Err(EngineError::CreateGfxInstanceError(err)),
            }
        };

        // create surface
        let surface = unsafe {
            match instance.create_surface(&window) {
                Ok(surface) => surface,
                Err(err) => return Err(EngineError::CreateGfxSurfaceError(err)),
            }
        };

        // get adapter
        let adapter = instance.enumerate_adapters().remove(0);

        let queue_family = {
            match adapter.queue_families.iter().find(|family| {
                surface.supports_queue_family(family) && family.queue_type().supports_graphics()
            }) {
                Some(queue_family) => queue_family,
                None => return Err(EngineError::GetQueueFamilyError),
            }
        };

        // create device and queue group
        let (device, queue_group) = {
            let mut gpu = unsafe {
                match adapter
                    .physical_device
                    .open(&[(queue_family, &[1.0])], gfx_hal::Features::empty())
                {
                    Ok(gpu) => gpu,
                    Err(err) => return Err(EngineError::CreateGpuError(err)),
                }
            };

            let queue_group = {
                match gpu.queue_groups.pop() {
                    Some(queue_group) => queue_group,
                    None => return Err(EngineError::GetQueueGroupError),
                }
            };

            (gpu.device, queue_group)
        };

        // create command pool
        let command_pool = unsafe {
            match device.create_command_pool(queue_group.family, CommandPoolCreateFlags::empty()) {
                Ok(command_pool) => command_pool,
                Err(err) => return Err(EngineError::CreateCommandPoolError(err)),
            }
        };

        // find surface color format
        let surface_color_format = {
            let supported_formats = surface
                .supported_formats(&adapter.physical_device)
                .unwrap_or(vec![]);

            let default_format = *supported_formats.get(0).unwrap_or(&Format::Rgba8Srgb);

            supported_formats
                .into_iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .unwrap_or(default_format)
        };

        // create render pass
        let render_pass = {
            let color_attachment = Attachment {
                format: Some(surface_color_format),
                samples: 1,
                ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: image::Layout::Undefined..image::Layout::Present,
            };

            let subpass = SubpassDesc {
                colors: &[(0, image::Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };

            unsafe {
                match device.create_render_pass(&[color_attachment], &[subpass], &[]) {
                    Ok(render_pass) => render_pass,
                    Err(err) => return Err(EngineError::CreateRenderPassError(err)),
                }
            }
        };

        // create pipeline layout
        let pipeline_layout = unsafe {
            match device.create_pipeline_layout(&[], &[]) {
                Ok(pipeline_layout) => pipeline_layout,
                Err(err) => return Err(EngineError::CreatePipelineLayoutError(err)),
            }
        };

        // make pipeline
        let pipeline = unsafe {
            let vert = compile_shader(app.shader(ShaderKind::Vertex), ShaderKind::Vertex);
            let frag = compile_shader(app.shader(ShaderKind::Fragment), ShaderKind::Fragment);
            make_pipeline::<B>(&device, &render_pass, &pipeline_layout, &vert, &frag)?
        };

        // create present fence
        let submission_complete_fence = {
            match device.create_fence(true) {
                Ok(submission_complete_fence) => submission_complete_fence,
                Err(err) => return Err(EngineError::CreateFenceError(err)),
            }
        };

        // create rendering finished semaphore
        let rendering_complete_semaphore = {
            match device.create_semaphore() {
                Ok(rendering_complete_semaphore) => rendering_complete_semaphore,
                Err(err) => return Err(EngineError::CreateSemaphoreError(err)),
            }
        };

        let swapchain_stat = {
            let inner_size = window.inner_size();
            SwapchainStat {
                should_configure_swapchain: true,
                surface_extent: Extent2D {
                    width: inner_size.width,
                    height: inner_size.height,
                },
                surface_color_format,
            }
        };

        let engine = Self {
            window,
            instance,
            adapter,
            device,
            surface,
            queue_group,
            command_pool,
            render_passes: vec![render_pass],
            pipeline_layouts: vec![pipeline_layout],
            pipelines: vec![pipeline],
            submission_complete_fence,
            rendering_complete_semaphore,
            swapchain_stat,
        };

        Ok(engine)
    }
}

impl<B: gfx_hal::Backend> Engine<B> {
    pub fn window_size_changed(&mut self, new_size: [u32; 2]) {
        self.swapchain_stat.should_configure_swapchain = true;
        self.swapchain_stat.surface_extent = Extent2D {
            width: new_size[0],
            height: new_size[1],
        };
    }

    fn on_render(&mut self) {
        let render_pass = &self.render_passes[0];
        let pipeline = &self.pipelines[0];

        unsafe {
            let render_timeout_ns = 1_000_000_000;

            self.device
                .wait_for_fence(&self.submission_complete_fence, render_timeout_ns)
                .expect("Out of memory or device lost");

            self.device
                .reset_fence(&self.submission_complete_fence)
                .expect("Out of memory");

            // This clears out the previous frame's command buffer and
            // returns it to the pool for use this frame.
            self.command_pool.reset(false);
        }

        self.swapchain_stat
            .configure(&self.adapter, &mut self.surface, &self.device);

        let surface_image = unsafe {
            // We refuse to wait more than a second, to avoid hanging.
            let acquire_timeout_ns = 1_000_000_000;

            match self.surface.acquire_image(acquire_timeout_ns) {
                Ok((image, _)) => image,
                Err(_) => {
                    self.swapchain_stat.should_configure_swapchain = true;
                    return;
                }
            }
        };

        let framebuffer = unsafe {
            self.device
                .create_framebuffer(
                    render_pass,
                    vec![surface_image.borrow()],
                    image::Extent {
                        width: self.swapchain_stat.surface_extent.width,
                        height: self.swapchain_stat.surface_extent.height,
                        depth: 1,
                    },
                )
                .unwrap()
        };

        let viewport = {
            Viewport {
                rect: Rect {
                    x: 0,
                    y: 0,
                    w: self.swapchain_stat.surface_extent.width as i16,
                    h: self.swapchain_stat.surface_extent.height as i16,
                },
                depth: 0.0..1.0,
            }
        };

        unsafe {
            let mut command_buffers = Vec::<B::CommandBuffer>::new();
            self.command_pool
                .allocate(1, CmdLevel::Primary, &mut command_buffers);
            command_buffers[0].begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

            command_buffers[0].set_viewports(0, &[viewport.clone()]);
            command_buffers[0].set_scissors(0, &[viewport.rect]);

            command_buffers[0].begin_render_pass(
                render_pass,
                &framebuffer,
                viewport.rect,
                &[ClearValue {
                    color: ClearColor {
                        float32: [0.0, 0.0, 0.0, 1.0],
                    },
                }],
                SubpassContents::Inline,
            );

            command_buffers[0].bind_graphics_pipeline(pipeline);
            command_buffers[0].draw(0..3, 0..1);
            command_buffers[0].end_render_pass();
            command_buffers[0].finish();

            let submission = Submission {
                command_buffers: &command_buffers,
                wait_semaphores: None,
                signal_semaphores: vec![&self.rendering_complete_semaphore],
            };

            self.queue_group.queues[0].submit(submission, Some(&self.submission_complete_fence));

            let result = self.queue_group.queues[0].present(
                &mut self.surface,
                surface_image,
                Some(&self.rendering_complete_semaphore),
            );

            self.swapchain_stat.should_configure_swapchain |= result.is_err();
            self.device.destroy_framebuffer(framebuffer);
            self.command_pool.free(command_buffers);
        }
    }

    fn on_event(&mut self, event: &mut Event) -> bool {
        false
    }

    fn destroy(mut self) {
        unsafe {
            self.device
                .destroy_semaphore(self.rendering_complete_semaphore);
            self.device.destroy_fence(self.submission_complete_fence);

            for pipeline in self.pipelines {
                self.device.destroy_graphics_pipeline(pipeline);
            }

            for pipeline_layout in self.pipeline_layouts {
                self.device.destroy_pipeline_layout(pipeline_layout);
            }

            for render_pass in self.render_passes {
                self.device.destroy_render_pass(render_pass);
            }

            self.device.destroy_command_pool(self.command_pool);
            self.surface.unconfigure_swapchain(&self.device);
            self.instance.destroy_surface(self.surface);
        }
    }
}

unsafe fn make_pipeline<B: gfx_hal::Backend>(
    device: &B::Device,
    render_pass: &B::RenderPass,
    pipeline_layout: &B::PipelineLayout,
    vertex_shader: &Vec<u32>,
    fragment_shader: &Vec<u32>,
) -> Result<B::GraphicsPipeline, EngineError> {
    let vertex_shader_module = {
        match device.create_shader_module(vertex_shader) {
            Ok(vertex_shader_module) => vertex_shader_module,
            Err(err) => return Err(EngineError::CreateShaderModule(err)),
        }
    };

    let fragment_shader_module = {
        match device.create_shader_module(fragment_shader) {
            Ok(fragment_shader_module) => fragment_shader_module,
            Err(err) => return Err(EngineError::CreateShaderModule(err)),
        }
    };

    let vs_entry = EntryPoint {
        entry: "main",
        module: &vertex_shader_module,
        specialization: Specialization::default(),
    };
    let fs_entry = EntryPoint {
        entry: "main",
        module: &fragment_shader_module,
        specialization: Specialization::default(),
    };

    let primitive_assembler = PrimitiveAssemblerDesc::Vertex {
        buffers: &[],
        attributes: &[],
        input_assembler: InputAssemblerDesc::new(Primitive::TriangleList),
        vertex: vs_entry,
        tessellation: None,
        geometry: None,
    };

    let mut pipeline_desc = GraphicsPipelineDesc::new(
        primitive_assembler,
        Rasterizer {
            cull_face: Face::BACK,
            ..Rasterizer::FILL
        },
        Some(fs_entry),
        pipeline_layout,
        Subpass {
            index: 0,
            main_pass: render_pass,
        },
    );

    pipeline_desc.blender.targets.push(ColorBlendDesc {
        mask: ColorMask::ALL,
        blend: Some(BlendState::ALPHA),
    });

    let pipeline = {
        match device.create_graphics_pipeline(&pipeline_desc, None) {
            Ok(pipeline) => pipeline,
            Err(err) => return Err(EngineError::CreateGraphicsPipelineError(err)),
        }
    };

    device.destroy_shader_module(vertex_shader_module);
    device.destroy_shader_module(fragment_shader_module);

    Ok(pipeline)
}

fn compile_shader(glsl: &str, shader_kind: ShaderKind) -> Vec<u32> {
    let mut compiler = Compiler::new().unwrap();
    let compiled_shader = compiler
        .compile_into_spirv(glsl, shader_kind, "unnamed", "main", None)
        .expect("Failed to compile shader");

    compiled_shader.as_binary().to_vec()
}
