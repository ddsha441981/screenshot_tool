use crate::{config::Config, error::ScreenshotError};
use std::path::PathBuf;
use chrono::Local;
use uuid::Uuid;

pub fn validate_output_path(path: &PathBuf) -> Result<(), ScreenshotError> {
    if path.to_string_lossy().contains("..") {
        return Err(ScreenshotError::PermissionDenied(
            "Path traversal not allowed".to_string()
        ));
    }
    
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    Ok(())
}

pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '|' | '?' | '*' | '/' | '\\' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect()
}

pub fn generate_filename(config: &Config, prefix: &str) -> Result<String, ScreenshotError> {
    let filename = if let Some(ref custom) = config.custom_filename {
        format!("{}.{}", sanitize_filename(custom), config.default_format)
    } else {
        let timestamp = Local::now();
        let formatted = timestamp.format(&config.filename_template).to_string();
        let name = if prefix.is_empty() {
            formatted
        } else {
            format!("{}_{}", prefix, formatted)
        };

        format!("{}.{}", name, config.default_format)
    };

    Ok(filename)
}

pub fn ensure_unique_filename(path: PathBuf) -> PathBuf {
    if !path.exists() {
        return path;
    }

    let stem = path.file_stem().unwrap().to_string_lossy();
    let extension = path.extension().unwrap().to_string_lossy();
    let parent = path.parent().unwrap();

    let mut counter = 1;
    loop {
        let new_name = format!("{}_{}.{}", stem, counter, extension);
        let new_path = parent.join(new_name);
        if !new_path.exists() {
            return new_path;
        }
        counter += 1;
        if counter > 9999 {
            let uuid = Uuid::new_v4();
            let unique_name = format!("{}_{}_{}.{}", stem, counter, uuid, extension);
            return parent.join(unique_name);
        }
    }
}
