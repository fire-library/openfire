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

    pub fn about_method(&self) -> String {
        match self {
            Chapter1Equation::One => {
                include_str!("../../../../resources/br187/chapter_1/equation_1/about.md")
                    .to_string()
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            Chapter1Equation::One => {
                include_str!("../../../../resources/br187/chapter_1/equation_1/limitations.md")
                    .to_string()
            }
        }
    }
}
