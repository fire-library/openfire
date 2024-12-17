pub mod chapter_14;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum SFPEHandbookChapter {
    Fourteen(chapter_14::Chapter14Method),
}

impl SFPEHandbookChapter {
    pub fn friendly_reference(&self) -> String {
        match self {
            SFPEHandbookChapter::Fourteen(method) => {
                format!("Chapter 14, {}", method.friendly_reference())
            }
        }
    }
}
