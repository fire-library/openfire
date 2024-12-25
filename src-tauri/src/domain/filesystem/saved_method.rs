use crate::domain::method::{
    parameter::{ParameterType, ParameterValue},
    Method, MethodType,
};
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
            .map(|(name, parameter)| {
                let p = parameter.read().unwrap();

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
