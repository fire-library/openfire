pub mod appendix_a;
pub mod chapter_1;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum BR187Chapter {
    One(chapter_1::Chapter1Equation),
    AppendixA,
}

impl BR187Chapter {
    pub fn friendly_reference(&self) -> String {
        match self {
            BR187Chapter::One(equation) => {
                format!("Ch. 1 | {}", equation.friendly_reference())
            }
            BR187Chapter::AppendixA => "Appendix A".to_string(),
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            BR187Chapter::One(method) => method.about_method(),
            BR187Chapter::AppendixA => {
                include_str!("../../../resources/br187/appendix_a/about.md").to_string()
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            BR187Chapter::One(method) => method.method_limitations(),
            BR187Chapter::AppendixA => {
                include_str!("../../../resources/br187/appendix_a/limitations.md").to_string()
            }
        }
    }
}
