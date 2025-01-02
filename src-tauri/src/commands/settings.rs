use crate::domain::settings::{license, Settings};
use tauri::AppHandle;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tauri::command]
#[specta::specta]
pub fn has_agreed_to_latest_license<R: tauri::Runtime>(
    app_handle: AppHandle<R>,
) -> Result<bool, String> {
    let settings = Settings::load(&app_handle)?;
    let has_agreed = settings
        .license
        .get(&license::LATEST_LICENSE)
        .unwrap_or(&false);

    Ok(*has_agreed)
}

#[tauri::command]
#[specta::specta]
pub fn agree_to_license<R: tauri::Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    license::agree_to_license(&app_handle, license::LATEST_LICENSE)
}

#[tauri::command]
#[specta::specta]
pub fn openfire_version() -> Result<String, String> {
    Ok(VERSION.to_string())
}
