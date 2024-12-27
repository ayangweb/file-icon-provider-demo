use file_icon_provider::get_file_icon;
use image::{DynamicImage, ImageFormat, RgbaImage};
use std::{io::Cursor, path::PathBuf};
use tauri::command;

#[command]
pub async fn get_icon(path: PathBuf, size: Option<u16>) -> Result<Vec<u8>, String> {
    let size = size.unwrap_or(32);

    let icon = get_file_icon(path, size).map_err(|err| err.to_string())?;

    let image = RgbaImage::from_raw(icon.width, icon.height, icon.pixels)
        .map(DynamicImage::ImageRgba8)
        .ok_or_else(|| "Failed to create RGBA image".to_string())?;

    let mut bytes = Cursor::new(Vec::new());

    image
        .write_to(&mut bytes, ImageFormat::Png)
        .map_err(|err| err.to_string())?;

    Ok(bytes.into_inner())
}
