use std::{io::Cursor, path::PathBuf};

use file_icon_provider::get_file_icon;
use image::{DynamicImage, ImageFormat, RgbaImage};

#[tauri::command]
fn get_icon(path: PathBuf, size: Option<u16>) -> Result<Vec<u8>, String> {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_icon])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
