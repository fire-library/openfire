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
pub fn get_auto_update<R: tauri::Runtime>(app_handle: AppHandle<R>) -> Result<bool, String> {
    let settings = Settings::load(&app_handle)?;
    let update = settings.update.auto_update;

    Ok(update)
}

#[tauri::command]
#[specta::specta]
pub fn set_auto_update<R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    activated: bool,
) -> Result<(), String> {
    let mut settings = Settings::load(&app_handle)?;
    settings.update.auto_update = activated;
    let path = Settings::path(&app_handle)?;
    settings.save(path)?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn get_update_skipped<R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    version: String,
) -> Result<bool, String> {
    let settings = Settings::load(&app_handle)?;
    let skipped = settings
        .update
        .skipped_update
        .as_ref()
        .map_or(false, |v| v == &version);

    Ok(skipped)
}

#[tauri::command]
#[specta::specta]
pub fn set_update_skipped<R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    version: String,
) -> Result<(), String> {
    let mut settings = Settings::load(&app_handle)?;
    settings.update.skipped_update = Some(version);
    let path = Settings::path(&app_handle)?;
    settings.save(path)?;

    Ok(())
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
