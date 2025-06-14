pub mod integration_tests;

pub mod factor;
pub mod regime;

use factor::Factor;
use regime::Regime;

use crate::chapter_10::equation_10_18;
use framework::method::calculation::Calculation;
use framework::method::form::{Form, FormStep};
use framework::method::parameter::ParameterValue;
use framework::method::parameter::builder::ParamBuilder;
use framework::method::parameter::{ArcParameter, ParameterTrait};
use framework::method::parameters::Parameters;
use framework::method::runner::MethodRunner;
use framework::method::tag::Tag;
use framework::method::test::IntegrationTests;
use framework::method::validation::ParameterError;
use framework::method::{Method, step::Step};
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct BurningRegimeBuilder;

use crate::IntroductionToFireDynamics;
use crate::chapter_10::Chapter10Method;

struct Symbols {
    rho: &'static str,
    a_w: &'static str,
    h: &'static str,
    g: &'static str,
    a_f: &'static str,
    factor: &'static str,
    regime: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    rho: "\\rho",
    a_w: "A_w",
    h: "H",
    g: "g",
    a_f: "A_f",
    factor: "F",
    regime: "Regime",
};

impl MethodRunner for BurningRegimeBuilder {
    fn name(&self) -> String {
        "Burning regime".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &IntroductionToFireDynamics::ChapterTen(Chapter10Method::BurningRegime)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::FireDynamics]
    }
    fn description(&self) -> Option<String> {
        Some(
            "Determines the burning regime in a compartment fire with wood or wood-based fuels."
                .to_string(),
        )
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let factor = params.get(SYMBOLS.factor);
        let regime = params.get(SYMBOLS.regime);

        Some(vec![factor, regime])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let rho = ParamBuilder::float(SYMBOLS.rho)
            .name("Density of air")
            .units("kg/m^3")
            .default_value(Some(ParameterValue::Float(1.2)))
            .min_exclusive(0.0)
            .required()
            .build();

        let a_w = ParamBuilder::float(SYMBOLS.a_w)
            .name("Area of ventilation opening")
            .units("m^2")
            .min(0.0)
            .required()
            .build();

        let h = ParamBuilder::float(SYMBOLS.h)
            .name("Height of ventilation opening")
            .units("m")
            .min(0.0)
            .required()
            .build();

        let g = ParamBuilder::float(SYMBOLS.g)
            .name("Gravitaional acceleration")
            .units("m/s^2")
            .default_value(Some(ParameterValue::Float(9.81)))
            .required()
            .build();

        let a_f = ParamBuilder::float(SYMBOLS.a_f)
            .name("Surface area of the fuel")
            .units("m^2")
            .min_exclusive(0.0)
            .required()
            .build();

        let factor = ParamBuilder::float(SYMBOLS.factor)
            .name("Burning regime factor")
            .build();

        let regime = ParamBuilder::string(SYMBOLS.regime)
            .name("Burning regime")
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

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let rho = params.get(SYMBOLS.rho);
        let a_w = params.get(SYMBOLS.a_w);
        let h = params.get(SYMBOLS.h);
        let g = params.get(SYMBOLS.g);
        let a_f = params.get(SYMBOLS.a_f);

        let factor = params.get(SYMBOLS.factor);

        let factor_equation = Factor::new_boxed(
            factor.clone(),
            rho.clone(),
            g.clone(),
            a_w.clone(),
            h.clone(),
            a_f.clone(),
        );

        let mut step_1 = FormStep::new(
            "Input | Eq. 10.18",
            "Input required to calculate the burning regime of the fire, following Harmathy's method.",
        );
        for param in params.values().into_iter() {
            if param.symbol() == SYMBOLS.factor || param.symbol() == SYMBOLS.regime {
                continue;
            }
            step_1.add_field(param.to_field())
        }

        step_1.add_intro();
        step_1.add_equation(factor_equation.generate_with_symbols()[0][0].clone());

        Form::new(vec![step_1])
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let rho = params.get(SYMBOLS.rho);
        let a_w = params.get(SYMBOLS.a_w);
        let h = params.get(SYMBOLS.h);
        let g = params.get(SYMBOLS.g);
        let a_f = params.get(SYMBOLS.a_f);

        let factor = params.get(SYMBOLS.factor);
        let regime = params.get(SYMBOLS.regime);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));
        let factor_equation = Factor::new_boxed(
            factor.clone(),
            rho.clone(),
            g.clone(),
            a_w.clone(),
            h.clone(),
            a_f.clone(),
        );
        let step_1 = Step {
            name: "Calculate Burning Rate Factor".to_string(),
            nomenclature: vec![
                a_f.clone(),
                a_w.clone(),
                factor.clone(),
                g.clone(),
                h.clone(),
                rho.clone(),
            ],
            input: vec![
                a_f.clone().into(),
                a_w.clone().into(),
                g.clone().into(),
                h.clone().into(),
                rho.clone().into(),
            ],
            process: factor_equation.generate_with_symbols(),
            calculation: factor_equation.generate_with_values(),
            render: true,
        };
        calc_sheet.write().unwrap().add_step(step_1);

        let regime_equation = Regime::new_boxed(factor.clone());
        let step_2 = Step {
            name: "Determine Burning Regime".to_string(),
            nomenclature: vec![factor.clone(), regime.clone()],
            input: vec![factor.clone().into()],
            process: regime_equation.generate_with_symbols(),
            calculation: regime_equation.generate_with_values(),
            render: true,
        };
        calc_sheet.write().unwrap().add_step(step_2);

        calc_sheet
    }

    fn tests(&self) -> Option<IntegrationTests> {
        Some(IntegrationTests {
            description: include_str!("./openfire_runner/integration_tests/description.md")
                .to_string(),
            tests: integration_tests::tests(),
        })
    }

    fn evaluate(&self, method: &mut Method) -> Result<(), Vec<ParameterError>> {
        let rho = method.parameters.get(SYMBOLS.rho).as_float();
        let a_w = method.parameters.get(SYMBOLS.a_w).as_float();
        let h = method.parameters.get(SYMBOLS.h).as_float();
        let g = method.parameters.get(SYMBOLS.g).as_float();
        let a_f = method.parameters.get(SYMBOLS.a_f).as_float();

        let factor = method.parameters.get(SYMBOLS.factor);
        let regime = method.parameters.get(SYMBOLS.regime);

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
}
