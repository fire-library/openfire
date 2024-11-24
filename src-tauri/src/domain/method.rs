pub mod builder;
pub mod calculation;
pub mod equation;
pub mod form;
pub mod parameter;
pub mod step;
pub mod validation;

// use parameter::{ArcParameter, ParameterTrait, Parameters};
use calculation::ArcCalculation;
use parameter::Parameters;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::impls::br187;
use super::impls::pd7974;
use super::impls::sfpe_handbook;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct Method {
    pub name: String,
    pub metadata: Vec<Metadata>,
    pub description: Option<String>,
    pub reference: Vec<String>,
    pub method_type: MethodType,
    pub parameters: Parameters,
    pub quick_calc_compatible: bool,
    pub calc_sheet: ArcCalculation,
    pub form: form::Form,
}

impl Method {
    pub fn evaluate(&mut self) -> Result<(), String> {
        match &self.method_type {
            MethodType::PD7974Part2Section7Equation1 => {
                pd7974::part_2::section_7::equation_1::evaluate(self)?
            }
            MethodType::BR187Chapter1Equation1 => br187::chapter_1::equation_1::evaluate(self)?,
            MethodType::SFPEAlpertHeatReleaseFromTemperatureAndPosition => {
                sfpe_handbook::alpert::heat_release_from_temp_and_position::evaluate(self)?
            }
        };

        self.calc_sheet.write().unwrap().stale = false;

        Ok(())
    }
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct Metadata {
    name: String,
    required: bool,
    value: Option<String>,
}

impl Metadata {
    pub fn new(name: String, required: bool) -> Self {
        Self {
            name: name,
            required: required,
            value: None,
        }
    }

    pub fn update(&mut self, value: Option<String>) {
        self.value = value;
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.required && self.value.is_none() {
            return Err(format!("{} is required", self.name));
        }

        Ok(())
    }
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum MethodType {
    PD7974Part2Section7Equation1,
    BR187Chapter1Equation1,
    SFPEAlpertHeatReleaseFromTemperatureAndPosition,
}

impl MethodType {
    pub fn method(&self) -> Method {
        match &self {
            &MethodType::PD7974Part2Section7Equation1 => {
                pd7974::part_2::section_7::equation_1::method()
            }
            &MethodType::BR187Chapter1Equation1 => br187::chapter_1::equation_1::method(),
            &MethodType::SFPEAlpertHeatReleaseFromTemperatureAndPosition => {
                sfpe_handbook::alpert::heat_release_from_temp_and_position::method()
            }
        }
    }
}
