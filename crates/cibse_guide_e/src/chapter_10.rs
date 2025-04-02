pub mod equation_10_1;
pub mod equation_10_2;
pub mod equation_10_3;
pub mod equation_10_4;
pub mod equation_10_7;
pub mod equation_10_8;
pub mod equation_10_10;

use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter10Method {
    Equation10_1,
    Equation10_2,
    Equation10_3,
    Equation10_4,
    Equation10_7,
    Equation10_8,
    Equation10_10,
}

impl Chapter10Method {
    pub fn id(&self) -> String {
        match self {
            &Chapter10Method::Equation10_1 => "equation_10_1".to_string(),
            &Chapter10Method::Equation10_2 => "equation_10_2".to_string(),
            &Chapter10Method::Equation10_3 => "equation_10_3".to_string(),
            &Chapter10Method::Equation10_4 => "equation_10_4".to_string(),
            &Chapter10Method::Equation10_7 => "equation_10_7".to_string(),
            &Chapter10Method::Equation10_8 => "equation_10_8".to_string(),
            &Chapter10Method::Equation10_10 => "equation_10_10".to_string(),
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
            &Chapter10Method::Equation10_3 => {
                format!("Ch. 10 | Eq. 10.3")
            }
            &Chapter10Method::Equation10_4 => {
                format!("Ch. 10 | Eq. 10.4")
            }
            &Chapter10Method::Equation10_7 => {
                format!("Ch. 10 | Eq. 10.7")
            }
            &Chapter10Method::Equation10_8 => {
                format!("Ch. 10 | Eq. 10.8")
            }
            &Chapter10Method::Equation10_10 => {
                format!("Ch. 10 | Eq. 10.10")
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
            &Chapter10Method::Equation10_3 => {
                include_str!("../resources/chapter_10/equation_10_3/description.md").to_string()
            }
            &Chapter10Method::Equation10_4 => {
                include_str!("../resources/chapter_10/equation_10_4/description.md").to_string()
            }
            &Chapter10Method::Equation10_7 => {
                include_str!("../resources/chapter_10/equation_10_7/description.md").to_string()
            }
            &Chapter10Method::Equation10_8 => {
                include_str!("../resources/chapter_10/equation_10_8/description.md").to_string()
            }
            &Chapter10Method::Equation10_10 => {
                include_str!("../resources/chapter_10/equation_10_10/description.md").to_string()
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
            &Chapter10Method::Equation10_3 => {
                include_str!("../resources/chapter_10/equation_10_3/limitations.md").to_string()
            }
            &Chapter10Method::Equation10_4 => {
                include_str!("../resources/chapter_10/equation_10_4/limitations.md").to_string()
            }
            &Chapter10Method::Equation10_7 => {
                include_str!("../resources/chapter_10/equation_10_7/limitations.md").to_string()
            }
            &Chapter10Method::Equation10_8 => {
                include_str!("../resources/chapter_10/equation_10_8/limitations.md").to_string()
            }
            &Chapter10Method::Equation10_10 => {
                include_str!("../resources/chapter_10/equation_10_10/limitations.md").to_string()
            }
        }
    }
}
