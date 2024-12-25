use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use std::{collections::HashMap, io::Read};
use tauri::{AppHandle, Manager};

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum License {
    Dec2024,
}

pub type LicenseAgreements = HashMap<License, bool>;

pub const LATEST_LICENSE: License = License::Dec2024;

pub fn license_agreement_path<R: tauri::Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    if let Ok(mut path) = app.path().app_data_dir() {
        if path.exists() == false {
            std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
        }
        path.push("license_agreements.yaml");
        Ok(path)
    } else {
        Err("Failed to get app data directory".to_string())
    }
}

pub fn agree_to_license<R: tauri::Runtime>(
    app: &AppHandle<R>,
    license: License,
) -> Result<(), String> {
    if let Ok(mut agreements) = load_lisence_agreements(&app) {
        agreements.insert(license, true);
        let path = license_agreement_path(app)?;

        let yaml = serde_yaml::to_string(&agreements).map_err(|e| e.to_string())?;

        std::fs::write(path, yaml).map_err(|e| e.to_string())?;

        Ok(())
    } else {
        Err("Failed to agree to license".to_string())
    }
}

pub fn load_lisence_agreements<R: tauri::Runtime>(
    app: &AppHandle<R>,
) -> Result<LicenseAgreements, String> {
    if let Ok(path) = license_agreement_path(app) {
        if let Ok(mut file) = File::open(path) {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|e| e.to_string())?;

            return serde_yaml::from_str::<LicenseAgreements>(&contents).map_err(|e| e.to_string());
        }
    }

    Ok(LicenseAgreements::new())
}
