use super::Settings;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::AppHandle;

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum License {
    Dec2024,
}

pub type LicenseAgreements = HashMap<License, bool>;

pub const LATEST_LICENSE: License = License::Dec2024;

pub fn agree_to_license<R: tauri::Runtime>(
    app: &AppHandle<R>,
    license: License,
) -> Result<(), String> {
    if let Ok(mut settings) = Settings::load(&app) {
        settings.analytics = Some(true);
        settings.license.insert(license, true);
        let path = Settings::path(app)?;

        settings.save(path)
    } else {
        Err("Failed to agree to license".to_string())
    }
}
