pub mod chapter_1;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum BR187Chapter {
    One(chapter_1::Chapter1Equation),
}

impl BR187Chapter {
    pub fn friendly_reference(&self) -> String {
        match self {
            BR187Chapter::One(equation) => {
                format!("Ch. 1 | {}", equation.friendly_reference())
            }
        }
    }
}
