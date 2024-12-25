pub mod builder;
use super::equation::Equation;
use super::form::FieldType;
use super::validation::ParameterError;
use crate::domain::method::form::Field;
use core::panic;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ParameterValue {
    String(String),
    Float(f64),
    Bool(bool),
    Object(Vec<ArcParameter>),
    List(Vec<ArcParameter>),
}

impl PartialEq for ParameterValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParameterValue::String(a), ParameterValue::String(b)) => a == b,
            (ParameterValue::Float(a), ParameterValue::Float(b)) => a == b,
            (ParameterValue::Bool(a), ParameterValue::Bool(b)) => a == b,
            _ => false,
        }
    }
}

impl ParameterValue {
    pub fn to_string(&self) -> Result<String, String> {
        match self {
            ParameterValue::String(value) => Ok(value.clone()),
            ParameterValue::Float(value) => Ok(value.to_string()),
            ParameterValue::Bool(value) => Ok(value.to_string()),
            ParameterValue::Object(_) => Err("Cannot Convert ParamList to String".to_string()),
            ParameterValue::List(_) => Err("Cannot Convert List to String".to_string()),
        }
    }

    pub fn to_float(&self) -> Result<f64, String> {
        match self {
            ParameterValue::Float(value) => Ok(value.clone()),
            _ => Err("Value is not a float".to_string()),
        }
    }
}

