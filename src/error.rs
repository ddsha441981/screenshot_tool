use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScreenshotError {
    #[error("No screens found")]
    NoScreensFound,

    #[error("Screen {0} not found")]
    ScreenNotFound(usize),

    #[error("Screen capture failed: {0}")]
    CaptureFailed(String),

    #[error("File save error: {0}")]
    SaveError(#[from] std::io::Error),

    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Invalid quality value: {0} (must be 1-100)")]
    InvalidQuality(u8),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("External command failed: {0}")]
    ExternalCommandFailed(String),

    #[error("Clipboard error: {0}")]
    ClipboardError(String),
}

impl ScreenshotError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::NoScreensFound | Self::ScreenNotFound(_) => 2,
            Self::CaptureFailed(_) => 3,
            Self::SaveError(_) => 4,
            Self::PermissionDenied(_) => 13,
            Self::PlatformNotSupported(_) => 5,
            _ => 1,
        }
    }
}
