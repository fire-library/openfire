pub mod br187;
pub mod introduction_to_fire_dynamics;
pub mod pd7974;
pub mod sfpe_handbook;
pub mod cibse_e
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
    CIBSEE(Option<cibse_e::CIBSEEChapter)
}

impl Document {
    pub fn name(&self) -> String {
        match &self {
            &Document::BR187(_) => "BR 187".to_string(),
            &Document::PD7974(part) => match part {
                Some(part) => format!("PD 7974-{}", part.number()),
                None => "PD 7974".to_string(),
            },
            &Document::SFPEHandbook(_) => "SFPE Handbook".to_string(),
            &Document::IntroductionToFireDynamics(_) => "Introduction to Fire Dynamics".to_string(),
            &Document::CIBSEE(_) => "CIBSE Guide E".to_string(),
        }
    }

    pub fn friendly_reference(&self) -> String {
        match &self {
            &Document::BR187(chapter) => match chapter {
                Some(chapter) => format!("BR 187 | {}", chapter.friendly_reference()),
                None => "BR 187".to_string(),
            },
            &Document::PD7974(part) => match part {
                Some(part) => format!("PD 7974-{}", part.friendly_reference()),
                None => "PD 7974".to_string(),
            },
            &Document::SFPEHandbook(c) => match c {
                Some(c) => format!("SFPE Handbook | {}", c.friendly_reference()),
                None => "SFPE Handbook".to_string(),
            },
            &Document::IntroductionToFireDynamics(c) => match c {
                Some(c) => format!("Introduction to Fire Dynamics | {}", c.friendly_reference()),
                None => "Introduction to Fire Dynamics".to_string(),
            },
            &Document::CIBSEE(c:) => match c{
                Some(c:) => format!("CIBSE Guide E | {}", c.friendly_reference()),
                None => "CIBSE Guide E".to_string(),
            }
        }
    }

    pub fn harvard_reference(&self) -> String {
        match &self {
            &Document::BR187(_) => "Chitty,R., 2014. External fire spread: Building Separation and Boundary Distances (BR 187). 2nd edition".to_string(),
            &Document::PD7974(optional_part) => match optional_part{
                None => panic!("Part required to create reference"),
                Some(part) => format!("BSI, 2019. PD 7974-{}: Application of Fire Safety Engineering Principles to the Design of Buildings – {}. BSI",
                part.number(), part.title()),
            } 
            &Document::SFPEHandbook(_) => "Hurley, M.J., et al., 2016. SFPE Handbook of Fire Protection Engineering. 5th ed. Springer.".to_string(),
            &Document::IntroductionToFireDynamics(_) => "Drysdale, D., 2011. Introduction to fire dynamics. 3rd ed. Wiley.".to_string(),
            &Document::CIBSEE(_) => "Chartered Institution of Building Services Engineers, 2019. Guide E: Fire Safety Engineering. 4th ed. London.".to_string(),
        }
    }

    pub fn about_document(&self) -> String {
        match &self {
            &Document::BR187(_) => include_str!("../../resources/br187/about.md").to_string(),
            &Document::PD7974(_) => include_str!("../../resources/pd7974/about.md").to_string(),
            &Document::SFPEHandbook(_) => include_str!("../../resources/sfpe_handbook/about.md").to_string(),
            &Document::IntroductionToFireDynamics(_) => include_str!("../../resources/introduction_to_fire_dynamics/about.md").to_string()
        }
    }

    pub fn about_method(&self) -> String {
        match &self {
            &Document::BR187(chapter) => match chapter {
                Some(chapter) => chapter.about_method(),
                None => panic!("No method provided"),
            },
            &Document::PD7974(part) => match part {
                Some(part) => part.about_method(),
                None => panic!("No part provided"),
            },
            &Document::SFPEHandbook(chapter) => match chapter {
                Some(chapter) => chapter.about_method(),
                None => panic!("No chapter provided"),
            },
            &Document::IntroductionToFireDynamics(chapter) => match chapter {
                Some(chapter) => chapter.about_method(),
                None => panic!("No chapter provided"),
            },
            &Document::CIBSEE(chapter: ) => match chapter {
                Some(chapter) => chapter.about_method(),
                None => panic!("No chapter provided"),
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match &self {
            &Document::BR187(chapter) => match chapter {
                Some(chapter) => chapter.method_limitations(),
                None => panic!("No method provided"),
            },
            &Document::PD7974(part) => match part {
                Some(part) => part.method_limitations(),
                None => panic!("No part provided"),
            },
            &Document::SFPEHandbook(chapter) => match chapter {
                Some(chapter) => chapter.method_limitations(),
                None => panic!("No chapter provided"),
            },
            &Document::IntroductionToFireDynamics(chapter) => match chapter {
                Some(chapter) => chapter.method_limitations(),
                None => panic!("No chapter provided"),
            },
            &Document::CIBSEE(chapter:) => match chapter {
                Some(chapter) => chapter.method_limitations(),
                None => panic!("No chapter provided"),
            }
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
        DocumentImplementations{
            document: Document::CIBSEE(None).name(),
            implementations: vec![
                cibse_e::chapter_10::ExtractPointsBuilder::index_page()
            ],
        },
    ],
}
