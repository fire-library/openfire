pub mod section_8;

use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Section {
    Eight(section_8::Section8Method),
    Document,
}

impl Section {
    pub fn id(&self) -> String {
        match self {
            Section::Eight(method) => format!("_section_8_{}", method.id()),
            Section::Document => "".to_string(),
        }
    }

    pub fn friendly_reference(&self) -> String {
        match self {
            Section::Eight(method) => method.friendly_reference(),
            Section::Document => panic!("Document has no method"),
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            Section::Eight(method) => method.about_method(),
            Section::Document => panic!("Document has no method"),
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            Section::Eight(method) => method.method_limitations(),
            Section::Document => panic!("Document has no method"),
        }
    }
}
