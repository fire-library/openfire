use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

#[derive(Clone, Type, Serialize, Deserialize, Debug, PartialEq)]
pub enum Validation {
    Required,
    IsFloat,
    MinLength(u32),
    MaxLength(u32),
    Range(f64, f64),
    Min(f64),
    Max(f64),
}

#[derive(Debug, Error, Serialize, Deserialize, Type)]
pub enum ParameterError {
    #[error("Validation failed on field '{id}': {message}")]
    ValidationError {
        id: String,
        symbol: String,
        message: String,
    },
}

impl ParameterError {
    pub fn parameter_id(&self) -> String {
        match self {
            ParameterError::ValidationError { id, .. } => id.clone(),
        }
    }
    pub fn message(&self) -> String {
        match self {
            ParameterError::ValidationError { message, .. } => message.clone(),
        }
    }
}

impl ParameterError {
    pub fn new(id: &str, symbol: &str, message: String) -> Self {
        ParameterError::ValidationError {
            id: id.to_string(),
            symbol: symbol.to_string(),
            message: message,
        }
    }
}
