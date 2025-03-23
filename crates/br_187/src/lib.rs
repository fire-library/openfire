pub mod appendix_a;
pub mod chapter_1;

use framework::method::runner::Reference;
use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum BR187 {
    ChapterOne(chapter_1::Equation),
    AppendixA(appendix_a::Method),
    Document,
}

impl Reference for BR187 {
    fn id(&self) -> String {
        let top_level = self.document_id();
        match self {
            BR187::ChapterOne(equation) => format!("{}_chapter_1_{}", top_level, equation.id()),
            BR187::AppendixA(method) => format!("{}_appendix_a_{}", top_level, method.id()),
            BR187::Document => top_level.to_string(),
        }
    }
    fn document_name(&self) -> String {
        "BR 187".to_string()
    }

    fn document_id(&self) -> String {
        "br_187".to_string()
    }

    fn about_document(&self) -> String {
        include_str!("../resources/about.md").to_string()
    }

    fn harvard_reference(&self) -> String {
        "Chitty,R., 2014. External fire spread: Building Separation and Boundary Distances (BR 187). 2nd edition".to_string()
    }

    fn friendly_reference(&self) -> String {
        match self {
            BR187::ChapterOne(equation) => {
                format!("BR 187 | Ch. 1 | {}", equation.friendly_reference())
            }
            BR187::AppendixA(method) => {
                format!("BR 187 | Appendix A - {}", method.friendly_reference())
            }
            BR187::Document => "BR 187".to_string(),
        }
    }

    fn about_method(&self) -> String {
        match self {
            BR187::ChapterOne(method) => method.about_method(),
            BR187::AppendixA(method) => method.about_method(),
            BR187::Document => panic!("Document has no method"),
        }
    }

    fn method_limitations(&self) -> String {
        match self {
            BR187::ChapterOne(method) => method.method_limitations(),
            BR187::AppendixA(method) => method.method_limitations(),
            BR187::Document => panic!("Document has no method"),
        }
    }
}
