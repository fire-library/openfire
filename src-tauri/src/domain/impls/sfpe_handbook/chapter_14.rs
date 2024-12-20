pub mod alpert;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter14Method {
    HeatReleaseFromTempAndPosition,
}

impl Chapter14Method {
    pub fn friendly_reference(&self) -> String {
        match self {
            Chapter14Method::HeatReleaseFromTempAndPosition => "Alpert's correlation".to_string(),
        }
    }
}
