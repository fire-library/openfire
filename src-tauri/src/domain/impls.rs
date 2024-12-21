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
            &Document::BR187(_) => "BR 187".to_string(),
            &Document::PD7974(part) => match part {
                Some(part) => format!("PD 7974-{}", part.number()),
                None => "PD 7974".to_string(),
            },
            &Document::SFPEHandbook(_) => "SFPE Handbook".to_string(),
            &Document::IntroductionToFireDynamics(_) => "Introduction to Fire Dynamics".to_string(),
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
        }
    }
    pub fn document_about_information(&self) -> String {
        match &self{
            &Document::BR187(_) => "BR 187: External Fire Spread – Building Separation and Boundary Distances is a guidance document developed by the Building Research Establishment (BRE). It provides technical recommendations for assessing and mitigating the risk of fire spreading between buildings via external walls. The document is widely used in the UK to ensure compliance with fire safety regulations. BR 187 outlines methods to calculate boundary distances, evaluate external fire resistance, and design buildings with adequate separation to minimize fire hazards. It is an essential resource for fire engineers, architects, and regulatory authorities.".to_string(),
            &Document::PD7974(_) => "The PD 7974 suite, published by the British Standards Institution (BSI), provides a framework for fire safety engineering design. It consists of seven complementary documents offering guidance on various aspects of fire safety analysis, including fire dynamics, smoke movement, structural response, evacuation, and fire service intervention. Developed to support BS 9999 and BS 7974, the suite uses performance-based principles to enable tailored fire safety solutions for complex or innovative building designs. Each document includes methodologies, design scenarios, and calculation approaches, making the suite an indispensable resource for fire engineers seeking compliance with UK and international fire safety regulations.".to_string(),
            &Document::SFPEHandbook(_)  => "The SFPE Handbook of Fire Protection Engineering is the definitive reference for fire safety professionals worldwide, published by the Society of Fire Protection Engineers (SFPE). This comprehensive resource provides advanced scientific and engineering principles to analyze fire dynamics, human behavior, structural performance, smoke control, and fire suppression systems. Featuring contributions from leading experts, the handbook supports performance-based design and is widely used in academia, research, and professional practice. It serves as a critical tool for fire engineers, architects, and regulators to develop innovative, evidence-based fire safety strategies that meet national and international standards.".to_string(),
            &Document::IntroductionToFireDynamics(_) => "Introduction to Fire Dynamics by Dougal Drysdale is a foundational textbook in fire safety engineering, widely regarded as the standard reference for understanding the principles of fire behavior. The book provides a detailed exploration of fire science, covering topics such as ignition, flame spread, heat transfer, and combustion products. It bridges theory and practical application, offering insights into fire dynamics modeling and real-world fire scenarios. Essential for students, researchers, and professionals, Drysdale’s work equips readers with the knowledge needed to analyze and predict fire behavior, making it indispensable for advancing fire safety engineering practice globally.".to_string(),
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
