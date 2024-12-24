pub mod burning_regime;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter10Method {
    BurningRegime,
}

impl Chapter10Method {
    pub fn friendly_reference(&self) -> String {
        match self {
            Chapter10Method::BurningRegime => "Eq. 10.18".to_string(),
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            Chapter10Method::BurningRegime => {
                include_str!("../../../../resources/introduction_to_fire_dynamics/chapter_10/burning_regime/about.md")
                    .to_string()
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            Chapter10Method::BurningRegime => {
                include_str!("../../../../resources/introduction_to_fire_dynamics/chapter_10/burning_regime/limitations.md")
                    .to_string()
            }
        }
    }
}
