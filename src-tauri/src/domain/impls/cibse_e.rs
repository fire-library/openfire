pub mod chapter_10;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum CIBSEEChapter {
    Ten(chapter_10::Chapter10Method),
}

impl CIBSEEChapter {
    pub fn friendly_reference(&self) -> String {
        match self {
            CIBSEEChapter::Ten(method) => {
                format!("Ch. 10 | {}", method.friendly_reference())
            }
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            CIBSEEChapter::Ten(method) => method.about_method(),
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            CIBSEEChapter::Ten(method) => method.method_limitations(),
        }
    }
}
