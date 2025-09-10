pub mod fullscreen;
pub mod selection;
pub mod window;

use crate::{config::Config, error::ScreenshotError, utils::path::generate_filename};
use std::path::PathBuf;
use image::{ImageFormat, ImageBuffer, ColorType};
use log::{debug, info};

pub fn save_image_with_config(
    image_data: &[u8],
    width: u32,
    height: u32,
    config: &Config,
    prefix: &str,
) -> Result<PathBuf, ScreenshotError> {
    std::fs::create_dir_all(&config.output_directory)?;

    let filename = generate_filename(config, prefix)?;
    let path = config.output_directory.join(filename);

    debug!("Saving image to: {}", path.display());

    let format = match config.default_format.to_lowercase().as_str() {
        "png" => ImageFormat::Png,
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "webp" => ImageFormat::WebP,
        _ => return Err(ScreenshotError::InvalidFormat(config.default_format.clone())),
    };

    match format {
        ImageFormat::Png => {
            image::save_buffer_with_format(
                &path,
                image_data,
                width,
                height,
                ColorType::Rgba8,
                ImageFormat::Png,
            )?;
        },
        ImageFormat::Jpeg => {
            let rgb_data: Vec<u8> = image_data
                .chunks(4)
                .flat_map(|rgba| &rgba[..3])
                .copied()
                .collect();

            let img = ImageBuffer::from_raw(width, height, rgb_data)
                .ok_or_else(|| ScreenshotError::ImageError(
                    image::ImageError::Parameter(image::error::ParameterError::from_kind(
                        image::error::ParameterErrorKind::DimensionMismatch
                    ))
                ))?;

            let dynamic_img = image::DynamicImage::ImageRgb8(img);
            dynamic_img.save_with_format(&path, ImageFormat::Jpeg)?;
        },
        ImageFormat::WebP => {
            return Err(ScreenshotError::PlatformNotSupported(
                "WebP format not yet supported".to_string()
            ));
        },
        _ => unreachable!(),
    }

    info!("Image saved successfully: {}", path.display());
    Ok(path)
}
