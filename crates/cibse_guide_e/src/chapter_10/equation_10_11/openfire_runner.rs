pub mod integration_tests;

use framework::method::calculation::Calculation;
use framework::method::calculation::CalculationComponent;
use framework::method::form::{Form, FormStep};
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;
use framework::method::parameter::ParameterValue;
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
    q: &'static str,
    z: &'static str,
    v_e: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    q: "Q",
    z: "z",
    v_e: "v_e",
};

#[derive(Default)]
pub struct Chapter10Equation11Runner;

impl MethodRunner for Chapter10Equation11Runner {
    fn name(&self) -> String {
        "Limiting average air velocity for opposed air flow | Fire in large volume to adjoining room".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterTen(crate::chapter_10::Chapter10Method::Equation10_11)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Ventilation]
    }
    fn description(&self) -> Option<String> {
        Some("Limiting average air velocity where opposed air flow is used to stop smoke spread from large space to adjoining small space below the smoke layer interface".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let v_e = params.get(SYMBOLS.v_e);

        Some(vec![v_e])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let v_e = params.get(SYMBOLS.v_e);
        let q = params.get(SYMBOLS.q);
        let z = params.get(SYMBOLS.z);

        let mut step = FormStep::new(
            "Input | Eq. 10.11",
            "Calculate the limiting average air velocity to prevent smoke spread to an adjoining small space.",
        );
        step.add_field(q.to_field());
        step.add_field(z.to_field());

        step.add_intro();
        step.add_equation(CalculationComponent::Equation(super::equation(
            v_e.symbol(),
            q.symbol(),
            z.symbol(),
        )));

        Form::new(vec![step])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let v_e = ParamBuilder::float(&SYMBOLS.v_e)
            .name("Limiting average air velocity")
            .units("m/s")
            .build();

        let q = ParamBuilder::float(SYMBOLS.q)
            .name("Heat Release Rate")
            .units("kW")
            .min_exclusive(0.0)
            .required()
            .build();

        let z = ParamBuilder::float(SYMBOLS.z)
            .name("Distance above the base of the fire to the bottom of the opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        params.add(v_e);
        params.add(q);
        params.add(z);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let v_e = params.get(SYMBOLS.v_e);
        let q = params.get(SYMBOLS.q);
        let z = params.get(SYMBOLS.z);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let step = vec![q.clone(), z.clone()];
        let mut nomenclature = step.clone();
        nomenclature.push(v_e.clone());

        let step = Step {
            name: "Limiting air velocity | Eq. 10.11".to_string(),
            nomenclature: nomenclature,
            input: step.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(super::equation(
                v_e.symbol(),
                q.symbol(),
                z.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::equation(v_e.symbol(), q.display_value(), z.display_value()),
                v_e.clone(),
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
        let v_e = method.parameters.get(SYMBOLS.v_e);
        let q = method.parameters.get(SYMBOLS.q).as_float();
        let z = method.parameters.get(SYMBOLS.z).as_float();

        let result = super::limiting_velocity(q, z);
        v_e.update(Some(result.to_string()))?;
        return Ok(());
    }
}
