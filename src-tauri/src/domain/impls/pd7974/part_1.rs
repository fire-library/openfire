pub mod section_8;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Section {
    Eight(section_8::Section8Method),
}

impl Section {
    pub fn friendly_reference(&self) -> String {
        match self {
            Section::Eight(method) => method.friendly_reference(),
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            Section::Eight(method) => method.about_method(),
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            Section::Eight(method) => method.method_limitations(),
        }
    }
}
