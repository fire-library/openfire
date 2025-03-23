// pub mod builder;

use super::{
    calculation::CalculationComponent,
    parameter::{ArcParameter, ParameterTrait},
};
use crate::method::validation::ParameterError;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type ArcField = Arc<RwLock<FieldType>>;

pub trait FieldTrait {
    fn update(&self, value: Option<String>) -> Result<(), Vec<ParameterError>>;
    fn sub_fields(&self) -> Vec<ArcField>;
    fn id(&self) -> String;
}

impl FieldTrait for ArcField {
    fn update(&self, value: Option<String>) -> Result<(), Vec<ParameterError>> {
        let mut field = self.write().unwrap();
        match &mut *field {
            FieldType::Individual(field) => {
                field.value = value.clone();
                field.touched = true;
                field.parameter.update(value)?;
            }
            FieldType::StringEnum(field, _values) => {
                field.value = value.clone();
                field.touched = true;
                field.parameter.update(value)?;
            }
            FieldType::Object(_fields) => {
                panic!("Cannot update an Object field directly");
            }
            FieldType::List(_) => {
                panic!("Cannot update an Object field directly");
            }
        }
        Ok(())
    }

    fn id(&self) -> String {
        match &*self.read().unwrap() {
            FieldType::Individual(field) => field.id.clone(),
            FieldType::StringEnum(field, _) => field.id.clone(),
            FieldType::Object(field) => field.id.clone(),
            FieldType::List(field) => field.id.clone(),
        }
    }

    fn sub_fields(&self) -> Vec<ArcField> {
        let mut sub_fields = vec![];
        match &*self.read().unwrap() {
            FieldType::Individual(_) => {}
            FieldType::StringEnum(_, _) => {}
            FieldType::Object(fields) => {
                if let Some(fields) = &fields.value {
                    for field in fields {
                        sub_fields.push(field.clone());
                        sub_fields.extend(field.sub_fields());
                    }
                }
            }
            FieldType::List(fields) => {
                if let Some(fields) = &fields.value {
                    for field in fields {
                        sub_fields.push(field.clone());
                        sub_fields.extend(field.sub_fields());
                    }
                }
            }
        }

        return sub_fields;
    }
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum IntroComponent {
    Title(String),
    Text(String),
    Equation(CalculationComponent),
}

#[derive(Clone, Type, Serialize, Debug)]
pub struct FormStep {
    pub name: String,
    pub description: String,
    pub fields: Vec<ArcField>,
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

    pub fn add_field(&mut self, field: Option<ArcField>) {
        if let Some(field) = field {
            self.fields.push(field);
        }
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

    pub fn add_intro(&mut self) {
        self.introduction.push(vec![]);
    }
}

#[derive(Type, Serialize, Debug)]
pub enum FieldType {
    Individual(Field<String>),
    StringEnum(Field<String>, Vec<String>),
    Object(Field<Vec<ArcField>>),
    List(Field<Vec<ArcField>>),
}

impl FieldType {
    pub fn validate(&self) -> Result<(), Vec<ParameterError>> {
        match self {
            FieldType::Individual(field) => match field.parameter.validate() {
                Ok(_) => Ok(()),
                Err(error) => Err(error),
            },
            FieldType::StringEnum(field, _) => match field.parameter.validate() {
                Ok(_) => Ok(()),
                Err(error) => Err(error),
            },
            FieldType::Object(field) => {
                let mut errors = vec![];
                if let Some(fields) = &field.value {
                    for inner_field in fields {
                        if let Err(error) = inner_field.read().unwrap().validate() {
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
            FieldType::List(field) => {
                let mut errors = vec![];
                if let Some(fields) = &field.value {
                    for inner_field in fields {
                        if let Err(error) = inner_field.read().unwrap().validate() {
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
    }

    pub fn symbol(&self) -> String {
        match self {
            FieldType::Individual(field) => field.symbol.clone(),
            FieldType::StringEnum(field, _) => field.symbol.clone(),
            FieldType::Object(field) => field.symbol.clone(),
            FieldType::List(field) => field.symbol.clone(),
        }
    }
}

#[derive(Clone, Type, Serialize, Debug)]
pub struct Field<T> {
    pub id: String,
    pub name: String,
    pub value: Option<T>,
    pub symbol: String,
    pub units: Option<String>,
    pub touched: bool,
    pub parameter: ArcParameter,
    #[serde(skip)]
    pub create_type: Option<fn() -> ArcParameter>,
}

#[derive(Clone, Type, Serialize, Debug)]
pub struct Form {
    fields: HashMap<String, ArcField>,
    steps: Vec<FormStep>,
}

impl Form {
    pub fn new(steps: Vec<FormStep>) -> Self {
        let mut form = Form {
            steps: vec![],
            fields: HashMap::new(),
        };

        for step in steps {
            form.add_step(step);
        }

        return form;
    }

    pub fn add_field(&mut self, field: ArcField) {
        self.fields.insert(field.id(), field.clone());

        for field in field.sub_fields() {
            self.fields.insert(field.id(), field.clone());
        }
    }

    pub fn add_step(&mut self, step: FormStep) {
        for field in &step.fields {
            self.add_field(field.clone());
        }

        self.steps.push(step);
    }

    pub fn get_field(&self, field_id: String) -> ArcField {
        if let Some(field) = self.fields.get(&field_id) {
            return field.clone();
        } else {
            panic!("Field not found: {}", field_id);
        }
    }

    pub fn validate(&self) -> Result<(), Vec<ParameterError>> {
        let mut errors = vec![];
        for field in self.fields.values() {
            if let Err(error) = field.read().unwrap().validate() {
                errors.extend(error);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
