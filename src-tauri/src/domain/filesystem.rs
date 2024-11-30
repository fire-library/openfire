pub mod saved_method;

use saved_method::SavedMethod;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
#[serde(tag = "type")]
pub enum Filetypes {
    Method(SavedMethod),
}

impl Filetypes {
    pub fn save(&self, filename: &str) -> Result<(), String> {
        let yaml = serde_yaml::to_string(self).map_err(|e| e.to_string())?;

        let output = File::create(filename);

        match output {
            Ok(mut output_file) => {
                output_file
                    .write_all(yaml.as_bytes())
                    .expect("Failed to write to file");
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
