use super::{ArcParameter, Parameters};

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Type, Serialize, Deserialize, Debug, Clone)]
pub struct Object(pub Parameters);

impl Object {
    pub fn values(&self) -> Vec<ArcParameter> {
        self.0.values()
    }
}
