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

    pub fn about_method(&self) -> String {
        match self {
            Chapter14Method::HeatReleaseFromTempAndPosition => {
                include_str!("../../../../resources/sfpe_handbook/chapter_14/alpert/about.md")
                    .to_string()
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            Chapter14Method::HeatReleaseFromTempAndPosition => {
                include_str!("../../../../resources/sfpe_handbook/chapter_14/alpert/limitations.md")
                    .to_string()
            }
        }
    }
}
