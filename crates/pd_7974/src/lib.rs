pub mod part_1;

use framework::method::runner::Reference;
use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum PD7974 {
    One(part_1::Section),
    Document,
}

impl PD7974 {
    pub fn title(&self) -> String {
        match self {
            PD7974::One(_) => {
                "Initiation and Development of Fire within the Enclosure of Origin (Sub-system 1)"
                    .to_string()
            }
            PD7974::Document => panic!("Document has no title"),
        }
    }
    pub fn number(&self) -> String {
        match self {
            PD7974::One(_) => "1".to_string(),
            PD7974::Document => panic!("Document has no number"),
        }
    }
}

impl Reference for PD7974 {
    fn id(&self) -> String {
        let top_level = self.document_id();
        match self {
            PD7974::One(section) => {
                format!("{}_{}", top_level, section.id())
            }
            PD7974::Document => top_level.to_string(),
        }
    }

    fn document_id(&self) -> String {
        let top_level = "pd_7974".to_string();
        match self {
            PD7974::One(_section) => {
                format!("{}_part_1", top_level)
            }
            PD7974::Document => top_level.to_string(),
        }
    }

    fn about_document(&self) -> String {
        include_str!("../resources/about.md").to_string()
    }

    fn document_name(&self) -> String {
        format!("PD 7974-{}", self.number())
    }

    fn harvard_reference(&self) -> String {
        format!(
            "BSI, 2019. PD 7974-{}: Application of Fire Safety Engineering Principles to the Design of Buildings - {}. BSI",
            self.number(),
            self.title()
        )
    }

    fn friendly_reference(&self) -> String {
        match self {
            PD7974::One(section) => match section {
                section => format!("PD 7974-1 | {}", section.friendly_reference()),
            },
            PD7974::Document => "PD 7974".to_string(),
        }
    }

    fn about_method(&self) -> String {
        match self {
            PD7974::One(section) => section.about_method(),
            PD7974::Document => panic!("Document has no method"),
        }
    }

    fn method_limitations(&self) -> String {
        match self {
            PD7974::One(section) => section.method_limitations(),
            PD7974::Document => panic!("Document has no method"),
        }
    }
}
