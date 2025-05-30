pub mod chapter_10;
pub mod chapter_6;
pub mod chapter_7;

use framework::method::runner::Reference;
use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum CIBSEGuideE {
    ChapterSix(chapter_6::Chapter6Method),
    ChapterSeven(chapter_7::Chapter7Method),
    ChapterTen(chapter_10::Chapter10Method),
    Document,
}

impl Reference for CIBSEGuideE {
    fn id(&self) -> String {
        let top_level = self.document_id();
        match self {
            CIBSEGuideE::ChapterSix(method) => format!("{}_chapter_6_{}", top_level, method.id()),
            CIBSEGuideE::ChapterSeven(method) => format!("{}_chapter_7_{}", top_level, method.id()),
            CIBSEGuideE::ChapterTen(method) => format!("{}_chapter_10_{}", top_level, method.id()),
            CIBSEGuideE::Document => top_level.to_string(),
        }
    }

    fn document_id(&self) -> String {
        "cibse_guide_e".to_string()
    }

    fn document_name(&self) -> String {
        "CIBSE Guide E".to_string()
    }

    fn friendly_reference(&self) -> String {
        match self {
            CIBSEGuideE::ChapterSix(method) => {
                format!("{} | {}", self.document_name(), method.friendly_reference())
            }
            CIBSEGuideE::ChapterSeven(method) => {
                format!("{} | {}", self.document_name(), method.friendly_reference())
            }
            CIBSEGuideE::ChapterTen(method) => {
                format!("{} | {}", self.document_name(), method.friendly_reference())
            }
            CIBSEGuideE::Document => panic!("Document has no method"),
        }
    }

    fn harvard_reference(&self) -> String {
        "CIBSE (Chartered Institution of Building Services Engineers). (2019). CIBSE Guide E: Fire Safety Engineering. 4th edn. London: CIBSE.".to_string()
    }

    fn about_document(&self) -> String {
        include_str!("../resources/description.md").to_string()
    }

    fn about_method(&self) -> String {
        match self {
            CIBSEGuideE::ChapterSix(method) => method.about_method(),
            CIBSEGuideE::ChapterSeven(method) => method.about_method(),
            CIBSEGuideE::ChapterTen(method) => method.about_method(),
            CIBSEGuideE::Document => panic!("Document has no method"),
        }
    }

    fn method_limitations(&self) -> String {
        match self {
            CIBSEGuideE::ChapterSix(method) => method.method_limitations(),
            CIBSEGuideE::ChapterSeven(method) => method.method_limitations(),
            CIBSEGuideE::ChapterTen(method) => method.method_limitations(),
            CIBSEGuideE::Document => panic!("Document has no method"),
        }
    }
}
