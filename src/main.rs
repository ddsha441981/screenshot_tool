use clap::{Parser, Subcommand};
use std::path::PathBuf;
use screenshots::Screen;
use chrono::prelude::*;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(name = "screenshot")]
#[command(about = "A fast screenshot capture tool")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(short, long, default_value = "0")]
    delay: u64,
    
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
   
    Fullscreen {
        #[arg(short, long, default_value = "0")]
        screen: usize,
    },
    Selection,
    Window,
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.delay > 0 {
        println!("Waiting {} seconds...", cli.delay);
        std::thread::sleep(std::time::Duration::from_secs(cli.delay));
    }

    let output_dir = cli.output.unwrap_or_else(get_default_screenshot_dir);
    std::fs::create_dir_all(&output_dir)?;

    match cli.command {
        Commands::Fullscreen { screen } => {
            let path = capture_fullscreen(screen, output_dir)?;
            if !cli.quiet {
                println!("✅ Screenshot saved: {}", path.display());
            }
        },
        Commands::Selection => {
            let path = capture_selection(output_dir)?;
            if !cli.quiet {
                println!("✅ Screenshot saved: {}", path.display());
            }
        },
        Commands::Window => {
            let path = capture_window(output_dir)?;
            if !cli.quiet {
                println!("✅ Screenshot saved: {}", path.display());
            }
        },
        Commands::List => {
            list_screens();
        },
    }

    Ok(())
}

fn get_default_screenshot_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join("Pictures").join("Screenshots")
    } else if cfg!(target_os = "macos") {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join("Desktop")
    } else {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join("Pictures").join("Screenshots")
    }
}

fn capture_fullscreen(screen_id: usize, save_dir: PathBuf) -> Result<PathBuf> {
    let screens = Screen::all()?;
    let screen = screens.get(screen_id).ok_or_else(|| anyhow::anyhow!("Screen {} not found", screen_id))?;

    let image = screen.capture()?;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("screenshot_{}.png", timestamp);
    let path = save_dir.join(filename);

    image::save_buffer_with_format(
        &path,
        image.rgba(),
        image.width(),
        image.height(),
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    )?;

    Ok(path)
}

fn capture_selection(save_dir: PathBuf) -> Result<PathBuf> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("selection_{}.png", timestamp);
    let path = save_dir.join(filename);

    #[cfg(target_os = "linux")]
    {
        let output = std::process::Command::new("gnome-screenshot")
            .arg("-a").arg("-f").arg(&path)
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Selection capture failed"));
        }
    }

    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("screencapture")
            .arg("-s").arg("-x").arg(&path)
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Selection capture failed"));
        }
    }

    #[cfg(target_os = "windows")]
    {
        return Err(anyhow::anyhow!("Selection capture not supported on Windows"));
    }

    Ok(path)
}

fn capture_window(save_dir: PathBuf) -> Result<PathBuf> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("window_{}.png", timestamp);
    let path = save_dir.join(filename);

    #[cfg(target_os = "linux")]
    {
        let output = std::process::Command::new("gnome-screenshot")
            .arg("-w").arg("-f").arg(&path)
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Window capture failed"));
        }
    }

    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("screencapture")
            .arg("-W").arg("-x").arg(&path)
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Window capture failed"));
        }
    }

    #[cfg(target_os = "windows")]
    {
        return Err(anyhow::anyhow!("Window capture not supported on Windows"));
    }

    Ok(path)
}

fn list_screens() {
    match Screen::all() {
        Ok(screens) => {
            println!("Available screens:");
            for (i, screen) in screens.iter().enumerate() {
                let info = &screen.display_info;
                println!("  {} - {}x{} at ({}, {})", i, info.width, info.height, info.x, info.y);
            }
        },
        Err(e) => {
            eprintln!("Failed to list screens: {}", e);
        }
    }
}














