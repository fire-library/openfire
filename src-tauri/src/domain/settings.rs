pub mod license;
pub mod software_update;

use serde::{Deserialize, Serialize};
use software_update::SoftwareUpdate;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub license: license::LicenseAgreements,
    pub analytics: Option<bool>,

    #[serde(default = "default_update_settings")]
    pub update: SoftwareUpdate,

    #[serde(default = "create_user_id")]
    pub user: String,
}

fn default_update_settings() -> SoftwareUpdate {
    SoftwareUpdate {
        auto_update: false,
        skipped_update: None,
    }
}

fn create_user_id() -> String {
    Uuid::new_v4().to_string()
}

impl Settings {
    pub fn new() -> Self {
        Self {
            license: license::LicenseAgreements::new(),
            analytics: None,
            update: default_update_settings(),
            user: create_user_id(),
        }
    }

    pub fn path<R: tauri::Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
        if let Ok(mut path) = app.path().app_data_dir() {
            if path.exists() == false {
                std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
            }
            path.push("settings.yaml");
            Ok(path)
        } else {
            Err("Failed to get app data directory".to_string())
        }
    }

    pub fn save(&self, path: PathBuf) -> Result<(), String> {
        let yaml = serde_yaml::to_string(self).map_err(|e| e.to_string())?;
        std::fs::write(path, yaml).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn load<R: tauri::Runtime>(app: &AppHandle<R>) -> Result<Self, String> {
        if let Ok(path) = Self::path(app) {
            if let Ok(mut file) = File::open(path) {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .map_err(|e| e.to_string())?;

                return serde_yaml::from_str::<Settings>(&contents).map_err(|e| e.to_string());
            }
        }

        Ok(Self::new())
    }
}
