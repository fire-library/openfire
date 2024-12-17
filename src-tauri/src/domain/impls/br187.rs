pub mod chapter_1;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter {
    One(chapter_1::Chapter1Equation),
}

impl Chapter {
    pub fn friendly_reference(&self) -> String {
        match self {
            Chapter::One(equation) => {
                format!("Chapter 1, {}", equation.friendly_reference())
            }
        }
    }
}
