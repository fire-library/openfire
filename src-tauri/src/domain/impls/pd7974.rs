pub mod part_1;

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
    pub fn part_description(&self) -> String {
        match self {
            Part::One(_) => "PD 7974-1: Application of Fire Safety Engineering Principles to the Design of Buildings – Initiation and Development of Fire within the Enclosure of Origin is the first part of the PD 7974 suite, published by the British Standards Institution (BSI). It provides methodologies for analyzing fire initiation, growth, and development within a compartment. The document includes guidance on heat release rates, fire load densities, ventilation effects, and other critical parameters influencing fire behavior. Used by fire engineers, it supports performance-based design by offering tools to assess fire scenarios and ensure compliance with safety objectives in building design and risk assessments.".to_string(),
        }
    }
}
