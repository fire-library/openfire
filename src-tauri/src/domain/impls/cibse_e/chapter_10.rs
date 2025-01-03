pub mod extract_points;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter10Method {
    ExtractPoints,
}

impl Chapter10Method {
    pub fn friendly_reference(&self) -> String {
        match self {
            Chapter10Method::ExtractPoints => {
                format!("Eq. 10.1, 10.2 & 10.3")
            }
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            Chapter10Method::ExtractPoints => {
                include_str!("../../../../resources/cibse_e/chapter_10/extract_points/about.md")
                    .to_string()
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            Chapter10Method::ExtractPoints => include_str!(
                "../../../../resources/cibse_e/chapter_10/extract_points/limitations.md"
            )
            .to_string(),
        }
    }
}