impl fmt::Display for ParameterValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParameterValue::String(value) => write!(f, "{}", value),
            ParameterValue::Float(value) => write!(f, "{}", value),
            ParameterValue::Bool(value) => {
                if *value {
                    write!(f, "True")
                } else {
                    write!(f, "False")
                }
            }
            ParameterValue::Object(_) => write!(f, "Object"),
            ParameterValue::List(_) => write!(f, "List"),
        }
    }
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Validation {
    Required,
    MinLength(u32),
    MaxLength(u32),
    Range(f64, f64),
    MinExclusive(f64),
    Min(f64),
    Max(f64),
    Relation(Comparison),
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Comparison {
    GreaterThanOrEqual(ArcParameter),
    LessThanOrEqual(ArcParameter),
}

#[derive(Type, Serialize, Deserialize, Debug)]
pub enum DisplayOptions {
    DecimalPlaces(u32),
}

pub trait DisplayOptionsTrait {
    fn decimal_places(&self) -> Option<u32>;
}

impl DisplayOptionsTrait for Vec<DisplayOptions> {
    fn decimal_places(&self) -> Option<u32> {
        for option in self {
            if let DisplayOptions::DecimalPlaces(places) = option {
                return Some(*places);
            }
        }
        None
    }
}

#[derive(Type, Serialize, Deserialize, Debug)]
pub enum ParameterType {
    String(Parameter<String>),
    Float(Parameter<f64>),
    Bool(Parameter<bool>),
    Object(Parameters),
}

#[derive(Type, Serialize, Deserialize, Debug)]
pub struct Parameter<T: PartialEq> {
    pub id: String,
    pub name: String,
    #[specta(skip)]
    #[serde(skip)]
    pub expression: Option<Box<dyn Equation>>,
    pub value: Option<T>,
    pub display_options: Vec<DisplayOptions>,
    pub units: Option<String>,
    pub validations: Vec<Validation>,
}

impl<T: PartialEq> PartialEq for Parameter<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Parameter<String> {
    pub fn validate(&self) -> Result<(), ParameterError> {
        for validation in &self.validations {
            match validation {
                Validation::Required => match &self.value {
                    Some(value) => {
                        if value.trim().is_empty() {
                            return Err(ParameterError::new(&self.id, "required".to_string()));
                        }
                    }
                    None => {
                        return Err(ParameterError::new(&self.id, "required".to_string()));
                    }
                },
                Validation::MinLength(min) => match &self.value {
                    Some(value) => {
                        if (value.len() as u32) < *min {
                            return Err(ParameterError::new(
                                &self.id,
                                format!("must be at least {} characters", min),
                            ));
                        }
                    }
                    None => {}
                },
                Validation::MaxLength(max) => match &self.value {
                    Some(value) => {
                        if (value.len() as u32) > *max {
                            return Err(ParameterError::new(
                                &self.id,
                                format!("must be at most {} characters", max),
                            ));
                        }
                    }
                    None => {}
                },
                _ => {}
            }
        }
        Ok(())
    }
}

impl Parameter<f64> {
    pub fn validate(&self) -> Result<(), ParameterError> {
        for validation in &self.validations {
            match validation {
                Validation::Required => match &self.value {
                    Some(_value) => {}
                    None => {
                        return Err(ParameterError::new(&self.id, "required".to_string()));
                    }
                },
                Validation::Range(min, max) => match &self.value {
                    Some(value) => {
                        if value < min || value > max {
                            return Err(ParameterError::new(
                                &self.id,
                                format!("must be between {} and {}", min, max),
                            ));
                        }
                    }
                    None => {}
                },
                Validation::Min(min) => match &self.value {
                    Some(value) => {
                        if value < min {
                            return Err(ParameterError::new(&self.id, format!("min: {}", min)));
                        }
                    }
                    None => {}
                },
                Validation::Max(max) => match &self.value {
                    Some(value) => {
                        if value > max {
                            return Err(ParameterError::new(&self.id, format!("max: {}", max)));
                        }
                    }
                    None => {}
                },
                Validation::MinExclusive(min) => match &self.value {
                    Some(value) => {
                        if value <= min {
                            return Err(ParameterError::new(
                                &self.id,
                                format!("must be greater than {}", min),
                            ));
                        }
                    }
                    None => {}
                },
                Validation::Relation(validation) => match &validation {
                    Comparison::GreaterThanOrEqual(param) => {
                        if let Some(value) = self.value {
                            match &*param.read().unwrap() {
                                ParameterType::Float(param) => {
                                    if let Some(param_value) = param.value {
                                        if value < param_value {
                                            return Err(ParameterError::new(
                                                &self.id,
                                                format!(
                                                    "must be less than or equal to: {}",
                                                    param.name
                                                ),
                                            ));
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    Comparison::LessThanOrEqual(param) => {
                        if let Some(value) = self.value {
                            match &*param.read().unwrap() {
                                ParameterType::Float(param) => {
                                    if let Some(param_value) = param.value {
                                        if value > param_value {
                                            return Err(ParameterError::new(
                                                &self.id,
                                                format!(
                                                    "must be less than or equal to: {}",
                                                    param.name
                                                ),
                                            ));
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                },
                _ => {}
            }
        }
        Ok(())
    }
}

impl Parameter<bool> {
    pub fn validate(&self) -> Result<(), ParameterError> {
        for validation in &self.validations {
            match validation {
                Validation::Required => match &self.value {
                    Some(_value) => {}
                    None => {
                        return Err(ParameterError::new(&self.id, "required".to_string()));
                    }
                },
                _ => {}
            }
        }
        Ok(())
    }
}

pub type Parameters = HashMap<String, ArcParameter>;

pub type ArcParameter = Arc<RwLock<ParameterType>>;

impl ParameterType {
    pub fn get_dependencies(&self) -> Vec<ArcParameter> {
        match self {
            ParameterType::String(p) => {
                if let Some(expression) = &p.expression {
                    expression.dependencies()
                } else {
                    vec![]
                }
            }
            ParameterType::Float(p) => {
                if let Some(expression) = &p.expression {
                    expression.dependencies()
                } else {
                    vec![]
                }
            }
            ParameterType::Bool(p) => {
                if let Some(expression) = &p.expression {
                    expression.dependencies()
                } else {
                    vec![]
                }
            }
            ParameterType::Object(_) => panic!("Object not supported yet"),
        }
    }

    pub fn id(&self) -> String {
        match self {
            ParameterType::Float(p) => p.id.clone(),
            ParameterType::String(p) => p.id.clone(),
            ParameterType::Bool(p) => p.id.clone(),
            ParameterType::Object(_) => panic!("Object not supported"),
        }
    }

    pub fn expression(&self) -> &Option<Box<dyn Equation>> {
        match self {
            ParameterType::Float(p) => &p.expression,
            ParameterType::String(p) => &p.expression,
            ParameterType::Bool(p) => &p.expression,
            ParameterType::Object(_) => panic!("Object not supported"),
        }
    }
}

pub trait ParametersTrait {
    fn get_parameter(&self, name: &str) -> ArcParameter;
    fn add(&mut self, parameter: ArcParameter);
}

impl ParametersTrait for Parameters {
    fn get_parameter(&self, name: &str) -> ArcParameter {
        if let Some(parameter) = self.get(name) {
            return parameter.clone();
        } else {
            panic!("Parameter {} not found", name);
        }
    }

    fn add(&mut self, parameter: ArcParameter) {
        self.insert(parameter.id(), parameter.clone());
    }
}

pub trait ParameterTrait {
    fn validate(&self) -> Result<(), ParameterError>;
    fn to_field(&self) -> FieldType;
    fn as_float(&self) -> f64;
    fn get_float(&self) -> Option<f64>;
    fn as_string(&self) -> String;
    fn update(&self, value: Option<String>) -> Result<(), ParameterError>;
    fn display_value(&self) -> String;
    fn id(&self) -> String;
}

impl ParameterTrait for ArcParameter {
    fn validate(&self) -> Result<(), ParameterError> {
        let p = self.read().unwrap();

        match &*p {
            ParameterType::Object(parameters) => {
                for parameter in parameters.values() {
                    parameter.validate()?;
                }
                return Ok(());
            }
            ParameterType::Float(parameter) => parameter.validate()?,
            ParameterType::String(parameter) => parameter.validate()?,
            ParameterType::Bool(parameter) => parameter.validate()?,
        }
        Ok(())
    }

    fn to_field(&self) -> FieldType {
        let p = self.read().unwrap();
        match &*p {
            ParameterType::Float(p) => FieldType::Individual(Arc::new(RwLock::new(Field {
                id: p.id.clone(),
                name: p.name.clone(),
                value: p.value.as_ref().map(|v| v.to_string()),
                touched: false,
                parameter: self.clone(),
            }))),
            ParameterType::String(p) => FieldType::Individual(Arc::new(RwLock::new(Field {
                id: p.id.clone(),
                name: p.name.clone(),
                value: p.value.as_ref().map(|v| v.to_string()),
                touched: false,
                parameter: self.clone(),
            }))),
            ParameterType::Bool(p) => FieldType::Individual(Arc::new(RwLock::new(Field {
                id: p.id.clone(),
                name: p.name.clone(),
                value: p.value.as_ref().map(|v| v.to_string()),
                touched: false,
                parameter: self.clone(),
            }))),
            ParameterType::Object(p) => panic!("Cannot convert Object to Field"),
        }
    }

    fn update(&self, value: Option<String>) -> Result<(), ParameterError> {
        let mut param = self.write().unwrap();

        match &mut *param {
            ParameterType::Object(_) => panic!("Cannot update Object"),
            ParameterType::Float(p) => {
                if let Some(value) = value {
                    if let Ok(value) = value.parse::<f64>() {
                        p.value = Some(value);
                    } else if value.is_empty() {
                        p.value = None;
                    } else {
                        return Err(ParameterError::new(&p.id, "Invalid Float".to_string()));
                    }
                } else {
                    p.value = None
                }
            }
            ParameterType::Bool(p) => {
                if let Some(value) = value {
                    if value.to_lowercase() == "true" {
                        p.value = Some(true);
                    } else if value.to_lowercase() == "false" {
                        p.value = Some(false);
                    } else if value.is_empty() {
                        p.value = None;
                    } else {
                        return Err(ParameterError::new(&p.id, "Invalid Bool".to_string()));
                    }
                } else {
                    p.value = None
                }
            }
            ParameterType::String(p) => {
                if let Some(value) = value {
                    p.value = Some(value);
                } else {
                    p.value = None
                }
            }
        }

        std::mem::drop(param);

        self.validate()?;

        Ok(())
    }

    fn as_float(&self) -> f64 {
        let p = self.read().unwrap();
        match &*p {
            ParameterType::Float(float) => match float.value {
                Some(v) => v,
                _ => panic!("Value should be a float"),
            },
            _ => panic!("Value should be a float"),
        }
    }

    fn get_float(&self) -> Option<f64> {
        let p = self.read().unwrap();
        match &*p {
            ParameterType::Float(float) => match float.value {
                Some(v) => Some(v),
                _ => None,
            },
            _ => None,
        }
    }

    fn as_string(&self) -> String {
        let p = self.read().unwrap();
        match &*p {
            ParameterType::String(string) => match string.value.clone() {
                Some(v) => v,
                _ => panic!("Value should be a string"),
            },
            _ => panic!("Value should be a string"),
        }
    }

    fn display_value(&self) -> String {
        let p = self.read().unwrap();
        match &*p {
            ParameterType::String(value) => match value.value.clone() {
                Some(v) => v.clone(),
                _ => "".to_string(),
            },
            ParameterType::Float(param) => {
                let decimal_places = param.display_options.decimal_places();

                if let (Some(decimal_places), Some(value)) = (decimal_places, param.value) {
                    format!("{:.1$}", value, decimal_places as usize)
                } else {
                    "".to_string()
                }
            }
            ParameterType::Bool(_) => panic!("Bool not supported"),
            ParameterType::Object(_) => panic!("Object not supported"),
        }
    }

    fn id(&self) -> String {
        match &*self.read().unwrap() {
            ParameterType::Float(p) => p.id.clone(),
            ParameterType::String(p) => p.id.clone(),
            ParameterType::Bool(p) => p.id.clone(),
            ParameterType::Object(_) => panic!("Object not supported"),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::sync::{Arc, RwLock};

//     fn create_parameter(value: ParameterValue) -> Arc<RwLock<ParameterType>> {
//         Arc::new(RwLock::new(Parameter {
//             id: "test".to_string(),
//             value: Some(value),
//             validations: vec![],
//             expression: None,
//             name: "Test".to_string(),
//             units: None,
//             display_options: vec![],
//         }))
//     }

//     #[test]
//     fn test_update_float() {
//         let parameter = create_parameter(ParameterValue::Float(0.0));
//         let result = parameter.update(Some("42.0".to_string()));
//         assert!(result.is_ok());
//         assert_eq!(
//             parameter.read().unwrap().value,
//             Some(ParameterValue::Float(42.0))
//         );
//     }

//     #[test]
//     fn test_update_invalid_float() {
//         let parameter = create_parameter(ParameterValue::Float(0.0));
//         let result = parameter.update(Some("invalid".to_string()));
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_update_bool_true() {
//         let parameter = create_parameter(ParameterValue::Bool(false));
//         let result = parameter.update(Some("true".to_string()));
//         assert!(result.is_ok());
//         assert_eq!(
//             parameter.read().unwrap().value,
//             Some(ParameterValue::Bool(true))
//         );
//     }

//     #[test]
//     fn test_update_bool_false() {
//         let parameter = create_parameter(ParameterValue::Bool(true));
//         let result = parameter.update(Some("false".to_string()));
//         assert!(result.is_ok());
//         assert_eq!(
//             parameter.read().unwrap().value,
//             Some(ParameterValue::Bool(false))
//         );
//     }

//     #[test]
//     fn test_update_invalid_bool() {
//         let parameter = create_parameter(ParameterValue::Bool(false));
//         let result = parameter.update(Some("invalid".to_string()));
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_update_string() {
//         let parameter = create_parameter(ParameterValue::String("old".to_string()));
//         let result = parameter.update(Some("new".to_string()));
//         assert!(result.is_ok());
//         assert_eq!(
//             parameter.read().unwrap().value,
//             Some(ParameterValue::String("new".to_string()))
//         );
//     }
// }
