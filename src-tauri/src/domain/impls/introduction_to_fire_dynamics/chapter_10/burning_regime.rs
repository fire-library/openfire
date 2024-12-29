pub mod factor;
pub mod regime;

use factor::Factor;
use regime::Regime;

use crate::domain::impls::tag::Tag;
use crate::domain::method::builder::MethodBuilderTrait;
use crate::domain::method::calculation::Calculation;
use crate::domain::method::form::{Form, FormStep};
use crate::domain::method::parameter::builder::ParamBuilder;
use crate::domain::method::parameter::ParameterValue;
use crate::domain::method::parameter::Parameters;
use crate::domain::method::parameter::{ParameterTrait, ParametersTrait};
use crate::domain::method::validation::ParameterError;
use crate::domain::method::{step::Step, Method};
use crate::domain::method::{MethodType, Reference};
use introduction_to_fire_dynamics::chapter_10::equation_10_18;
use std::sync::{Arc, RwLock};

pub struct BurningRegimeBuilder;

use super::super::super::Document;
use super::super::IntroductionToFireDynamicsChapter;
use super::Chapter10Method;

impl MethodBuilderTrait for BurningRegimeBuilder {
    fn name() -> String {
        "Burning Regime".to_string()
    }
    fn tags() -> Vec<Tag> {
        vec![Tag::FireDynamics]
    }
    fn description() -> Option<String> {
        Some("Determines the burning regime of a fire.".to_string())
    }
    fn quick_calc_compatible() -> bool {
        true
    }
    fn reference() -> Reference {
        Reference(Document::IntroductionToFireDynamics(Some(
            IntroductionToFireDynamicsChapter::Ten(Chapter10Method::BurningRegime),
        )))
    }
    fn parameters() -> Parameters {
        let mut params = Parameters::new();

        let rho = ParamBuilder::float("\\rho")
            .name("Density of fuel")
            .units("kg/m^3")
            .min_exclusive(0.0)
            .required()
            .build();

        let a_w = ParamBuilder::float("A_w")
            .name("Area of ventilation opening")
            .units("m^2")
            .min(0.0)
            .required()
            .build();

        let h = ParamBuilder::float("H")
            .name("Height of ventilation opening")
            .units("m")
            .min(0.0)
            .required()
            .build();

        let g = ParamBuilder::float("g")
            .name("Gravitaional acceleration")
            .units("m/s^2")
            .default_value(Some(ParameterValue::Float(9.81)))
            .build();

        let a_f = ParamBuilder::float("A_f")
            .name("Surface area of the fuel")
            .units("m^2")
            .min_exclusive(0.0)
            .required()
            .build();

        let factor = ParamBuilder::float("F")
            .name("Burning regime factor")
            .expression(Factor::new_boxed(
                rho.clone(),
                g.clone(),
                a_w.clone(),
                h.clone(),
                a_f.clone(),
            ))
            .build();

        let regime = ParamBuilder::string("Regime")
            .name("Burning regime")
            .expression(Regime::new_boxed(factor.clone()))
            .build();

        params.add(rho);
        params.add(a_w);
        params.add(h);
        params.add(g);
        params.add(a_f);

        params.add(factor);
        params.add(regime);

        return params;
    }
    fn form(params: &Parameters) -> crate::domain::method::form::Form {
        let mut step_1 = FormStep::new(
            "Input | Eq. 10.18",
            "Input required to calculate the burning regime of the fire, following Harmathy's method.",
        );
        for param in params.values().into_iter() {
            if param.id() == "g" || param.id() == "F" || param.id() == "Regime" {
                continue;
            }
            step_1.add_field(param.to_field())
        }
        let factor = params.get_parameter("F");
        step_1.add_intro();
        step_1.add_title("Equations");
        step_1.add_intro();
        step_1.add_equation(
            factor
                .read()
                .unwrap()
                .expression()
                .as_ref()
                .unwrap()
                .generate_with_symbols()[0][0]
                .clone(),
        );

        Form {
            steps: vec![step_1],
        }
    }

    fn calc_sheet(params: &Parameters) -> crate::domain::method::calculation::ArcCalculation {
        let factor = params.get_parameter("F");
        let regime = params.get_parameter("Regime");

        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new()));
        let step_1 = Step {
            name: "Calculate Burning Rate Factor".to_string(),
            parameters: vec![factor],
        };
        calc_sheet.write().unwrap().add_step(step_1);

        let step_2 = Step {
            name: "Determine Burning Regime".to_string(),
            parameters: vec![regime],
        };
        calc_sheet.write().unwrap().add_step(step_2);

        calc_sheet
    }

    fn method_type() -> MethodType {
        MethodType::IntroductionToFireDynamcicsChapter10BurningRegime
    }
}

pub fn evaluate(method: &mut Method) -> Result<(), ParameterError> {
    let rho = method.parameters.get_parameter("\\rho").as_float();
    let a_w = method.parameters.get_parameter("A_w").as_float();
    let h = method.parameters.get_parameter("H").as_float();
    let g = method.parameters.get_parameter("g").as_float();
    let a_f = method.parameters.get_parameter("A_f").as_float();

    let factor = method.parameters.get_parameter("F");
    let regime = method.parameters.get_parameter("Regime");

    let regime_result = equation_10_18::calculate(rho, g, a_w, h, a_f);
    factor.update(Some(regime_result.to_string()))?;

    if regime_result < 0.235 {
        regime.update(Some("Ventilation Controlled".to_string()))?;
    } else if regime_result > 0.290 {
        regime.update(Some("Fuel Controlled".to_string()))?;
    } else {
        regime.update(Some("Undefined / Crossover".to_string()))?;
    }

    return Ok(());
}
