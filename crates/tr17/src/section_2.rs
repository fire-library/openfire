pub mod equation_1;

use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Section2Method {
    EquationOne,
}

impl Section2Method {
    pub fn id(&self) -> String {
        match self {
            &Section2Method::EquationOne => "equation_1".to_string(),
        }
    }

    pub fn friendly_reference(&self) -> String {
        match self {
            &Section2Method::EquationOne => {
                format!("Eq. 1")
            }
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            &Section2Method::EquationOne => {
                include_str!("../resources/section_2/equation_1/about.md").to_string()
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            &Section2Method::EquationOne => {
                include_str!("../resources/section_2/equation_1/limitations.md").to_string()
            }
        }
    }
}
