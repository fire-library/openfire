pub mod chapter_10;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum IntroductionToFireDynamicsChapter {
    Ten(chapter_10::Chapter10Method),
}

impl IntroductionToFireDynamicsChapter {
    pub fn friendly_reference(&self) -> String {
        match self {
            IntroductionToFireDynamicsChapter::Ten(method) => {
                format!("Chapter 10, {}", method.friendly_reference())
            }
        }
    }
}
