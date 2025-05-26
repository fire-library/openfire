pub mod equation_6_7;
pub mod equation_6_57;
pub mod equation_6_58;

use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter6Method {
    Equation6_7,
    Equation6_57,
    Equation6_58,
}

impl Chapter6Method {
    pub fn id(&self) -> String {
        match self {
            Chapter6Method::Equation6_7 => "equation_6_7".to_string(),
            Chapter6Method::Equation6_57 => "equation_6_57".to_string(),
            Chapter6Method::Equation6_58 => "equation_6_58".to_string(),
        }
    }

    pub fn friendly_reference(&self) -> String {
        match self {
            Chapter6Method::Equation6_7 => format!("Ch. 6 | Eq. 6.7"),
            Chapter6Method::Equation6_57 => format!("Ch. 6 | Eq. 6.57"),
            Chapter6Method::Equation6_58 => format!("Ch. 6 | Eq. 6.58"),
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            Chapter6Method::Equation6_7 => {
                include_str!("../resources/chapter_6/equation_6_7/description.md").to_string()
            }
            Chapter6Method::Equation6_57 => {
                include_str!("../resources/chapter_6/equation_6_57/description.md").to_string()
            }
            Chapter6Method::Equation6_58 => {
                include_str!("../resources/chapter_6/equation_6_58/description.md").to_string()
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            Chapter6Method::Equation6_7 => {
                include_str!("../resources/chapter_6/equation_6_7/limitations.md").to_string()
            }
            Chapter6Method::Equation6_57 => {
                include_str!("../resources/chapter_6/equation_6_57/limitations.md").to_string()
            }
            Chapter6Method::Equation6_58 => {
                include_str!("../resources/chapter_6/equation_6_58/limitations.md").to_string()
            }
        }
    }
}
