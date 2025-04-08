pub mod equation_6_7;

use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter10Method {
    Equation6_7,
}

impl Chapter10Method {
    pub fn id(&self) -> String {
        match self {
            &Chapter10Method::Equation10_1 => "equation_6_7".to_string(),
        }
    }

    pub fn friendly_reference(&self) -> String {
        match self {
            &Chapter10Method::Equation10_1 => {
                format!("Ch. 6 | Eq. 6.7")
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
            &Chapter10Method::Equation10_11 => {
                include_str!("../resources/chapter_10/equation_10_11/description.md").to_string()
            }
            &Chapter10Method::Equation10_12 => {
                include_str!("../resources/chapter_10/equation_10_12/description.md").to_string()
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
            &Chapter10Method::Equation10_11 => {
                include_str!("../resources/chapter_10/equation_10_11/limitations.md").to_string()
            }
            &Chapter10Method::Equation10_12 => {
                include_str!("../resources/chapter_10/equation_10_12/limitations.md").to_string()
            }
        }
    }
}
