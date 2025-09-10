use crate::{config::Config, error::ScreenshotError};
use std::path::PathBuf;
use std::process::Command;
use log::{debug, info};

pub fn capture(config: &Config) -> Result<PathBuf, ScreenshotError> {
    debug!("Starting window capture");
    
    std::fs::create_dir_all(&config.output_directory)?;

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("window_{}.{}", timestamp, config.default_format);
    let path = config.output_directory.join(filename);

    let success = if cfg!(target_os = "linux") {
        capture_linux_window(&path)?
    } else if cfg!(target_os = "macos") {
        capture_macos_window(&path)?
    } else if cfg!(target_os = "windows") {
        capture_windows_window(&path)?
    } else {
        return Err(ScreenshotError::PlatformNotSupported(
            "Window capture not supported on this platform".to_string()
        ));
    };

    if success && path.exists() {
        info!("Window capture saved: {}", path.display());
        Ok(path)
    } else {
        Err(ScreenshotError::CaptureFailed("Window capture failed".to_string()))
    }
}

fn capture_linux_window(path: &PathBuf) -> Result<bool, ScreenshotError> {
    #[cfg(target_os = "linux")]
    {
        debug!("Using gnome-screenshot for Linux window capture");

        let output = Command::new("gnome-screenshot")
            .arg("-w")
            .arg("-f")
            .arg(path)
            .output()
            .map_err(|e| ScreenshotError::ExternalCommandFailed(
                format!("Failed to run gnome-screenshot: {}", e)
            ))?;

        Ok(output.status.success())
    }

    #[cfg(not(target_os = "linux"))]
    {
        Err(ScreenshotError::PlatformNotSupported(
            "Linux-specific capture called on non-Linux platform".to_string()
        ))
    }
}

fn capture_macos_window(path: &PathBuf) -> Result<bool, ScreenshotError> {
    #[cfg(target_os = "macos")]
    {
        debug!("Using screencapture for macOS window capture");

        let output = Command::new("screencapture")
            .arg("-W")
            .arg("-x") // Don't play sound
            .arg(path)
            .output()
            .map_err(|e| ScreenshotError::ExternalCommandFailed(
                format!("Failed to run screencapture: {}", e)
            ))?;

        Ok(output.status.success())
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(ScreenshotError::PlatformNotSupported(
            "macOS-specific capture called on non-macOS platform".to_string()
        ))
    }
}

fn capture_windows_window(_path: &PathBuf) -> Result<bool, ScreenshotError> {
    #[cfg(target_os = "windows")]
    {
        // Windows implementation would require more complex code
        Err(ScreenshotError::PlatformNotSupported(
            "Window capture not yet implemented for Windows".to_string()
        ))
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err(ScreenshotError::PlatformNotSupported(
            "Windows-specific capture called on non-Windows platform".to_string()
        ))
    }
}
