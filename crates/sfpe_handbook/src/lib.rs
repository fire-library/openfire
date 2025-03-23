pub mod chapter_14;

use framework::method::runner::Reference;
use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum SFPEHandbook {
    ChapterFourteen(chapter_14::Chapter14Method),
    Document,
}

impl Reference for SFPEHandbook {
    fn id(&self) -> String {
        let top_level = self.document_id();
        match self {
            SFPEHandbook::ChapterFourteen(method) => {
                format!("{}_chapter_14_{}", top_level, method.id())
            }
            SFPEHandbook::Document => top_level.to_string(),
        }
    }

    fn document_id(&self) -> String {
        "sfpe_handbook".to_string()
    }

    fn document_name(&self) -> String {
        "SFPE Handbook".to_string()
    }

    fn about_document(&self) -> String {
        include_str!("../resources/about.md").to_string()
    }

    fn harvard_reference(&self) -> String {
        "Hurley, M.J., et al., 2016. SFPE Handbook of Fire Protection Engineering. 5th ed. Springer.".to_string()
    }

    fn friendly_reference(&self) -> String {
        match self {
            SFPEHandbook::ChapterFourteen(method) => {
                format!("SFPE Handbook | Ch. 14 | {}", method.friendly_reference())
            }
            SFPEHandbook::Document => "SFPE Handbook".to_string(),
        }
    }

    fn about_method(&self) -> String {
        match self {
            SFPEHandbook::ChapterFourteen(method) => method.about_method(),
            SFPEHandbook::Document => panic!("Document has no method"),
        }
    }

    fn method_limitations(&self) -> String {
        match self {
            SFPEHandbook::ChapterFourteen(method) => method.method_limitations(),
            SFPEHandbook::Document => panic!("Document has no method"),
        }
    }
}
