use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoftwareUpdate {
    #[serde(default = "default_auto_update")]
    pub auto_update: bool,

    #[serde(default = "none_update")]
    pub update: Option<Update>,
}

fn default_auto_update() -> bool {
    false
}

fn none_update() -> Option<Update> {
    None
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Update {
    version: bool,
    ignored: bool,
}
