use crate::method::{
    Method,
    parameter::{ArcParameter, ParameterType, ParameterValue},
};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct SavedMethod {
    pub id: String,
    pub parameters: Vec<SavedParameter>,
}

impl From<Method> for SavedMethod {
    fn from(method: Method) -> Self {
        let parameters = method
            .parameters
            .values()
            .iter()
            .map(|parameter| parameter.clone().into())
            .collect();

        let method = SavedMethod {
            id: method.id,
            parameters: parameters,
        };

        method
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct SavedParameter {
    pub name: String,
    pub value: Option<ParameterValue>,
}

impl From<ArcParameter> for SavedParameter {
    fn from(parameter: ArcParameter) -> Self {
        let p = parameter.read().unwrap();
        let name = p.symbol();

        match &*p {
            ParameterType::Float(float) => SavedParameter {
                name: name.clone(),
                value: float.value.map(|v| ParameterValue::Float(v)),
            },
            ParameterType::String(string) => SavedParameter {
                name: name.clone(),
                value: string.value.clone().map(|v| ParameterValue::String(v)),
            },
            _ => panic!("Unsupported parameter type"),
        }
    }
}
