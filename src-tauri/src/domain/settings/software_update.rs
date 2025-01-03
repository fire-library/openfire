use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoftwareUpdate {
    #[serde(default = "default_auto_update")]
    pub auto_update: bool,

    #[serde(default = "none_string")]
    pub skipped_update: Option<String>,
}

fn default_auto_update() -> bool {
    false
}

fn none_update() -> Option<Update> {
    None
}

fn none_string() -> Option<String> {
    None
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Update {
    version: bool,
    ignored: bool,
}
