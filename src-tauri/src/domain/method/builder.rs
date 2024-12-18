use crate::domain::impls::tag::Tag;
use crate::domain::impls::Icon;
use crate::domain::impls::Implementation;
use crate::domain::method::form::Form;
use crate::domain::method::parameter::Parameters;
use crate::domain::method::MethodType;
use crate::domain::method::{Method, Reference};

use super::calculation::ArcCalculation;

pub trait MethodBuilderTrait {
    fn name() -> String;
    fn tags() -> Vec<Tag>;
    fn description() -> Option<String>;
    fn reference() -> Reference;
    fn parameters() -> Parameters;
    fn quick_calc_compatible() -> bool;
    fn calc_sheet(params: &Parameters) -> ArcCalculation;
    fn form(params: &Parameters) -> Form;
    fn method_type() -> MethodType;
    fn index_page() -> Implementation {
        Implementation {
            name: Self::name(),
            tags: Self::tags().iter().map(|t| t.to_string()).collect(),
            description: Self::description().unwrap_or("".to_string()),
            reference: Self::reference(),
            search_reference: Self::reference().0.friendly_reference(),
            method_type: Self::method_type(),
            icon: Icon::FireIcon,
            colors: "text-red-700 bg-red-50".to_string(),
        }
    }
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
