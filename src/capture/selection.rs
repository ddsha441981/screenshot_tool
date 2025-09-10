use crate::{config::Config, error::ScreenshotError};
use std::path::PathBuf;
use std::process::Command;
use log::{debug, info};

pub fn capture(config: &Config) -> Result<PathBuf, ScreenshotError> {
    debug!("Starting selection capture");
    
    std::fs::create_dir_all(&config.output_directory)?;

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("selection_{}.{}", timestamp, config.default_format);
    let path = config.output_directory.join(filename);

    let success = if cfg!(target_os = "linux") {
        capture_linux_selection(&path)?
    } else if cfg!(target_os = "macos") {
        capture_macos_selection(&path)?
    } else if cfg!(target_os = "windows") {
        capture_windows_selection(&path)?
    } else {
        return Err(ScreenshotError::PlatformNotSupported(
            "Selection capture not supported on this platform".to_string()
        ));
    };

    if success && path.exists() {
        info!("Selection capture saved: {}", path.display());
        Ok(path)
    } else {
        Err(ScreenshotError::CaptureFailed("Selection capture failed".to_string()))
    }
}

fn capture_linux_selection(path: &PathBuf) -> Result<bool, ScreenshotError> {
    #[cfg(target_os = "linux")]
    {
        debug!("Trying alternative screenshot tools without flash");
        
        if let Ok(output) = Command::new("maim")
            .arg("-s")  // selection mode
            .arg(path)
            .output()
        {
            if output.status.success() {
                return Ok(true);
            }
        }
        if let Ok(output) = Command::new("scrot")
            .arg("-s") 
            .arg(path)
            .output()
        {
            if output.status.success() {
                return Ok(true);
            }
        }
        if let Ok(output) = Command::new("import")
            .arg(path)
            .output()
        {
            if output.status.success() {
                return Ok(true);
            }
        }
        
        debug!("Falling back to gnome-screenshot (may have flash)");
        let output = Command::new("gnome-screenshot")
            .arg("-a")
            .arg("-f")
            .arg(path)
            .output()
            .map_err(|e| ScreenshotError::ExternalCommandFailed(
                format!("All screenshot tools failed. Last error: {}", e)
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

// fn capture_linux_selection(path: &PathBuf) -> Result<bool, ScreenshotError> {
//     #[cfg(target_os = "linux")]
//     {
//         debug!("Using gnome-screenshot for Linux selection");
//
//         let output = Command::new("gnome-screenshot")
//             .arg("-a")
//             .arg("-f")
//             .arg(path)
//             .output()
//             .map_err(|e| ScreenshotError::ExternalCommandFailed(
//                 format!("Failed to run gnome-screenshot: {}", e)
//             ))?;
//
//         if !output.status.success() {
//             debug!("gnome-screenshot failed, trying import");
//             let output = Command::new("import")
//                 .arg(path)
//                 .output()
//                 .map_err(|e| ScreenshotError::ExternalCommandFailed(
//                     format!("Failed to run import: {}", e)
//                 ))?;
//
//             return Ok(output.status.success());
//         }
//
//         Ok(true)
//     }
//
//     #[cfg(not(target_os = "linux"))]
//     {
//         Err(ScreenshotError::PlatformNotSupported(
//             "Linux-specific capture called on non-Linux platform".to_string()
//         ))
//     }
// }

fn capture_macos_selection(path: &PathBuf) -> Result<bool, ScreenshotError> {
    #[cfg(target_os = "macos")]
    {
        debug!("Using screencapture for macOS selection (no flash)");

        let output = Command::new("screencapture")
            .arg("-s")   // selection mode
            .arg("-x")   // no sound
            .arg("-o")   // no shadow
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


// fn capture_macos_selection(path: &PathBuf) -> Result<bool, ScreenshotError> {
//     #[cfg(target_os = "macos")]
//     {
//         debug!("Using screencapture for macOS selection");
//
//         let output = Command::new("screencapture")
//             .arg("-s")
//             .arg("-x") // Don't play sound
//             .arg(path)
//             .output()
//             .map_err(|e| ScreenshotError::ExternalCommandFailed(
//                 format!("Failed to run screencapture: {}", e)
//             ))?;
//
//         Ok(output.status.success())
//     }
//
//     #[cfg(not(target_os = "macos"))]
//     {
//         Err(ScreenshotError::PlatformNotSupported(
//             "macOS-specific capture called on non-macOS platform".to_string()
//         ))
//     }
// }

fn capture_windows_selection(_path: &PathBuf) -> Result<bool, ScreenshotError> {
    #[cfg(target_os = "windows")]
    {
        // Windows implementation would require more complex code
        // For now, return not supported
        Err(ScreenshotError::PlatformNotSupported(
            "Selection capture not yet implemented for Windows".to_string()
        ))
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err(ScreenshotError::PlatformNotSupported(
            "Windows-specific capture called on non-Windows platform".to_string()
        ))
    }
}
