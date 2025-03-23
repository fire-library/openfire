pub mod alpert;

use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter14Method {
    HeatReleaseFromTempAndPosition,
}

impl Chapter14Method {
    pub fn id(&self) -> String {
        match self {
            Chapter14Method::HeatReleaseFromTempAndPosition => {
                "heat_release_from_temp_and_position".to_string()
            }
        }
    }

    pub fn friendly_reference(&self) -> String {
        match self {
            Chapter14Method::HeatReleaseFromTempAndPosition => "Eq. 14.2 & 14.3".to_string(),
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            Chapter14Method::HeatReleaseFromTempAndPosition => {
                include_str!("../resources/chapter_14/alpert/about.md").to_string()
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            Chapter14Method::HeatReleaseFromTempAndPosition => {
                include_str!("../resources/chapter_14/alpert/limitations.md").to_string()
            }
        }
    }
}
