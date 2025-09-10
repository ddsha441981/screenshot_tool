pub mod config;
pub mod error;
pub mod capture;
pub mod utils;

pub use error::ScreenshotError;
pub type Result<T> = std::result::Result<T, ScreenshotError>;
