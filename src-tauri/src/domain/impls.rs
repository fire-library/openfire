pub mod br187;
pub mod introduction_to_fire_dynamics;
pub mod pd7974;
pub mod sfpe_handbook;
pub mod tag;
use pd7974::Part;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::domain::method::builder::MethodBuilderTrait;
use crate::domain::method::MethodType;

use super::method::Reference;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Icon {
    FireIcon,
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Document {
    BR187(Option<br187::BR187Chapter>),
    PD7974(Option<pd7974::Part>),
    SFPEHandbook(Option<sfpe_handbook::SFPEHandbookChapter>),
    IntroductionToFireDynamics(
        Option<introduction_to_fire_dynamics::IntroductionToFireDynamicsChapter>,
    ),
}

impl Document {
    pub fn name(&self) -> String {
        match &self {
            &Document::BR187(_) => "BR187".to_string(),
            &Document::PD7974(part) => match part {
                Some(part) => format!("PD7974: {}", part.friendly_reference()),
                None => "PD7974".to_string(),
            },
            &Document::SFPEHandbook(_) => "SFPE Handbook".to_string(),
            &Document::IntroductionToFireDynamics(_) => "Introduction to Fire Dynamics".to_string(),
        }
    }

    pub fn friendly_reference(&self) -> String {
        match &self {
            &Document::BR187(chapter) => match chapter {
                Some(chapter) => format!("BR187, {}", chapter.friendly_reference()),
                None => "BR187".to_string(),
            },
            &Document::PD7974(part) => match part {
                Some(part) => format!("PD7974, {}", part.friendly_reference()),
                None => "PD7974".to_string(),
            },
            &Document::SFPEHandbook(c) => match c {
                Some(c) => format!("SFPE Handbook, {}", c.friendly_reference()),
                None => "SFPE Handbook".to_string(),
            },
            &Document::IntroductionToFireDynamics(c) => match c {
                Some(c) => format!("Introduction to Fire Dynamics, {}", c.friendly_reference()),
                None => "Introduction to Fire Dynamics".to_string(),
            },
        }
    }

    pub fn harvard_reference(&self) -> String {
        match &self {
            &Document::BR187(_) => "Chitty,R., 2014. External fire spread: Building Separation and Boundary Distances (BR 187). 2nd edition".to_string(),
            &Document::PD7974(part) => match part {
                Some(part) => format!("PD7974, {}", part.friendly_reference()),
                None => "PD7974".to_string(),
            },
            &Document::SFPEHandbook(c) => match c {
                Some(c) => format!("Morgan Hurley (2015).SFPE Handbook of Fire Protection Engineering. 5th edition. {}", c.friendly_reference()),
                None => "SFPE Handbook".to_string(),
            },
            &Document::IntroductionToFireDynamics(c) => match c {
                Some(c) => format!("Drysdale, D., 2011. Introduction to fire dynamics. 3rd ed. Wiley. {}", c.friendly_reference()),
                None => "Introduction to Fire Dynamics".to_string(),
            },
        }
    }
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct Implementation {
    pub name: String,
    pub tags: Vec<String>,
    pub description: String,
    pub reference: Reference,
    pub search_reference: String,
    pub method_type: MethodType,
    pub icon: Icon,
    pub colors: String,
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct DocumentImplementations {
    pub document: String,
    pub implementations: Vec<Implementation>,
}

pub fn all_impls() -> Vec<DocumentImplementations> {
    vec![
        DocumentImplementations {
            document: Document::BR187(None).name(),
            implementations: vec![
                br187::chapter_1::equation_1::BR187Chapter1Equation1Builder::index_page(),
            ],
        },
        DocumentImplementations {
            document: Document::PD7974(Some(Part::One(None))).name(),
            implementations: vec![
        pd7974::part_1::section_8::maximum_enclosure_temperature::MaximumEnclosureTemperatureBuilder::index_page(),
        pd7974::part_1::section_8::hrr_at_flashover::HRRAtFlashoverBuilder::index_page(),
            ],
        },
        DocumentImplementations {
            document: Document::SFPEHandbook(None).name(),
            implementations: vec![
        sfpe_handbook::chapter_14::alpert::heat_release_from_temp_and_position::AlpertHeatReleaseFromTempAndPositionBuilder::index_page(),
            ],
        },
        DocumentImplementations {
            document: Document::IntroductionToFireDynamics(None).name(),
            implementations: vec![
        introduction_to_fire_dynamics::chapter_10::burning_regime::BurningRegimeBuilder::index_page()
            ],
        },
    ]
}
