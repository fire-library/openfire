pub mod equation_7_2;
pub mod equation_7_3;
pub mod equation_7_6;

use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter7Method {
    Equation7_2,
    Equation7_3,
    Equation7_6,
}

impl Chapter7Method {
    pub fn id(&self) -> String {
        match self {
            &Chapter7Method::Equation7_2 => "equation_7_2".to_string(),
            &Chapter7Method::Equation7_3 => "equation_7_3".to_string(),
            &Chapter7Method::Equation7_6 => "equation_7_6".to_string(),
        }
    }

    pub fn friendly_reference(&self) -> String {
        match self {
            &Chapter7Method::Equation7_2 => {
                format!("Ch. 7 | Eq. 7.2")
            }
            &Chapter7Method::Equation7_3 => {
                format!("Ch. 7 | Eq. 7.3")
            }
            &Chapter7Method::Equation7_6 => {
                format!("Ch. 7 | Eq. 7.6")
            }
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            &Chapter7Method::Equation7_2 => {
                include_str!("../resources/chapter_7/equation_7_2/description.md").to_string()
            }
            &Chapter7Method::Equation7_3 => {
                include_str!("../resources/chapter_7/equation_7_3/description.md").to_string()
            }
            &Chapter7Method::Equation7_6 => {
                include_str!("../resources/chapter_7/equation_7_6/description.md").to_string()
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            &Chapter7Method::Equation7_2 => {
                include_str!("../resources/chapter_7/equation_7_2/limitations.md").to_string()
            }
            &Chapter7Method::Equation7_3 => {
                include_str!("../resources/chapter_7/equation_7_3/limitations.md").to_string()
            }
            &Chapter7Method::Equation7_6 => {
                include_str!("../resources/chapter_7/equation_7_6/limitations.md").to_string()
            }
        }
    }
}
