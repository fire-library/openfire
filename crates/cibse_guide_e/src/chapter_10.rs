pub mod equation_10_1;

use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter10Method {
    Equation10_1,
}

impl Chapter10Method {
    pub fn id(&self) -> String {
        match self {
            &Chapter10Method::Equation10_1 => "equation_10_1".to_string(),
        }
    }

    pub fn friendly_reference(&self) -> String {
        match self {
            &Chapter10Method::Equation10_1 => {
                format!("Eq. 10.1")
            }
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            &Chapter10Method::Equation10_1 => {
                include_str!("../resources/chapter_10/equation_10_1/description.md").to_string()
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            &Chapter10Method::Equation10_1 => {
                include_str!("../resources/chapter_10/equation_10_1/limitations.md").to_string()
            }
        }
    }
}
