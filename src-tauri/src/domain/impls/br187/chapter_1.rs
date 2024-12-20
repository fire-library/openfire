pub mod equation_1;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter1Equation {
    One,
}

impl Chapter1Equation {
    pub fn friendly_reference(&self) -> String {
        match self {
            Chapter1Equation::One => {
                format!("Eq. 1")
            }
        }
    }
}
