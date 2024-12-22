use crate::domain::license;
use tauri::AppHandle;

#[tauri::command]
#[specta::specta]
pub fn has_agreed_to_latest_license<R: tauri::Runtime>(
    app_handle: AppHandle<R>,
) -> Result<bool, String> {
    let agreements = license::load_lisence_agreements(&app_handle)?;
    let has_agreed = agreements.get(&license::LATEST_LICENSE).unwrap_or(&false);

    Ok(*has_agreed)
}

#[tauri::command]
#[specta::specta]
pub fn agree_to_license<R: tauri::Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    license::agree_to_license(&app_handle, license::LATEST_LICENSE)
}
