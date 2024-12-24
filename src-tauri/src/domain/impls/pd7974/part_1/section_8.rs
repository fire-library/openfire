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
                format!("Eq. 28, 29 & 33")
            }
            Section8Method::MaximumEnclosureTemperature => {
                format!("Eq. 42, 43 & 44")
            }
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            Section8Method::HRRAtFlashover => {
                include_str!("../../../../../resources/pd7974/part_1/section_8/hrr_at_flashover/about.md")
                    .to_string()
            }
            Section8Method::MaximumEnclosureTemperature => {
                include_str!("../../../../../resources/pd7974/part_1/section_8/maximum_enclosure_temperature/about.md")
                    .to_string()
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            Section8Method::HRRAtFlashover => {
                include_str!("../../../../../resources/pd7974/part_1/section_8/hrr_at_flashover/limitations.md")
                    .to_string()
            }
            Section8Method::MaximumEnclosureTemperature => {
                include_str!("../../../../../resources/pd7974/part_1/section_8/maximum_enclosure_temperature/limitations.md")
                    .to_string()
            }
        }
    }
}
