pub mod integration_tests;

use framework::method::calculation::Calculation;
use framework::method::calculation::CalculationComponent;
use framework::method::form::{Form, FormStep};
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;
use framework::method::parameter::Parameters;
use framework::method::parameter::builder::ParamBuilder;
use framework::method::runner::MethodRunner;
use framework::method::tag::Tag;
use framework::method::test::IntegrationTests;
use framework::method::validation::ParameterError;
use framework::method::{Method, step::Step};

use crate::CIBSEGuideE;

use std::sync::{Arc, RwLock};
use std::vec;

struct Symbols {
    z_f: &'static str,
    q_t: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    z_f: "z_{f}",
    q_t: "Q_{t}",
};

#[derive(Default)]
pub struct Chapter6Equation55Runner;

impl MethodRunner for Chapter6Equation55Runner {
    fn name(&self) -> String {
        "Mean flame height of luminous flames".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterSix(crate::chapter_6::Chapter6Method::Equation6_55)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::HRR, Tag::FireDynamics]
    }
    fn description(&self) -> Option<String> {
        Some("Mean flame height of luminous flames for most fires away from walls".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let z_f = params.get(SYMBOLS.z_f);
        Some(vec![z_f])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let z_f = params.get(SYMBOLS.z_f);
        let q_t = params.get(SYMBOLS.q_t);

        let mut step_1 = FormStep::new(
            "Input | Eq. 6.55",
            "Calculate the mean flame height of luminous flames",
        );
        step_1.add_field(q_t.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(super::equation(
            z_f.symbol(),
            q_t.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let z_f = ParamBuilder::float(SYMBOLS.z_f)
            .name("Mean flame height")
            .units("m")
            .build();

        let q_t = ParamBuilder::float(SYMBOLS.q_t)
            .name("Total heat output of the fire")
            .units("kW")
            .min_exclusive(0.0)
            .required()
            .build();

        params.add(z_f);
        params.add(q_t);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let z_f = params.get(SYMBOLS.z_f);
        let q_t = params.get(SYMBOLS.q_t);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let input = vec![q_t.clone()];
        let nomenclature = input.clone();

        let step = Step {
            name: "Height of flame | Eq. 6.55".to_string(),
            nomenclature: nomenclature,
            input: input.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(super::equation(
                z_f.symbol(),
                q_t.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::equation(z_f.symbol(), q_t.display_value()),
                z_f.clone(),
            )]],
        };
        calc_sheet.write().unwrap().add_step(step);

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
        let z_f = method.parameters.get(SYMBOLS.z_f);
        let q_t = method.parameters.get(SYMBOLS.q_t).as_float();

        let result = super::mean_flame_height(q_t);
        z_f.update(Some(result.to_string()))?;

        Ok(())
    }
}
