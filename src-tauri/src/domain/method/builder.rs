use crate::domain::method::form::Form;
use crate::domain::method::parameter::Parameters;
use crate::domain::method::Metadata;
use crate::domain::method::Method;
use crate::domain::method::MethodType;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::calculation::ArcCalculation;
use super::form::FormStep;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct MethodBuilder {
    pub name: String,
    pub metadata: Vec<MetadataBuilder>,
    pub description: Option<String>,
    pub reference: Vec<String>,
    pub parameters: Parameters,
    pub quick_calc_compatible: bool,
    pub calc_sheet: Option<ArcCalculation>,
    pub form: Form,
    pub method_type: Option<MethodType>,
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct MetadataBuilder {
    pub id: String,
    pub name: String,
    pub required: bool,
}

impl MetadataBuilder {
    pub fn new(name: String, required: bool) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name,
            required: required,
        }
    }
}

impl MethodBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            metadata: vec![],
            description: None,
            reference: vec![],
            parameters: Parameters::new(),
            quick_calc_compatible: false,
            calc_sheet: None,
            form: Form::new(),
            method_type: None,
        }
    }

    pub fn reference(mut self, reference: Vec<&str>) -> Self {
        let reference: Vec<String> = reference.iter().map(|r| r.to_string()).collect();
        self.reference = reference;
        self
    }

    pub fn parameters(mut self, parameters: Parameters) -> Self {
        self.parameters = parameters;
        self
    }

    pub fn quick_calc_compatible(mut self, quick_calc_compatible: bool) -> Self {
        self.quick_calc_compatible = quick_calc_compatible;
        self
    }

    pub fn calc_sheet(mut self, result: ArcCalculation) -> Self {
        self.calc_sheet = Some(result);
        self
    }

    pub fn add_form_step(mut self, step: FormStep) -> Self {
        self.form.add_step(step);
        self
    }

    pub fn add_metadata(&mut self) -> &Self {
        let metadata = MetadataBuilder::new("".to_string(), false);
        self.metadata.push(metadata);
        self
    }

    pub fn method_type(mut self, method_type: MethodType) -> Self {
        self.method_type = Some(method_type);
        self
    }

    pub fn delete_metadata(&mut self, id: String) -> &Self {
        self.metadata.retain(|m| m.id != id);
        self
    }

    pub fn update_metadata_name(&mut self, id: String, name: String) -> &Self {
        for m in &mut self.metadata {
            if m.id == id {
                m.name = name.clone();
            }
        }

        self
    }

    pub fn update_metadata_required(&mut self, id: String, required: bool) -> &Self {
        for m in &mut self.metadata {
            if m.id == id {
                m.required = required.clone();
            }
        }

        self
    }

    pub fn build(self) -> Method {
        Method {
            name: self.name,
            metadata: self
                .metadata
                .iter()
                .map(|m| Metadata {
                    name: m.name.clone(),
                    required: m.required,
                    value: None,
                })
                .collect(),
            parameters: self.parameters,
            method_type: self.method_type.unwrap(),
            quick_calc_compatible: self.quick_calc_compatible,
            calc_sheet: self.calc_sheet.unwrap(),
            description: self.description,
            reference: self.reference,
            form: self.form,
        }
    }
}
