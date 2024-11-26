pub mod builder;
use super::equation::Equation;
use super::validation::ParameterError;
use crate::domain::method::form::{ArcField, Field};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};

#[derive(Clone, Type, Serialize, Deserialize, Debug, PartialEq)]
pub enum ParameterValue {
    String(String),
    Float(f64),
    Bool(bool),
    Null(ParameterType),
}

impl ParameterValue {
    pub fn is_some(&self) -> bool {
        match self {
            ParameterValue::Null(_) => false,
            _ => true,
        }
    }

    pub fn to_string(&self) -> Option<String> {
        match self {
            ParameterValue::String(value) => Some(value.clone()),
            ParameterValue::Float(value) => Some(value.to_string()),
            ParameterValue::Bool(value) => Some(value.to_string()),
            ParameterValue::Null(_) => None,
        }
    }

    pub fn to_float(&self) -> Result<f64, String> {
        match self {
            ParameterValue::Float(value) => Ok(value.clone()),
            _ => Err("Value is not a float".to_string()),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            ParameterValue::Bool(value) => *value,
            _ => false,
        }
    }
}

#[derive(Clone, Type, Serialize, Deserialize, Debug, PartialEq)]
pub enum ParameterType {
    String,
    Float,
    Bool,
}

impl fmt::Display for ParameterValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParameterValue::Null(_) => write!(f, ""),
            ParameterValue::String(value) => write!(f, "{}", value),
            ParameterValue::Float(value) => write!(f, "{}", value),
            ParameterValue::Bool(value) => {
                if *value {
                    write!(f, "True")
                } else {
                    write!(f, "False")
                }
            }
        }
    }
}

#[derive(Clone, Type, Serialize, Deserialize, Debug, PartialEq)]
pub enum ValueState<T> {
    Invalid(String),
    Valid(T),
}

impl<T> ValueState<T> {
    pub fn unwrap(self) -> T {
        match self {
            ValueState::Valid(value) => value,
            ValueState::Invalid(_) => panic!("Value is invalid"),
        }
    }
}

#[derive(Clone, Type, Serialize, Deserialize, Debug, PartialEq)]
pub enum Validation {
    Required,
    MinLength(u32),
    MaxLength(u32),
    Range(f64, f64),
    MinExclusive(f64),
    Min(f64),
    Max(f64),
}

#[derive(Type, Serialize, Deserialize, Debug)]
pub struct Parameter {
    pub id: String,
    pub name: String,
    #[specta(skip)]
    #[serde(skip)]
    pub expression: Option<Box<dyn Equation>>,
    pub value: ParameterValue,
    pub units: Option<String>,
    pub validations: Vec<Validation>,
}

pub type Parameters = HashMap<String, ArcParameter>;

pub type ArcParameter = Arc<RwLock<Parameter>>;

impl Parameter {
    pub fn get_dependencies(&self) -> Vec<ArcParameter> {
        if let Some(expression) = &self.expression {
            expression.dependencies()
        } else {
            vec![]
        }
    }
}

