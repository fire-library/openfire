pub mod hrr_at_flashover;
pub mod maximum_enclosure_temperature;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Section8Method {
    HRRAtFlashover,
    MaximumEnclosureTemperature,
}

impl Section8Method {
    pub fn friendly_reference(&self) -> String {
        match self {
            Section8Method::HRRAtFlashover => {
                format!("Equations 28, 29 & 33")
            }
            Section8Method::MaximumEnclosureTemperature => {
                format!("Equations 42, 43 & 44")
            }
        }
    }
}
