pub mod chapter_10;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter {
    Ten(chapter_10::Chapter10Method),
}

impl Chapter {
    pub fn friendly_reference(&self) -> String {
        match self {
            Chapter::Ten(method) => format!("Chapter 10, {}", method.friendly_reference()),
        }
    }
}
