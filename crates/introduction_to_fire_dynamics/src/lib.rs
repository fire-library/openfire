pub mod chapter_10;

use framework::method::runner::Reference;
use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum IntroductionToFireDynamics {
    ChapterTen(chapter_10::Chapter10Method),
    Document,
}

impl Reference for IntroductionToFireDynamics {
    fn id(&self) -> String {
        let top_level = self.document_id();
        match self {
            IntroductionToFireDynamics::ChapterTen(method) => {
                format!("{}_chapter_10_{}", top_level, method.id())
            }
            IntroductionToFireDynamics::Document => top_level.to_string(),
        }
    }

    fn document_id(&self) -> String {
        "introduction_to_fire_dynamics".to_string()
    }

    fn document_name(&self) -> String {
        "Introduction to Fire Dynamics".to_string()
    }

    fn about_document(&self) -> String {
        include_str!("../resources/about.md").to_string()
    }

    fn harvard_reference(&self) -> String {
        "Drysdale, D., 2011. Introduction to fire dynamics. 3rd ed. Wiley.".to_string()
    }

    fn friendly_reference(&self) -> String {
        match self {
            IntroductionToFireDynamics::ChapterTen(method) => {
                format!(
                    "Introduction to Fire Dynamics | Ch. 10 | {}",
                    method.friendly_reference()
                )
            }
            IntroductionToFireDynamics::Document => "Introduction to Fire Dynamics".to_string(),
        }
    }

    fn about_method(&self) -> String {
        match self {
            IntroductionToFireDynamics::ChapterTen(method) => method.about_method(),
            IntroductionToFireDynamics::Document => panic!("Document has no method"),
        }
    }

    fn method_limitations(&self) -> String {
        match self {
            IntroductionToFireDynamics::ChapterTen(method) => method.method_limitations(),
            IntroductionToFireDynamics::Document => panic!("Document has no method"),
        }
    }
}
