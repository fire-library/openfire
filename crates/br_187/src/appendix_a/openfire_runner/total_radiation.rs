use super::SURFACE_SYMBOLS;
use std::vec;

use super::super::openfire_runner::SYMBOLS;
use framework::method::calculation::CalculationComponent;
use framework::method::parameter::object::Object;
use framework::method::parameter::{ArcParameter, ParameterTrait};

pub fn process() -> Vec<Vec<CalculationComponent>> {
    let sum = format!(
        "{} = \\displaystyle{{\\sum_{{\\mathclap{{i \\in \\text{{{}}}}}}} I_{{ri}}}}",
        SYMBOLS.total_radiation, SYMBOLS.surfaces
    );

    vec![vec![CalculationComponent::Equation(sum)]]
}

pub fn calculation(
    total_radiation: ArcParameter,
    surfaces: Vec<ArcParameter>,
) -> Vec<Vec<CalculationComponent>> {
    let rad = surfaces
        .into_iter()
        .map(|surface| {
            let Object(object) = surface.as_object();
            let is_negative = object.get(&SURFACE_SYMBOLS.i_r).as_float();
            let value = object.get(&SURFACE_SYMBOLS.i_r).display_value();
            if is_negative < 0.0 {
                format!("({})", value)
            } else {
                value
            }
        })
        .collect::<Vec<String>>();

    let sum = rad.join(" + ");

    let calc = vec![vec![CalculationComponent::EquationWithResult(
        format!("{} = {}", SYMBOLS.total_radiation, sum),
        total_radiation,
    )]];

    calc
}
