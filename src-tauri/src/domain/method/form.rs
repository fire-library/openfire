// pub mod builder;

use super::parameter::{ArcParameter, ParameterTrait};
use crate::domain::method::validation::ParameterError;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::{Arc, RwLock};

pub type ArcField = Arc<RwLock<Field>>;

pub trait FieldTrait {
    fn update(&self, value: Option<String>) -> Result<(), ParameterError>;
}

impl FieldTrait for ArcField {
    fn update(&self, value: Option<String>) -> Result<(), ParameterError> {
        let mut field = self.write().unwrap();
        field.value = value.clone();
        field.touched = true;
        field.parameter.update(value)?;
        Ok(())
    }
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct FormStep {
    pub name: String,
    pub description: String,
    pub fields: Vec<ArcField>,
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct Field {
    pub id: String,
    pub name: String,
    pub value: Option<String>,
    pub touched: bool,
    pub parameter: ArcParameter,
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct Form {
    pub steps: Vec<FormStep>,
}

impl Form {
    pub fn new() -> Self {
        Form { steps: vec![] }
    }

    pub fn add_step(&mut self, step: FormStep) {
        self.steps.push(step);
    }

    pub fn get_field(&self, field_id: String) -> ArcField {
        for step in &self.steps {
            for field in &step.fields {
                if field.read().unwrap().id == field_id {
                    return field.clone();
                }
            }
        }

        unreachable!()
    }

    pub fn validate(&self) -> Result<(), Vec<ParameterError>> {
        let mut errors = vec![];
        for step in &self.steps {
            for field in &step.fields {
                if let Err(error) = field.read().unwrap().parameter.validate() {
                    errors.push(error);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
