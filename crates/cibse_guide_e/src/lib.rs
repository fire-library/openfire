pub mod chapter_10;

use framework::method::runner::Reference;
use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum CIBSEGuideE {
    ChapterTen(chapter_10::Chapter10Method),
    Document,
}

impl Reference for CIBSEGuideE {
    fn id(&self) -> String {
        let top_level = self.document_id();
        match self {
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
        "CIBSE Guide E | Ch 10 | Eq 10.1".to_string()
    }

    fn harvard_reference(&self) -> String {
        "CIBSE (Chartered Institution of Building Services Engineers). (2019). CIBSE Guide E: Fire Safety Engineering. 4th edn. London: CIBSE.".to_string()
    }

    fn about_document(&self) -> String {
        include_str!("../resources/description.md").to_string()
    }

    fn about_method(&self) -> String {
        match self {
            CIBSEGuideE::ChapterTen(method) => method.about_method(),
            CIBSEGuideE::Document => panic!("Document has no method"),
        }
    }

    fn method_limitations(&self) -> String {
        match self {
            CIBSEGuideE::ChapterTen(method) => method.method_limitations(),
            CIBSEGuideE::Document => panic!("Document has no method"),
        }
    }
}
