use super::{PAR_CENTER_ALIGNED, PAR_EDGE_ALIGNED, PER_EDGE_ALIGNED, SURFACE_SYMBOLS};
use std::vec;

use super::super::openfire_runner::SYMBOLS;
use crate::appendix_a::equation_a1;
use crate::appendix_a::equation_a2;
use crate::appendix_a::equation_a3;
use crate::appendix_a::equation_a4;
use crate::appendix_a::equation_a5;
use framework::method::calculation::CalculationComponent;
use framework::method::parameter::object::Object;
use framework::method::parameter::{ArcParameter, ParameterTrait};

pub fn process() -> Vec<Vec<CalculationComponent>> {
    let cond_1 = format!(
        "\\text{{if: {}}} = \\text{{{}}}",
        SURFACE_SYMBOLS.alignment, PAR_CENTER_ALIGNED
    );
    let x_1 = equation_a3::x_equation(SURFACE_SYMBOLS.x, SURFACE_SYMBOLS.w, SURFACE_SYMBOLS.s);
    let y_1 = equation_a3::y_equation(SURFACE_SYMBOLS.y, SURFACE_SYMBOLS.h, SURFACE_SYMBOLS.s);
    let phi_1 =
        equation_a3::phi_equation(&SURFACE_SYMBOLS.phi, &SURFACE_SYMBOLS.x, &SURFACE_SYMBOLS.y);

    let cond_2 = format!(
        "\\text{{else if: {}}} = \\text{{{}}}",
        SURFACE_SYMBOLS.alignment, PAR_EDGE_ALIGNED
    );
    let x_2 = equation_a4::x_equation(SURFACE_SYMBOLS.x, SURFACE_SYMBOLS.w, SURFACE_SYMBOLS.s);
    let y_2 = equation_a4::y_equation(SURFACE_SYMBOLS.y, SURFACE_SYMBOLS.h, SURFACE_SYMBOLS.s);
    let phi_2 =
        equation_a4::phi_equation(&SURFACE_SYMBOLS.phi, &SURFACE_SYMBOLS.x, &SURFACE_SYMBOLS.y);

    let cond_3 = format!(
        "\\text{{else: {}}} = \\text{{{}}}",
        SURFACE_SYMBOLS.alignment, PER_EDGE_ALIGNED
    );
    let x_3 = equation_a5::x_equation(SURFACE_SYMBOLS.x, SURFACE_SYMBOLS.w, SURFACE_SYMBOLS.s);
    let y_3 = equation_a5::y_equation(SURFACE_SYMBOLS.y, SURFACE_SYMBOLS.h, SURFACE_SYMBOLS.s);
    let phi_3 =
        equation_a5::phi_equation(&SURFACE_SYMBOLS.phi, &SURFACE_SYMBOLS.x, &SURFACE_SYMBOLS.y);

    let radiation_equation = equation_a1::radiation_intensity_equation(
        &SURFACE_SYMBOLS.i_s,
        &SYMBOLS.boltzman,
        &SURFACE_SYMBOLS.emissivity,
        &SURFACE_SYMBOLS.temperature,
    );
    let incident_radiation_equation = equation_a2::radiation_intensity_at_receiver_equation(
        &SURFACE_SYMBOLS.i_r,
        &SURFACE_SYMBOLS.phi,
        &SURFACE_SYMBOLS.i_s,
    );

    vec![
        vec![CalculationComponent::Equation(cond_1)],
        vec![
            CalculationComponent::Equation(x_1),
            CalculationComponent::Equation(y_1),
        ],
        vec![CalculationComponent::Equation(phi_1)],
        vec![CalculationComponent::Equation(cond_2)],
        vec![
            CalculationComponent::Equation(x_2.clone()),
            CalculationComponent::Equation(y_2.clone()),
        ],
        vec![CalculationComponent::Equation(phi_2)],
        vec![CalculationComponent::Equation(cond_3)],
        vec![
            CalculationComponent::Equation(x_3),
            CalculationComponent::Equation(y_3),
        ],
        vec![CalculationComponent::Equation(phi_3)],
        vec![CalculationComponent::Equation(radiation_equation)],
        vec![CalculationComponent::Equation(incident_radiation_equation)],
    ]
}

