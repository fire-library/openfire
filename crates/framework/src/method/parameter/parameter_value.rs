use crate::method::parameter::ArcParameter;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ParameterValue {
    String(String),
    Float(f64),
    Bool(bool),
    Object(Vec<ArcParameter>),
    List(Vec<ArcParameter>),
}

impl ParameterValue {
    pub fn display_value(&self) -> String {
        match self {
            ParameterValue::String(value) => value.to_string(),
            ParameterValue::Float(value) => value.to_string(),
            ParameterValue::Bool(value) => value.to_string(),
            ParameterValue::Object(_values) => panic!("Object parameter value not implemented"),
            ParameterValue::List(_values) => panic!("List parameter value not implemented"),
        }
    }
}
