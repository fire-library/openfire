pub mod equation_1;

use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Equation {
    One,
}

impl Equation {
    pub fn id(&self) -> String {
        match self {
            Equation::One => "equation_1".to_string(),
        }
    }

    pub fn friendly_reference(&self) -> String {
        match self {
            Equation::One => {
                format!("Eq. 1")
            }
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            Equation::One => include_str!("../resources/chapter_1/equation_1/about.md").to_string(),
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            Equation::One => {
                include_str!("../resources/chapter_1/equation_1/limitations.md").to_string()
            }
        }
    }
}
