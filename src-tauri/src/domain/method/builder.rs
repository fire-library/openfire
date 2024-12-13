use crate::domain::method::form::Form;
use crate::domain::method::parameter::Parameters;
use crate::domain::method::Method;
use crate::domain::method::MethodType;

use super::calculation::ArcCalculation;

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