// use screenshots::Screen;
// use std::path::PathBuf;
// use chrono::prelude::*;
// use anyhow::Result;
// use std::env;
//
// #[derive(Debug, Clone, PartialEq)]
// enum CaptureMode {
//     FullScreen,
//     Selection,
//     Window,
// }
//
// fn main() -> Result<()> {
//     let args: Vec<String> = env::args().collect();
//
//     if args.len() != 2 {
//         println!("Screenshot Tool - Terminal Version");
//         println!("Usage: {} <mode>", args[0]);
//         println!("Modes:");
//         println!("  fullscreen  - Capture full screen");
//         println!("  selection   - Capture selected area");
//         println!("  window      - Capture active window");
//         println!();
//         println!("Examples:");
//         println!("  {} fullscreen", args[0]);
//         println!("  {} selection", args[0]);
//         println!("  {} window", args[0]);
//         return Ok(());
//     }
//
//     let mode = match args[1].to_lowercase().as_str() {
//         "fullscreen" | "full" | "f" => CaptureMode::FullScreen,
//         "selection" | "select" | "area" | "s" => CaptureMode::Selection,
//         "window" | "win" | "w" => CaptureMode::Window,
//         _ => {
//             eprintln!("Error: Invalid mode '{}'. Use 'fullscreen', 'selection', or 'window'", args[1]);
//             return Ok(());
//         }
//     };
//
//     // Get OS-specific default screenshot directory
//     let save_dir = get_default_screenshot_dir();
//
//     // Create directory if it doesn't exist
//     std::fs::create_dir_all(&save_dir)?;
//
//     println!("Capturing {} screenshot...", match mode {
//         CaptureMode::FullScreen => "fullscreen",
//         CaptureMode::Selection => "selection",
//         CaptureMode::Window => "window",
//     });
//
//     match capture_screenshot(mode, save_dir) {
//         Ok(path) => {
//             println!("✅ Screenshot saved: {}", path.display());
//         }
//         Err(e) => {
//             eprintln!("❌ Error: {}", e);
//         }
//     }
//
//     Ok(())
// }
//
// fn get_default_screenshot_dir() -> PathBuf {
//     if cfg!(target_os = "windows") {
//         dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join("Pictures").join("Screenshots")
//     } else if cfg!(target_os = "macos") {
//         dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join("Desktop")
//     } else {
//         // Linux and other Unix-like systems
//         dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join("Pictures").join("Screenshots")
//     }
// }
//
// fn capture_screenshot(mode: CaptureMode, save_dir: PathBuf) -> Result<PathBuf> {
//     match mode {
//         CaptureMode::FullScreen => capture_fullscreen(save_dir),
//         CaptureMode::Selection => capture_selection(save_dir),
//         CaptureMode::Window => capture_window(save_dir),
//     }
// }
//
// fn capture_fullscreen(save_dir: PathBuf) -> Result<PathBuf> {
//     let screens = Screen::all()?;
//     let screen = screens.first().ok_or_else(|| anyhow::anyhow!("No screens found"))?;
//
//     let image = screen.capture()?;
//     let timestamp = Local::now().format("%Y%m%d_%H%M%S");
//     let filename = format!("screenshot_{}.png", timestamp);
//     let path = save_dir.join(filename);
//
//     // Save directly using image crate
//     image::save_buffer_with_format(
//         &path,
//         image.rgba(),
//         image.width(),
//         image.height(),
//         image::ColorType::Rgba8,
//         image::ImageFormat::Png,
//     )?;
//
//     Ok(path)
// }
//
// fn capture_selection(save_dir: PathBuf) -> Result<PathBuf> {
//     let timestamp = Local::now().format("%Y%m%d_%H%M%S");
//     let temp_path = save_dir.join(format!("selection_{}.png", timestamp));
//
//     let output = if cfg!(target_os = "linux") {
//         std::process::Command::new("gnome-screenshot")
//             .arg("-a") // area selection
//             .arg("-f")
//             .arg(&temp_path)
//             .output()?
//     } else if cfg!(target_os = "macos") {
//         std::process::Command::new("screencapture")
//             .arg("-s") // selection mode
//             .arg(&temp_path)
//             .output()?
//     } else {
//         return Err(anyhow::anyhow!("Selection capture not supported on this OS"));
//     };
//
//     if output.status.success() && temp_path.exists() {
//         Ok(temp_path)
//     } else {
//         Err(anyhow::anyhow!("Selection capture failed"))
//     }
// }
//
// fn capture_window(save_dir: PathBuf) -> Result<PathBuf> {
//     let timestamp = Local::now().format("%Y%m%d_%H%M%S");
//     let temp_path = save_dir.join(format!("window_{}.png", timestamp));
//
//     let output = if cfg!(target_os = "linux") {
//         std::process::Command::new("gnome-screenshot")
//             .arg("-w") // window selection
//             .arg("-f")
//             .arg(&temp_path)
//             .output()?
//     } else if cfg!(target_os = "macos") {
//         std::process::Command::new("screencapture")
//             .arg("-W") // window mode
//             .arg(&temp_path)
//             .output()?
//     } else {
//         return Err(anyhow::anyhow!("Window capture not supported on this OS"));
//     };
//
//     if output.status.success() && temp_path.exists() {
//         Ok(temp_path)
//     } else {
//         Err(anyhow::anyhow!("Window capture failed"))
//     }
// }
