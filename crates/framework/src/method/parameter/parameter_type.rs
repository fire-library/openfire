use serde::{Deserialize, Serialize};
use specta::Type;

use super::{ArcParameter, Parameter, ParameterValue, object::Object};

#[derive(Type, Serialize, Deserialize, Debug)]
pub enum ParameterType {
    String(Parameter<String>),
    Float(Parameter<f64>),
    OutputFloat(Parameter<f64>),
    Bool(Parameter<bool>),
    Object(Parameter<Object>),
    StringEnum(Parameter<String>, Vec<String>),
    List(Parameter<Vec<ArcParameter>>),
    Constant(Parameter<ParameterValue>),
}
