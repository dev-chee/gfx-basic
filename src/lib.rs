use engine::ShaderKind;
use error::EngineError;
use event::Event;
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowBuilder};

pub mod engine;
pub mod error;
pub mod event;
pub mod prelude;

pub type Engine = engine::Engine<gfx_backend::Backend>;

pub trait Application {
    fn name(&self) -> &str;

    fn version(&self) -> u32;

    fn shader(&self, kind: ShaderKind) -> &str;

    fn start(&mut self, engine: &mut Engine);

    fn update(&mut self, engine: &mut Engine);

    fn render(&mut self, engine: &mut Engine);

    fn late_update(&mut self, engine: &mut Engine);

    fn exit(&mut self, engine: &mut Engine);

    fn event(&mut self, engine: &mut Engine, event: &mut Event);
}

#[derive(Debug)]
pub struct Entry<A> {
    window_builder: WindowBuilder,
    app: A,
}

impl<A> Entry<A>
where
    A: Application + 'static,
{
    pub fn new(app: A) -> Self {
        Self {
            window_builder: Default::default(),
            app,
        }
    }

    pub fn window_title(mut self, title: &str) -> Self {
        self.window_builder = self.window_builder.with_title(title);
        self
    }

    pub fn window_size(mut self, size: [u32; 2]) -> Self {
        self.window_builder = self
            .window_builder
            .with_inner_size(PhysicalSize::<u32>::from(size));
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.window_builder = self.window_builder.with_resizable(resizable);
        self
    }

    pub fn build(self) -> Result<(), EngineError> {
        let Self {
            mut app,
            window_builder,
        } = self;

        // create event loop
        let event_loop = winit::event_loop::EventLoop::new();

        // create main window
        let window = {
            match window_builder.build(&event_loop) {
                Ok(window) => window,
                Err(err) => return Err(EngineError::BuildMainWindowError(err)),
            }
        };

        // create engine
        let engine = Engine::new(window, &mut app)?;
        run_loop(engine, app, event_loop);
        Ok(())
    }
}

fn run_loop<A>(engine: engine::Engine<gfx_backend::Backend>, app: A, event_loop: EventLoop<()>)
where
    A: Application + 'static,
{
    let mut engine = Some(engine);
    let mut app = Some(app);

    engine.as_mut().unwrap().start(app.as_mut().unwrap());

    event_loop.run(
        move |event, _event_loop_window_target, control_flow| match event {
            winit::event::Event::NewEvents(_) => {}
            winit::event::Event::WindowEvent { window_id, event } => match event {
                winit::event::WindowEvent::Resized(new_size) => {
                    engine
                        .as_mut()
                        .unwrap()
                        .window_size_changed(new_size.into());
                }
                winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    engine
                        .as_mut()
                        .unwrap()
                        .window_size_changed((*new_inner_size).into());
                }
                winit::event::WindowEvent::Moved(_) => {}
                winit::event::WindowEvent::CloseRequested => {
                    if window_id == engine.as_mut().unwrap().window.id() {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                }
                winit::event::WindowEvent::Destroyed => {}
                winit::event::WindowEvent::DroppedFile(_) => {}
                winit::event::WindowEvent::HoveredFile(_) => {}
                winit::event::WindowEvent::HoveredFileCancelled => {}
                winit::event::WindowEvent::ReceivedCharacter(_) => {}
                winit::event::WindowEvent::Focused(_) => {}
                winit::event::WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                } => {}
                winit::event::WindowEvent::ModifiersChanged(_) => {}
                winit::event::WindowEvent::CursorMoved {
                    device_id,
                    position,
                    ..
                } => {}
                winit::event::WindowEvent::CursorEntered { device_id } => {}
                winit::event::WindowEvent::CursorLeft { device_id } => {}
                winit::event::WindowEvent::MouseWheel {
                    device_id,
                    delta,
                    phase,
                    ..
                } => {}
                winit::event::WindowEvent::MouseInput {
                    device_id,
                    state,
                    button,
                    ..
                } => {}
                winit::event::WindowEvent::TouchpadPressure {
                    device_id,
                    pressure,
                    stage,
                } => {}
                winit::event::WindowEvent::AxisMotion {
                    device_id,
                    axis,
                    value,
                } => {}
                winit::event::WindowEvent::Touch(_) => {}
                winit::event::WindowEvent::ThemeChanged(_) => {}
            },
            winit::event::Event::DeviceEvent { device_id, event } => {}
            winit::event::Event::UserEvent(_) => {}
            winit::event::Event::Suspended => {}
            winit::event::Event::Resumed => {}
            winit::event::Event::MainEventsCleared => {
                engine.as_mut().unwrap().update(app.as_mut().unwrap());
            }
            winit::event::Event::RedrawRequested(_) => {
                engine.as_mut().unwrap().render(app.as_mut().unwrap());
            }
            winit::event::Event::RedrawEventsCleared => {
                engine.as_mut().unwrap().late_update(app.as_mut().unwrap());
            }
            winit::event::Event::LoopDestroyed => {
                engine.take().unwrap().exit(app.take().unwrap());
            }
        },
    );
}
