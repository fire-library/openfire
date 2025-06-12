use crate::method::form::Form;
use crate::method::parameters::Parameters;
use crate::method::tag::Tag;

use crate::method::Method;
use crate::method::validation::ParameterError;

use super::calculation::ArcCalculation;
use super::calculation::Calculation;
use super::implementation::{Icon, Implementation};
use super::parameter::ArcParameter;
use super::test::IntegrationTests;
use std::sync::RwLock;

pub trait Reference {
    fn id(&self) -> String;
    fn document_id(&self) -> String;
    fn document_name(&self) -> String;
    fn friendly_reference(&self) -> String;
    fn harvard_reference(&self) -> String;
    fn about_document(&self) -> String;
    fn about_method(&self) -> String;
    fn method_limitations(&self) -> String;
}

pub trait MethodRunner {
    fn id(&self) -> String {
        self.reference().id()
    }
    fn name(&self) -> String;
    fn reference(&self) -> &dyn Reference;
    fn tags(&self) -> Vec<Tag>;
    fn description(&self) -> Option<String>;
    fn parameters(&self) -> Parameters;
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>>;
    fn calc_sheet(&self, params: &Parameters, stale: Option<bool>) -> ArcCalculation;
    fn form(&self, params: &Parameters) -> Form;
    fn index_page(&self) -> Implementation {
        Implementation {
            name: self.name(),
            tags: self.tags().iter().map(|t| t.to_string()).collect(),
            description: self.description().unwrap_or("".to_string()),
            id: self.id(),
            search_reference: "asdf".to_string(),
            icon: Icon::FireIcon,
            colors: "text-red-700 bg-red-50".to_string(),
        }
    }
    fn evaluate(&self, method: &mut Method) -> Result<(), Vec<ParameterError>>;
    fn build_method(&self) -> Method {
        let parameters = self.parameters();

        Method {
            name: self.name(),
            description: self.description(),
            id: self.id(),
            quick_calc: self.quick_calc(&parameters),
            form: self.form(&parameters),
            calc_sheet: ArcCalculation::new(RwLock::new(Calculation::new(true))),
            parameters: parameters,
        }
    }

    fn tests(&self) -> Option<IntegrationTests> {
        None
    }
}
