use crate::domain::method::form::Form;
use crate::domain::method::parameter::Parameters;
use crate::domain::method::Method;
use crate::domain::method::MethodType;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::calculation::ArcCalculation;
use super::form::FormStep;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct MethodBuilder {
    pub name: String,
    pub description: Option<String>,
    pub reference: Vec<String>,
    pub parameters: Parameters,
    pub quick_calc_compatible: bool,
    pub calc_sheet: Option<ArcCalculation>,
    pub form: Form,
    pub method_type: Option<MethodType>,
}

pub trait MethodBuilderTrait {
    fn name() -> String;
    fn description() -> Option<String>;
    fn reference() -> Vec<String>;
    fn parameters() -> Parameters;
    fn quick_calc_compatible() -> bool;
    fn calc_sheet(params: &Parameters) -> ArcCalculation;
    fn form(params: &Parameters) -> Form;
    fn method_type() -> MethodType;
    fn build_method() -> Method {
        let parameters = Self::parameters();

        Method {
            name: Self::name(),
            description: Self::description(),
            reference: Self::reference(),
            method_type: Self::method_type(),
            quick_calc_compatible: Self::quick_calc_compatible(),
            form: Self::form(&parameters),
            calc_sheet: Self::calc_sheet(&parameters),
            parameters: parameters,
        }
    }
}

impl MethodBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
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

    pub fn method_type(mut self, method_type: MethodType) -> Self {
        self.method_type = Some(method_type);
        self
    }

    pub fn build(self) -> Method {
        Method {
            name: self.name,
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
