// pub mod builder;

use super::{
    calculation::CalculationComponent,
    parameter::{ArcParameter, ParameterTrait},
};
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
pub enum IntroComponent {
    Title(String),
    Text(String),
    Equation(CalculationComponent),
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct FormStep {
    pub name: String,
    pub description: String,
    pub fields: Vec<FieldType>,
    pub introduction: Vec<Vec<IntroComponent>>,
}

impl FormStep {
    pub fn new(name: &str, description: &str) -> Self {
        FormStep {
            name: name.to_string(),
            description: description.to_string(),
            fields: vec![],
            introduction: vec![],
        }
    }

    pub fn add_field(&mut self, field: FieldType) {
        self.fields.push(field);
    }

    pub fn add_text(&mut self, text: &str) {
        if let Some(last) = self.introduction.last_mut() {
            last.push(IntroComponent::Text(text.to_string()));
            return;
        }
    }
    pub fn add_equation(&mut self, equation: CalculationComponent) {
        if let Some(last) = self.introduction.last_mut() {
            last.push(IntroComponent::Equation(equation));
            return;
        }
    }
    pub fn add_title(&mut self, title: &str) {
        if let Some(last) = self.introduction.last_mut() {
            last.push(IntroComponent::Title(title.to_string()));
            return;
        }
    }
    pub fn add_intro(&mut self) {
        self.introduction.push(vec![]);
    }
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum FieldType {
    Individual(ArcField),
    Table(Vec<Vec<ArcField>>),
}

impl FieldType {
    pub fn get_field(&self, id: String) -> Option<ArcField> {
        match self {
            FieldType::Individual(field) => {
                if field.read().unwrap().id == id {
                    Some(field.clone())
                } else {
                    None
                }
            }
            FieldType::Table(fields) => {
                for row in fields {
                    for field in row {
                        if field.read().unwrap().id == id {
                            return Some(field.clone());
                        }
                    }
                }
                None
            }
        }
    }

    pub fn validate(&self) -> Result<(), Vec<ParameterError>> {
        match self {
            FieldType::Individual(field) => match field.read().unwrap().parameter.validate() {
                Ok(_) => Ok(()),
                Err(error) => Err(vec![error]),
            },
            FieldType::Table(fields) => {
                let mut errors = vec![];
                for row in fields {
                    for field in row {
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
    }
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
            for field_types in &step.fields {
                if let Some(field) = field_types.get_field(field_id.clone()) {
                    return field.clone();
                }
            }
        }

        unreachable!()
    }

    pub fn validate(&self) -> Result<(), Vec<ParameterError>> {
        let mut errors = vec![];
        for step in &self.steps {
            for field_types in &step.fields {
                if let Err(error) = field_types.validate() {
                    errors.extend(error);
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
