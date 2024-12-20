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
}
