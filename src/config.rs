use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::ScreenshotError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub output_directory: PathBuf,
    pub default_format: String,
    pub default_quality: u8,
    pub filename_template: String,
    pub custom_filename: Option<String>,
    pub auto_open: bool,
    pub cleanup_after_days: Option<u32>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_directory: get_default_screenshot_dir(),
            default_format: "png".to_string(),
            default_quality: 90,
            filename_template: "screenshot_%Y%m%d_%H%M%S".to_string(),
            custom_filename: None,
            auto_open: false,
            cleanup_after_days: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, ScreenshotError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| ScreenshotError::ConfigError(
                config::ConfigError::Message("No config directory found".to_string())
            ))?
            .join("screenshot");

        let config_path = config_dir.join("config.toml");

        if !config_path.exists() {
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let settings = config::Config::builder()
            .add_source(config::File::with_name(config_path.to_str().unwrap()))
            .build()?;

        Ok(settings.try_deserialize()?)
    }

    pub fn save(&self) -> Result<(), ScreenshotError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| ScreenshotError::ConfigError(
                config::ConfigError::Message("No config directory found".to_string())
            ))?
            .join("screenshot");

        std::fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("config.toml");
        let toml_string = toml::to_string_pretty(self)
            .map_err(|e| ScreenshotError::ConfigError(
                config::ConfigError::Message(format!("Failed to serialize config: {}", e))
            ))?;

        std::fs::write(config_path, toml_string)?;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), ScreenshotError> {
        match self.default_format.to_lowercase().as_str() {
            "png" | "jpg" | "jpeg" | "webp" => {},
            _ => return Err(ScreenshotError::InvalidFormat(self.default_format.clone())),
        }
        if !(1..=100).contains(&self.default_quality) {
            return Err(ScreenshotError::InvalidQuality(self.default_quality));
        }

        Ok(())
    }
}

fn get_default_screenshot_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Pictures")
            .join("Screenshots")
    } else if cfg!(target_os = "macos") {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Desktop")
    } else {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Pictures")
            .join("Screenshots")
    }
}
