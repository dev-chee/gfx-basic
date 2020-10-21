#[derive(Debug)]
pub enum EngineError {
    BuildMainWindowError(winit::error::OsError),
    CreateGfxInstanceError(gfx_hal::UnsupportedBackend),
    CreateGfxSurfaceError(gfx_hal::window::InitError),
    GetQueueFamilyError,
    CreateGpuError(gfx_hal::device::CreationError),
    GetQueueGroupError,
    CreateCommandPoolError(gfx_hal::device::OutOfMemory),
    CreateRenderPassError(gfx_hal::device::OutOfMemory),
    CreatePipelineLayoutError(gfx_hal::device::OutOfMemory),
    CreateShaderModule(gfx_hal::device::ShaderError),
    CreateGraphicsPipelineError(gfx_hal::pso::CreationError),
    CreateFenceError(gfx_hal::device::OutOfMemory),
    CreateSemaphoreError(gfx_hal::device::OutOfMemory),
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::BuildMainWindowError(_) => f.write_str("failed to create main window"),
            EngineError::CreateGfxInstanceError(_) => f.write_str("unsupported gfx backend"),
            EngineError::CreateGfxSurfaceError(_) => f.write_str("failed to create gfx surface"),
            EngineError::GetQueueFamilyError => {
                f.write_str("unsupported gfx queue family for graphic pipeline")
            }
            EngineError::CreateGpuError(_) => f.write_str("failed to create gfx gpu"),
            EngineError::GetQueueGroupError => {
                f.write_str("failed to get queue group at selected queue family")
            }
            EngineError::CreateCommandPoolError(_) => {
                f.write_str("failed to create gfx command pool")
            }
            EngineError::CreateRenderPassError(_) => {
                f.write_str("failed to create gfx render pass")
            }
            EngineError::CreatePipelineLayoutError(_) => {
                f.write_str("failed to create gfx pipeline layout")
            }
            EngineError::CreateShaderModule(_) => f.write_str("failed to create shader module"),
            EngineError::CreateGraphicsPipelineError(_) => {
                f.write_str("failed to create graphics pipeline")
            }
            EngineError::CreateFenceError(_) => f.write_str("failed to create fence"),
            EngineError::CreateSemaphoreError(_) => f.write_str("failed to create semaphore"),
        }
    }
}

impl std::error::Error for EngineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EngineError::BuildMainWindowError(err) => Some(err),
            EngineError::CreateGfxInstanceError(_) => None,
            EngineError::CreateGfxSurfaceError(err) => Some(err),
            EngineError::GetQueueFamilyError => None,
            EngineError::CreateGpuError(err) => Some(err),
            EngineError::GetQueueGroupError => None,
            EngineError::CreateCommandPoolError(err) => Some(err),
            EngineError::CreateRenderPassError(err) => Some(err),
            EngineError::CreatePipelineLayoutError(err) => Some(err),
            EngineError::CreateShaderModule(err) => Some(err),
            EngineError::CreateGraphicsPipelineError(err) => Some(err),
            EngineError::CreateFenceError(err) => Some(err),
            EngineError::CreateSemaphoreError(err) => Some(err),
        }
    }
}
