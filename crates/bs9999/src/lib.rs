pub mod chapter_15;

use framework::method::runner::Reference;
use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum BS9999 {
    ChapterFifteen(chapter_15::Chapter15Method),
    Document,
}

impl Reference for BS9999 {
    fn id(&self) -> String {
        let top_level = self.document_id();
        match self {
            BS9999::ChapterFifteen(method) => format!("{}_chapter_15_{}", top_level, method.id()),
            BS9999::Document => top_level.to_string(),
        }
    }
    fn document_id(&self) -> String {
        "bs_9999".to_string()
    }

    fn document_name(&self) -> String {
        "BS 9999".to_string()
    }

    fn about_document(&self) -> String {
        include_str!("../resources/about.md").to_string()
    }

    fn harvard_reference(&self) -> String {
        "British Standards Institution, 2017. BS 9999: Fire safety in the design, management and use of buildings - Code of practice. BSI.".to_string()
    }

    fn friendly_reference(&self) -> String {
        match self {
            BS9999::ChapterFifteen(method) => {
                format!("BS 9999 | Ch. 15 | {}", method.friendly_reference())
            }
            BS9999::Document => "BS 9999".to_string(),
        }
    }

    fn about_method(&self) -> String {
        match self {
            BS9999::ChapterFifteen(method) => method.about_method(),
            BS9999::Document => panic!("Document has no method"),
        }
    }

    fn method_limitations(&self) -> String {
        match self {
            BS9999::ChapterFifteen(method) => method.method_limitations(),
            BS9999::Document => panic!("Document has no method"),
        }
    }
}
