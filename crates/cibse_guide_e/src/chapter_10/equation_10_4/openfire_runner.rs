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
    t: &'static str,
    q: &'static str,
}

const SYMBOLS: Symbols = Symbols { t: "t", q: "q" };

#[derive(Default)]
pub struct Chapter10Equation4Runner;

impl MethodRunner for Chapter10Equation4Runner {
    fn name(&self) -> String {
        "Time to burning of skin due to radiant heat".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterTen(crate::chapter_10::Chapter10Method::Equation10_4)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Radiation]
    }
    fn description(&self) -> Option<String> {
        Some("Calculates the time to burning of skin due to radiant heat for heat fluxes above 1.7 kW/m2".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let t = params.get(SYMBOLS.t);

        Some(vec![t])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let t = params.get(SYMBOLS.t);
        let q = params.get(SYMBOLS.q);

        let mut step_1 = FormStep::new(
            "Input | Eq. 10.4",
            "Input required to calculate the time to burning of skin due to radiant heat.",
        );
        step_1.add_field(q.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(equation_1(
            t.symbol(),
            q.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let t = ParamBuilder::float(&SYMBOLS.t)
            .name("Time to burning of skin")
            .units("min")
            .build();

        let q = ParamBuilder::float(SYMBOLS.q)
            .name("Radiant heat flux")
            .units("kW/m2")
            .min_exclusive(0.0)
            .required()
            .build();

        params.add(t);
        params.add(q);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let t = params.get(SYMBOLS.t);
        let q = params.get(SYMBOLS.q);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));
        let step_1_deps = vec![q.clone()];
        let mut nomenclature = step_1_deps.clone();
        nomenclature.push(t.clone());

        let step = Step {
            name: "Time to burning of skin".to_string(),
            nomenclature: nomenclature,
            input: step_1_deps.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(equation_1(
                t.symbol(),
                q.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                equation_1(t.symbol(), q.display_value()),
                t.clone(),
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
        let t = method.parameters.get(SYMBOLS.t);
        let q = method.parameters.get(SYMBOLS.q).as_float();

        let result = super::time_burning_skin(q);
        t.update(Some(result.to_string()))?;

        return Ok(());
    }
}

fn equation_1(t: String, q: String) -> String {
    format!("{} = 1.33 * {}^{{-1.35}}", t, q,)
}
