pub mod equation_a1;
pub mod equation_a2;
pub mod equation_a3;
pub mod equation_a4;
pub mod equation_a5;
pub mod openfire_runner;

use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Method {
    ViewFactors,
}

impl Method {
    pub fn id(&self) -> String {
        match self {
            Method::ViewFactors => "view_factors".to_string(),
        }
    }

    pub fn friendly_reference(&self) -> String {
        match self {
            Method::ViewFactors => "View Factors".to_string(),
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            Method::ViewFactors => include_str!("../resources/appendix_a/about.md").to_string(),
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            Method::ViewFactors => {
                include_str!("../resources/appendix_a/limitations.md").to_string()
            }
        }
    }
}
