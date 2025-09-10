use screenshots::Screen;
use crate::{config::Config, error::ScreenshotError, capture::save_image_with_config};
use std::path::PathBuf;
use log::{debug, warn};

pub fn capture(screen_id: usize, config: &Config) -> Result<PathBuf, ScreenshotError> {
    debug!("Starting fullscreen capture for screen {}", screen_id);

    let screens = Screen::all()
        .map_err(|e| ScreenshotError::CaptureFailed(e.to_string()))?;

    if screens.is_empty() {
        return Err(ScreenshotError::NoScreensFound);
    }

    let screen = screens.get(screen_id)
        .ok_or(ScreenshotError::ScreenNotFound(screen_id))?;

    debug!("Capturing screen: {}x{}",
           screen.display_info.width,
           screen.display_info.height);

    let image = screen.capture()
        .map_err(|e| ScreenshotError::CaptureFailed(e.to_string()))?;

    let width = image.width();
    let height = image.height();
    let rgba_data = image.rgba();

    debug!("Image captured: {}x{} pixels, {} bytes",
           width, height, rgba_data.len());

    save_image_with_config(
        rgba_data,
        width,
        height,
        config,
        &format!("screen_{}", screen_id),
    )
}

pub fn capture_all_screens(config: &Config) -> Result<Vec<PathBuf>, ScreenshotError> {
    let screens = Screen::all()
        .map_err(|e| ScreenshotError::CaptureFailed(e.to_string()))?;

    if screens.is_empty() {
        return Err(ScreenshotError::NoScreensFound);
    }

    let mut paths = Vec::new();

    for (i, screen) in screens.iter().enumerate() {
        match screen.capture() {
            Ok(image) => {
                match save_image_with_config(
                    image.rgba(),
                    image.width(),
                    image.height(),
                    config,
                    &format!("screen_{}", i),
                ) {
                    Ok(path) => paths.push(path),
                    Err(e) => warn!("Failed to save screen {}: {}", i, e),
                }
            },
            Err(e) => warn!("Failed to capture screen {}: {}", i, e),
        }
    }

    if paths.is_empty() {
        return Err(ScreenshotError::CaptureFailed("No screens captured".to_string()));
    }

    Ok(paths)
}
