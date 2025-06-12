pub mod calculation;
pub mod equation;
pub mod form;
pub mod implementation;
pub mod parameter;
pub mod parameters;
pub mod runner;
pub mod step;
pub mod tag;
pub mod test;
pub mod validation;

use crate::filesystem::saved_method::SavedMethod;
use calculation::ArcCalculation;
use parameter::ArcParameter;
use parameter::ParameterType;
use parameter::ParameterValue;
use parameters::Parameters;
use serde::Serialize;
use specta::Type;
use validation::ParameterError;

#[derive(Clone, Type, Serialize, Debug)]
pub struct Method {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub parameters: Parameters,
    pub quick_calc: Option<Vec<ArcParameter>>,
    pub calc_sheet: ArcCalculation,
    pub form: form::Form,
}

impl Method {
    pub fn evaluate(&mut self) -> Result<(), Vec<ParameterError>> {
        let runner = crate::get_runner(&self.id);
        runner.evaluate(self)?;

        self.calc_sheet = runner.calc_sheet(&self.parameters, Some(false));

        Ok(())
    }

    pub fn update_form(&mut self) {
        self.form = crate::get_runner(&self.id).form(&self.parameters);
    }
}

impl TryFrom<SavedMethod> for Method {
    type Error = Vec<ParameterError>;

    fn try_from(saved: SavedMethod) -> Result<Self, Self::Error> {
        let runner = crate::get_runner(&saved.id);
        let mut method = runner.build_method();
        let mut errors = vec![];

        for param in saved.parameters {
            let p = method.parameters.get(&param.name);
            let mut p_write = p.write().unwrap();

            match (&mut *p_write, param.value) {
                (ParameterType::Float(float), Some(ParameterValue::Float(saved))) => {
                    float.value = Some(saved);
                }
                (ParameterType::Integer(integer), Some(ParameterValue::Integer(saved))) => {
                    integer.value = Some(saved);
                }
                (ParameterType::String(string), Some(ParameterValue::String(saved))) => {
                    string.value = Some(saved);
                }
                (ParameterType::Float(float), None) => {
                    float.value = None;
                }
                (ParameterType::Integer(integer), None) => {
                    integer.value = None;
                }
                (ParameterType::String(string), None) => {
                    string.value = None;
                }
                (ParameterType::String(string), _) => {
                    errors.push(ParameterError::new(
                        &string.id,
                        &string.symbol,
                        "Parameter type mismatch - Expected a string".to_string(),
                    ));
                }
                (ParameterType::Float(float), _) => {
                    errors.push(ParameterError::new(
                        &float.id,
                        &float.symbol,
                        "Parameter type mismatch - Expected a number".to_string(),
                    ));
                }
                _ => panic!("Parameter type mismatch"),
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        } else {
            method.form = runner.form(&method.parameters);
            Ok(method)
        }
    }
}
