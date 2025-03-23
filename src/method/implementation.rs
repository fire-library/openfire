use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Icon {
    FireIcon,
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct Implementation {
    pub id: String,
    pub name: String,
    pub tags: Vec<String>,
    pub description: String,
    pub search_reference: String,
    pub icon: Icon,
    pub colors: String,
}
