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
    pub fn load(filename: &str) -> Result<Filetypes, String> {
        let yaml = File::open(filename).and_then(|mut f| {
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;
            Ok(contents)
        });
        let unwrapped_yaml = match yaml {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        serde_json::from_str::<Filetypes>(&unwrapped_yaml).map_err(|e| e.to_string())
    }

    pub fn save(&self, filename: &str) -> Result<(), String> {
        let yaml = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;

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
