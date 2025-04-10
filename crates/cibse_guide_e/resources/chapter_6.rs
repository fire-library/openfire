pub mod equation_6_7;
pub mod equation_6_appendix;

use openfire::serde::{Deserialize, Serialize};
use openfire::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter10Method {
    Equation6_7,
    Equation6_Appendix,
}

impl Chapter10Method {
    pub fn id(&self) -> String {
        match self {
            &Chapter6Method::Equation6_7 => "equation_6_7".to_string(),
        }
        match self {
            &Chapter6Method::Equation6_Appendix => "equation_6_appendix".to_string(),
        }
    }

    pub fn friendly_reference(&self) -> String {
        match self {
            &Chapter6Method::Equation6_7 => {
                format!("Ch. 6 | Eq. 6.7")
            }
            &Chapter6Method::Equation6_Appendix => {
                format!("Ch. 6 | Appendix")
            }
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            &Chapter6Method::Equation6_7 => {
                include_str!("../resources/chapter_6/equation_6_7/description.md").to_string()
            }
            &Chapter6Method::Equation6_Appendix => {
                include_str!("../resources/chapter_6/equation_6_appendix/description.md").to_string()
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            &Chapter6Method::Equation6_7 => {
                include_str!("../resources/chapter_6/equation_6_7/limitations.md").to_string()
            }
            &Chapter6Method::Equation6_Appendix => {
                include_str!("../resources/chapter_6/equation_6_appendix/limitations.md").to_string()
            }
        }
    }
}
