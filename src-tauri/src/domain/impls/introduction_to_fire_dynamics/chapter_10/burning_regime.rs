pub mod factor;
pub mod regime;

use factor::Factor;
use regime::Regime;

use crate::domain::method::builder::MethodBuilderTrait;
use crate::domain::method::calculation::Calculation;
use crate::domain::method::form::{Form, FormStep};
use crate::domain::method::parameter::builder::ParameterBuilder;
use crate::domain::method::parameter::ParameterValue;
use crate::domain::method::parameter::Parameters;
use crate::domain::method::parameter::{ParameterTrait, ParametersTrait};
use crate::domain::method::MethodType;
use crate::domain::method::{step::Step, Method};
use introduction_to_fire_dynamics::chapter_10::equation_10_18;
use std::sync::{Arc, RwLock};

pub struct BurningRegimeBuilder;

impl MethodBuilderTrait for BurningRegimeBuilder {
    fn name() -> String {
        "Burning Regime".to_string()
    }
    fn description() -> Option<String> {
        Some("Determines the burning regime of a fire.".to_string())
    }
    fn quick_calc_compatible() -> bool {
        true
    }
    fn reference() -> Vec<String> {
        vec![
            "Introduciton to Fire Dynamics".to_string(),
            "Chapter 10".to_string(),
            "Equations 10.18a and 10.18b".to_string(),
        ]
    }
    fn parameters() -> Parameters {
        let mut params = Parameters::new();

        let rho = ParameterBuilder::float("\\rho")
            .name("Density of fuel")
            .units("kg/m^3")
            .min_exclusive(0.0)
            .required()
            .build();

        let a_w = ParameterBuilder::float("A_w")
            .name("Area of ventilation opening")
            .units("m^2")
            .min(0.0)
            .required()
            .build();

        let h = ParameterBuilder::float("H")
            .name("Height of ventilation opening")
            .units("m")
            .min(0.0)
            .required()
            .build();

        let g = ParameterBuilder::float("g")
            .name("Gravitaional acceleration")
            .units("m/s^2")
            .default_value(Some(ParameterValue::Float(9.81)))
            .build();

        let a_f = ParameterBuilder::float("A_f")
            .name("Surface area of the fuel")
            .units("m^2")
            .min_exclusive(0.0)
            .required()
            .build();

        let factor = ParameterBuilder::float("F")
            .name("Burning regime factor")
            .expression(Factor::new_boxed(
                rho.clone(),
                g.clone(),
                a_w.clone(),
                h.clone(),
                a_f.clone(),
            ))
            .build();

        let regime = ParameterBuilder::string("Regime")
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
            "Burning Regime",
            "Input required to calculate the burning regime of the fire.",
        );
        for param in params.values().into_iter() {
            if param.read().unwrap().id == "g"
                || param.read().unwrap().id == "F"
                || param.read().unwrap().id == "Regime"
            {
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
                .expression
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

pub fn evaluate(method: &mut Method) -> Result<(), String> {
    let rho = method.parameters.get_parameter("\\rho").as_float();
    let a_w = method.parameters.get_parameter("A_w").as_float();
    let h = method.parameters.get_parameter("H").as_float();
    let g = method.parameters.get_parameter("g").as_float();
    let a_f = method.parameters.get_parameter("A_f").as_float();

    let factor = method.parameters.get_parameter("F");
    let regime = method.parameters.get_parameter("Regime");

    let regime_result = equation_10_18::calculate(rho, g, a_w, h, a_f);
    factor.write().unwrap().value = Some(ParameterValue::Float(regime_result));

    if regime_result < 0.235 {
        regime.write().unwrap().value =
            Some(ParameterValue::String("Ventilation Controlled".to_string()));
    } else if regime_result > 0.290 {
        regime.write().unwrap().value = Some(ParameterValue::String("Fuel Controlled".to_string()));
    } else {
        regime.write().unwrap().value =
            Some(ParameterValue::String("Undefined / Crossover".to_string()));
    }

    return Ok(());
}