pub trait ParametersTrait {
    fn get_parameter(&self, name: &str) -> ArcParameter;
    fn add(&mut self, parameter: ArcParameter);
    fn validate(&mut self) -> Result<(), Vec<ParameterError>>;
    fn update_field(&mut self, name: &str, value: Option<String>) -> Result<(), ParameterError> {
        let parameter = self.get_parameter(name);
        let mut parameter = parameter.write().unwrap();

        if let Some(value) = value {
            match parameter.value.clone() {
                ParameterValue::String(_) => {
                    parameter.value = ParameterValue::String(value);
                }
                ParameterValue::Bool(_) => {
                    if value.to_lowercase() == "true" {
                        parameter.value = ParameterValue::Bool(true);
                    } else if value.to_lowercase() == "false" {
                        parameter.value = ParameterValue::Bool(false);
                    } else {
                        return Err(ParameterError::new(
                            &parameter.id,
                            "Invalid boolean".to_string(),
                        ));
                    }
                }
                ParameterValue::Float(_) => match value.parse::<f64>() {
                    Ok(value) => {
                        parameter.value = ParameterValue::Float(value);
                    }
                    Err(_) => {
                        return Err(ParameterError::new(
                            &parameter.id,
                            "Invalid number".to_string(),
                        ));
                    }
                },
                ParameterValue::Null(parameter_type) => match parameter_type {
                    ParameterType::String => {
                        parameter.value = ParameterValue::String(value);
                    }
                    ParameterType::Bool => {
                        if value.to_lowercase() == "true" {
                            parameter.value = ParameterValue::Bool(true);
                        } else if value.to_lowercase() == "false" {
                            parameter.value = ParameterValue::Bool(false);
                        } else {
                            return Err(ParameterError::new(
                                &parameter.id,
                                "Invalid boolean".to_string(),
                            ));
                        }
                    }
                    ParameterType::Float => match value.parse::<f64>() {
                        Ok(value) => {
                            parameter.value = ParameterValue::Float(value);
                        }
                        Err(_) => {
                            return Err(ParameterError::new(
                                &parameter.id,
                                "Invalid number".to_string(),
                            ));
                        }
                    },
                },
            }
        }

        Ok(())
    }
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
        self.insert(parameter.read().unwrap().id.clone(), parameter.clone());
    }

    fn validate(&mut self) -> Result<(), Vec<ParameterError>> {
        let mut errors = vec![];
        for parameter in self.values() {
            if let Err(error) = parameter.validate() {
                errors.push(error);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

pub trait ParameterTrait {
    fn validate(&self) -> Result<(), ParameterError>;
    fn to_field(&self) -> ArcField;
    fn update(&self, value: Option<String>) -> Result<(), ParameterError>;
}

impl ParameterTrait for ArcParameter {
    fn validate(&self) -> Result<(), ParameterError> {
        let p = self.read().unwrap();
        let value = p.value.clone();
        for validation in &p.validations {
            match validation {
                Validation::Required => match &value {
                    ParameterValue::String(string) => {
                        if string.trim().is_empty() {
                            return Err(ParameterError::new(&p.id, "required".to_string()));
                        }
                    }
                    ParameterValue::Null(_) => {
                        return Err(ParameterError::new(&p.id, "required".to_string()));
                    }
                    _ => {}
                },
                Validation::MinLength(min) => match &value {
                    ParameterValue::String(value) => {
                        if (value.len() as u32) < *min {
                            return Err(ParameterError::new(
                                &p.id,
                                format!("must be at least {} characters", min),
                            ));
                        }
                    }
                    _ => {}
                },
                Validation::MaxLength(max) => match &value {
                    ParameterValue::String(value) => {
                        if (value.len() as u32) > *max {
                            return Err(ParameterError::new(
                                &p.id,
                                format!("must be at most {} characters", max),
                            ));
                        }
                    }
                    _ => {}
                },
                Validation::Range(min, max) => match &value {
                    ParameterValue::Float(value) => {
                        if value < min || value > max {
                            return Err(ParameterError::new(
                                &p.id,
                                format!("must be between {} and {}", min, max),
                            ));
                        }
                    }
                    _ => {}
                },
                Validation::Min(min) => match &value {
                    ParameterValue::Float(value) => {
                        if value < min {
                            return Err(ParameterError::new(&p.id, format!("min: {}", min)));
                        }
                    }
                    _ => {}
                },
                Validation::Max(max) => match &value {
                    ParameterValue::Float(value) => {
                        if value > max {
                            return Err(ParameterError::new(&p.id, format!("max: {}", max)));
                        }
                    }
                    _ => {}
                },
                Validation::MinExclusive(min) => match &value {
                    ParameterValue::Float(value) => {
                        if value <= min {
                            return Err(ParameterError::new(
                                &p.id,
                                format!("must be greater than {}", min),
                            ));
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }

    fn to_field(&self) -> ArcField {
        let p = self.read().unwrap();
        Arc::new(RwLock::new(Field {
            id: p.id.clone(),
            name: p.name.clone(),
            value: p.value.to_string(),
            touched: false,
            parameter: self.clone(),
        }))
    }

    fn update(&self, value: Option<String>) -> Result<(), ParameterError> {
        let mut p = self.write().unwrap();

        if let Some(value) = value {
            match p.value.clone() {
                ParameterValue::Float(_float) => {
                    if let Ok(value) = value.parse::<f64>() {
                        p.value = ParameterValue::Float(value);
                    } else {
                        return Err(ParameterError::new(&p.id, "Invalid number".to_string()));
                    }
                }
                ParameterValue::Bool(_bool) => {
                    if value.to_lowercase() == "true" {
                        p.value = ParameterValue::Bool(true);
                    } else if value.to_lowercase() == "false" {
                        p.value = ParameterValue::Bool(false);
                    } else {
                        return Err(ParameterError::new(&p.id, "Invalid boolean".to_string()));
                    }
                }
                ParameterValue::String(_string) => {
                    p.value = ParameterValue::String(value);
                }
                ParameterValue::Null(param_type) => match param_type {
                    ParameterType::String => {
                        p.value = ParameterValue::String(value);
                    }
                    ParameterType::Bool => {
                        if value.to_lowercase() == "true" {
                            p.value = ParameterValue::Bool(true);
                        } else if value.to_lowercase() == "false" {
                            p.value = ParameterValue::Bool(false);
                        } else {
                            return Err(ParameterError::new(&p.id, "Invalid boolean".to_string()));
                        }
                    }
                    ParameterType::Float => {
                        if let Ok(value) = value.parse::<f64>() {
                            p.value = ParameterValue::Float(value);
                        } else {
                            return Err(ParameterError::new(&p.id, "Invalid number".to_string()));
                        }
                    }
                },
            }
        } else {
            match p.value.clone() {
                ParameterValue::Float(_float) => {
                    p.value = ParameterValue::Null(ParameterType::Float);
                }
                ParameterValue::Bool(_bool) => {
                    p.value = ParameterValue::Null(ParameterType::Bool);
                }
                ParameterValue::String(_string) => {
                    p.value = ParameterValue::Null(ParameterType::String);
                }
                ParameterValue::Null(_) => {}
            }
        }
        std::mem::drop(p);

        self.validate()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, RwLock};

    fn create_parameter(value: ParameterValue) -> Arc<RwLock<Parameter>> {
        Arc::new(RwLock::new(Parameter {
            id: "test".to_string(),
            value,
            validations: vec![],
            expression: None,
            name: "Test".to_string(),
            units: None,
        }))
    }

    #[test]
    fn test_update_float() {
        let parameter = create_parameter(ParameterValue::Float(0.0));
        let result = parameter.update(Some("42.0".to_string()));
        assert!(result.is_ok());
        assert_eq!(parameter.read().unwrap().value, ParameterValue::Float(42.0));
    }

    #[test]
    fn test_update_invalid_float() {
        let parameter = create_parameter(ParameterValue::Float(0.0));
        let result = parameter.update(Some("invalid".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_update_bool_true() {
        let parameter = create_parameter(ParameterValue::Bool(false));
        let result = parameter.update(Some("true".to_string()));
        assert!(result.is_ok());
        assert_eq!(parameter.read().unwrap().value, ParameterValue::Bool(true));
    }

    #[test]
    fn test_update_bool_false() {
        let parameter = create_parameter(ParameterValue::Bool(true));
        let result = parameter.update(Some("false".to_string()));
        assert!(result.is_ok());
        assert_eq!(parameter.read().unwrap().value, ParameterValue::Bool(false));
    }

    #[test]
    fn test_update_invalid_bool() {
        let parameter = create_parameter(ParameterValue::Bool(false));
        let result = parameter.update(Some("invalid".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_update_string() {
        let parameter = create_parameter(ParameterValue::String("old".to_string()));
        let result = parameter.update(Some("new".to_string()));
        assert!(result.is_ok());
        assert_eq!(
            parameter.read().unwrap().value,
            ParameterValue::String("new".to_string())
        );
    }

    #[test]
    fn test_update_null_to_string() {
        let parameter = create_parameter(ParameterValue::Null(ParameterType::String));
        let result = parameter.update(Some("new".to_string()));
        assert!(result.is_ok());
        assert_eq!(
            parameter.read().unwrap().value,
            ParameterValue::String("new".to_string())
        );
    }

    #[test]
    fn test_update_null_to_bool() {
        let parameter = create_parameter(ParameterValue::Null(ParameterType::Bool));
        let result = parameter.update(Some("true".to_string()));
        assert!(result.is_ok());
        assert_eq!(parameter.read().unwrap().value, ParameterValue::Bool(true));
    }

    #[test]
    fn test_update_null_to_float() {
        let parameter = create_parameter(ParameterValue::Null(ParameterType::Float));
        let result = parameter.update(Some("42.0".to_string()));
        assert!(result.is_ok());
        assert_eq!(parameter.read().unwrap().value, ParameterValue::Float(42.0));
    }

    #[test]
    fn test_update_to_null() {
        let parameter = create_parameter(ParameterValue::Float(42.0));
        let result = parameter.update(None);
        assert!(result.is_ok());
        assert_eq!(
            parameter.read().unwrap().value,
            ParameterValue::Null(ParameterType::Float)
        );
    }
}
