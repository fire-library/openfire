pub mod section_2;

use framework::method::runner::Reference;
use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum TR17 {
    SectionTwo(section_2::Section2Method),
    Document,
}

impl Reference for TR17 {
    fn id(&self) -> String {
        let top_level = self.document_id();
        match self {
            TR17::SectionTwo(method) => {
                format!("{}_section_2_{}", top_level, method.id())
            }
            TR17::Document => top_level.to_string(),
        }
    }

    fn document_id(&self) -> String {
        "tr_17".to_string()
    }

    fn about_document(&self) -> String {
        match self {
            TR17::SectionTwo(_) => include_str!("../resources/about.md").to_string(),
            TR17::Document => include_str!("../resources/about.md").to_string(),
        }
    }
    fn document_name(&self) -> String {
        "TR 17".to_string()
    }
    fn harvard_reference(&self) -> String {
        "Wade, C.A., 2013. Room size limits when using a fire zone model for smoke-filling calculations. Technical Recommendation TR 17. BRANZ.".to_string()
    }
    fn friendly_reference(&self) -> String {
        match self {
            TR17::SectionTwo(equation) => {
                format!("TR 17 | Section 2 | {}", equation.friendly_reference())
            }
            TR17::Document => "TR 17".to_string(),
        }
    }

    fn about_method(&self) -> String {
        match self {
            TR17::SectionTwo(method) => method.about_method(),
            TR17::Document => panic!("Document has no method"),
        }
    }

    fn method_limitations(&self) -> String {
        match self {
            TR17::SectionTwo(method) => method.method_limitations(),
            TR17::Document => panic!("Document has no method"),
        }
    }
}
