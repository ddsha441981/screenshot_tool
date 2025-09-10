use crate::error::ScreenshotError;
use std::path::PathBuf;
use log::{debug, warn};

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
use arboard::Clipboard;

pub fn copy_file_to_clipboard(path: &PathBuf) -> Result<(), ScreenshotError> {
    debug!("Copying image to clipboard: {}", path.display());

    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    {
        let image_data = std::fs::read(path)?;
        let img = image::load_from_memory(&image_data)?;
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();

        let mut clipboard = Clipboard::new()
            .map_err(|e| ScreenshotError::ClipboardError(e.to_string()))?;

        let img_data = arboard::ImageData {
            width: width as usize,
            height: height as usize,
            bytes: rgb_img.into_raw().into(),
        };

        clipboard.set_image(img_data)
            .map_err(|e| ScreenshotError::ClipboardError(e.to_string()))?;

        Ok(())
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        warn!("Clipboard not supported on this platform");
        Err(ScreenshotError::PlatformNotSupported(
            "Clipboard not supported on this platform".to_string()
        ))
    }
}
