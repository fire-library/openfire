use crate::domain::method::{parameter::ParameterValue, Method, MethodType};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct SavedMethod {
    pub method_type: MethodType,
    pub parameters: Vec<SavedParameter>,
}

impl From<Method> for SavedMethod {
    fn from(method: Method) -> Self {
        let parameters = method
            .parameters
            .iter()
            .map(|(name, parameter)| SavedParameter {
                name: name.clone(),
                value: parameter.read().unwrap().value.clone(),
            })
            .collect();

        SavedMethod {
            method_type: method.method_type,
            parameters: parameters,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct SavedParameter {
    pub name: String,
    pub value: Option<ParameterValue>,
}
