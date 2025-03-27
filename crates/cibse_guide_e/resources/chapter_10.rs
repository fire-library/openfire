pub mod equation_10_1;
pub mod equation_10_2;

use openfire::serde::{Deserialize, Serialize};
use openfire::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter10Method {
    Equation10_1,
    Equation10_2,
}

impl Chapter10Method {
    pub fn id(&self) -> String {
        match self {
            &Chapter10Method::Equation10_1 => "equation_10_1".to_string(),
            &Chapter10Method::Equation10_2 => "equation_10_2".to_string(),
        }
    }

    pub fn friendly_reference(&self) -> String {
        match self {
            &Chapter10Method::Equation10_1 => {
                format!("Ch. 10 | Eq. 10.1")
            }
            &Chapter10Method::Equation10_2 => {
                format!("Ch. 10 | Eq. 10.2")
            }
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            &Chapter10Method::Equation10_1 => {
                include_str!("../resources/chapter_10/equation_10_1/description.md").to_string()
            }
            &Chapter10Method::Equation10_2 => {
                include_str!("../resources/chapter_10/equation_10_2/description.md").to_string()
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            &Chapter10Method::Equation10_1 => {
                include_str!("../resources/chapter_10/equation_10_1/limitations.md").to_string()
            }
            &Chapter10Method::Equation10_2 => {
                include_str!("../resources/chapter_10/equation_10_2/limitations.md").to_string()
            }
        }
    }
}
