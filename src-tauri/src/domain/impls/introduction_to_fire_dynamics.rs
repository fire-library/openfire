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
                format!("Ch. 10 | {}", method.friendly_reference())
            }
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            IntroductionToFireDynamicsChapter::Ten(method) => method.about_method(),
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            IntroductionToFireDynamicsChapter::Ten(method) => method.method_limitations(),
        }
    }
}
