use serde::{Deserialize, Serialize};
use specta::Type;
use std::fmt::Debug;

use super::parameter::ArcParameter;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct Dependency {
    pub parameter: ArcParameter,
    pub identifier: Option<String>,
}

impl From<ArcParameter> for Dependency {
    fn from(p: ArcParameter) -> Self {
        Dependency {
            parameter: p,
            identifier: None,
        }
    }
}
