pub mod equation_28;
pub mod equation_29;
pub mod equation_33;
pub mod equation_41;
pub mod equation_42;
pub mod equation_43;
pub mod equation_44;
pub mod hrr_at_flashover_runner;
pub mod maximum_enclosure_temperature_runner;
pub mod maximum_hrr;

use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Section8Method {
    HRRAtFlashover,
    MaximumEnclosureTemperature,
    MaximumHRR,
}

impl Section8Method {
    pub fn id(&self) -> String {
        match self {
            Section8Method::HRRAtFlashover => "hrr_at_flashover".to_string(),
            Section8Method::MaximumEnclosureTemperature => {
                "maximum_enclosure_temperature".to_string()
            },
            Section8Method::MaximumHRR => "maximum hrr".to_string(),
        }
    }
    pub fn friendly_reference(&self) -> String {
        match self {
            Section8Method::HRRAtFlashover => {
                format!("Eq. 28 & 29")
            }
            Section8Method::MaximumEnclosureTemperature => {
                format!("Eq. 41, 42, 43 & 44")
            }
            Section8Method::MaximumHRR => {
                format!("Eq. 4 & 33")
            }
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            Section8Method::HRRAtFlashover => {
                include_str!("../../resources/part_1/section_8/hrr_at_flashover/about.md")
                    .to_string()
            }
            Section8Method::MaximumEnclosureTemperature => include_str!(
                "../../resources/part_1/section_8/maximum_enclosure_temperature/about.md"
            )
            .to_string(),
            Section8Method::MaximumHRR => include_str!(
                "../../resources/part_1/section_8/maximum_hrr/about.md"
            ).to_string(),
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            Section8Method::HRRAtFlashover => {
                include_str!("../../resources/part_1/section_8/hrr_at_flashover/limitations.md")
                    .to_string()
            }
            Section8Method::MaximumEnclosureTemperature => include_str!(
                "../../resources/part_1/section_8/maximum_enclosure_temperature/limitations.md"
            )
            .to_string(),
            Section8Method::MaximumHRR => include_str!(
                "../../resources/part_1/section_8/maximum_hrr/limitations.md"
            ).to_string(),
        }
    }
}
