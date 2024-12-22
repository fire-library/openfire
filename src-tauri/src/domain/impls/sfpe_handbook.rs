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
                format!("Ch. 14 | {}", method.friendly_reference())
            }
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            SFPEHandbookChapter::Fourteen(method) => method.about_method(),
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            SFPEHandbookChapter::Fourteen(method) => method.method_limitations(),
        }
    }
}
