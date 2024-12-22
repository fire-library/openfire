pub mod part_1;

use core::panic;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Part {
    One(Option<part_1::Section>),
}

impl Part {
    pub fn friendly_reference(&self) -> String {
        match self {
            Part::One(section) => match section {
                Some(section) => format!("1 | {}", section.friendly_reference()),
                None => "Part 1".to_string(),
            },
        }
    }
    pub fn title(&self) -> String {
        match self {
            Part::One(_) => {
                "Initiation and Development of Fire within the Enclosure of Origin (Sub-system 1)"
                    .to_string()
            }
        }
    }
    pub fn number(&self) -> String {
        match self {
            Part::One(_) => "1".to_string(),
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            Part::One(section) => match section {
                Some(section) => section.about_method(),
                None => panic!("No section found"),
            },
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            Part::One(section) => match section {
                Some(section) => section.method_limitations(),
                None => panic!("No section found"),
            },
        }
    }
}