pub fn calculation(surfaces: Vec<ArcParameter>) -> Vec<Vec<CalculationComponent>> {
    let calc = surfaces
        .into_iter()
        .map(|surface| {
            let Object(object) = surface.as_object();
            let name = object.get(&SURFACE_SYMBOLS.name);
            let phi = object.get(&SURFACE_SYMBOLS.phi);
            let emissivity = object.get(&SURFACE_SYMBOLS.emissivity);
            let h = object.get(&SURFACE_SYMBOLS.h);
            let w = object.get(&SURFACE_SYMBOLS.w);
            let s = object.get(&SURFACE_SYMBOLS.s);
            let x = object.get(&SURFACE_SYMBOLS.x);
            let y = object.get(&SURFACE_SYMBOLS.y);
            let alignment = object.get(&SURFACE_SYMBOLS.alignment).as_string();

            let temp = object.get(&SURFACE_SYMBOLS.temperature);
            let i_r = object.get(&SURFACE_SYMBOLS.i_r);
            let i_s = object.get(&SURFACE_SYMBOLS.i_s);
            let x_eq;
            let y_eq;
            let phi_eq;

            if alignment == PAR_CENTER_ALIGNED {
                x_eq = equation_a3::x_equation(
                    SURFACE_SYMBOLS.x,
                    &w.display_value(),
                    &s.display_value(),
                );
                y_eq = equation_a3::y_equation(
                    SURFACE_SYMBOLS.y,
                    &h.display_value(),
                    &s.display_value(),
                );

                phi_eq = equation_a3::phi_equation(
                    &SURFACE_SYMBOLS.phi,
                    &x.display_value(),
                    &y.display_value(),
                );
            } else if alignment == PAR_EDGE_ALIGNED {
                x_eq = equation_a4::x_equation(
                    SURFACE_SYMBOLS.x,
                    &w.display_value(),
                    &s.display_value(),
                );
                y_eq = equation_a4::y_equation(
                    SURFACE_SYMBOLS.y,
                    &h.display_value(),
                    &s.display_value(),
                );

                phi_eq = equation_a4::phi_equation(
                    &SURFACE_SYMBOLS.phi,
                    &x.display_value(),
                    &y.display_value(),
                );
            } else {
                x_eq = equation_a5::x_equation(
                    SURFACE_SYMBOLS.x,
                    &w.display_value(),
                    &s.display_value(),
                );
                y_eq = equation_a5::y_equation(
                    SURFACE_SYMBOLS.y,
                    &h.display_value(),
                    &s.display_value(),
                );

                phi_eq = equation_a5::phi_equation(
                    &SURFACE_SYMBOLS.phi,
                    &x.display_value(),
                    &y.display_value(),
                );
            }
            let radiation_equation = equation_a1::radiation_intensity_equation(
                &SURFACE_SYMBOLS.i_s,
                "5.67\\times 10^{-11}",
                &emissivity.display_value(),
                &temp.display_value(),
            );
            let incident_radiation_equation = equation_a2::radiation_intensity_at_receiver_equation(
                &SURFACE_SYMBOLS.i_r,
                &phi.display_value(),
                &i_s.display_value(),
            );

            vec![
                vec![CalculationComponent::H3(name.display_value())],
                vec![CalculationComponent::Text(format!(
                    "Alignment: {}, Therefore:",
                    alignment
                ))],
                vec![
                    CalculationComponent::EquationWithResult(x_eq, x),
                    CalculationComponent::EquationWithResult(y_eq, y),
                ],
                vec![CalculationComponent::EquationWithResult(phi_eq, phi)],
                vec![CalculationComponent::EquationWithResult(
                    radiation_equation,
                    i_s,
                )],
                vec![CalculationComponent::EquationWithResult(
                    incident_radiation_equation,
                    i_r,
                )],
            ]
        })
        .flatten()
        .collect::<Vec<Vec<CalculationComponent>>>();

    calc
}
