pub mod chapter_14;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter {
    Fourteen(chapter_14::Chapter14Method),
}

impl Chapter {
    pub fn friendly_reference(&self) -> String {
        match self {
            Chapter::Fourteen(method) => {
                format!("Chapter 14, {}", method.friendly_reference())
            }
        }
    }
}
