pub mod figure_6a;
pub mod figure_6b;
pub mod figure_6c;

use framework::serde::{Deserialize, Serialize};
use framework::specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Chapter15Method {
    Figure6a,
    Figure6b,
    Figure6c,
}

impl Chapter15Method {
    pub fn id(&self) -> String {
        match self {
            &Chapter15Method::Figure6a => "figure_6a".to_string(),
            &Chapter15Method::Figure6b => "figure_6b".to_string(),
            &Chapter15Method::Figure6c => "figure_6c".to_string(),
        }
    }

    pub fn friendly_reference(&self) -> String {
        match self {
            &Chapter15Method::Figure6a => {
                format!("Fig. 6a")
            }
            &Chapter15Method::Figure6b => {
                format!("Fig. 6b")
            }
            &Chapter15Method::Figure6c => {
                format!("Fig. 6c")
            }
        }
    }

    pub fn about_method(&self) -> String {
        match self {
            &Chapter15Method::Figure6a => {
                include_str!("../resources/chapter_15/about.md").to_string()
            }
            &Chapter15Method::Figure6b => {
                include_str!("../resources/chapter_15/about.md").to_string()
            }
            &Chapter15Method::Figure6c => {
                include_str!("../resources/chapter_15/about.md").to_string()
            }
        }
    }

    pub fn method_limitations(&self) -> String {
        match self {
            &Chapter15Method::Figure6a => {
                include_str!("../resources/chapter_15/limitations.md").to_string()
            }
            &Chapter15Method::Figure6b => {
                include_str!("../resources/chapter_15/limitations.md").to_string()
            }
            &Chapter15Method::Figure6c => {
                include_str!("../resources/chapter_15/limitations.md").to_string()
            }
        }
    }
}
