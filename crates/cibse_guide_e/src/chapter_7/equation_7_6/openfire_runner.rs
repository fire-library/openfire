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
    f_p: &'static str,
    w: &'static str,
}

const SYMBOLS: Symbols = Symbols { f_p: "F_p", w: "w" };

#[derive(Default)]
pub struct Chapter7Equation6Runner;

impl MethodRunner for Chapter7Equation6Runner {
    fn name(&self) -> String {
        "Maximum flow rate of persons through a doorway".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterSeven(crate::chapter_7::Chapter7Method::Equation7_6)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Evacuation]
    }
    fn description(&self) -> Option<String> {
        Some("Maximum flow rate of persons through a doorway or level corridor".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let f_p = params.get(SYMBOLS.f_p);

        Some(vec![f_p])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let f_p = params.get(SYMBOLS.f_p);
        let w = params.get(SYMBOLS.w);

        let mut step_1 = FormStep::new(
            "Input | Eq. 7.8",
            "Calculate the maximum flow rate of persons through a doorway or level corridor",
        );
        step_1.add_field(w.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(super::equation(
            f_p.symbol(),
            w.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let f_p = ParamBuilder::float(SYMBOLS.f_p)
            .name("Maximum flow rate of persons through an opening")
            .units("persons/s")
            .build();

        let w = ParamBuilder::integer(&SYMBOLS.w)
            .name("Width of the opening or corridor")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        params.add(f_p);
        params.add(w);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let f_p = params.get(SYMBOLS.f_p);
        let w = params.get(SYMBOLS.w);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let input = vec![w.clone()];
        let mut nomenclature = input.clone();
        nomenclature.push(f_p.clone());

        let step = Step {
            name: "Maximum flow rate of persons  | Eq. 7.8".to_string(),
            nomenclature: nomenclature,
            input: input.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(super::equation(
                f_p.symbol(),
                w.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::equation(f_p.symbol(), w.display_value()),
                f_p.clone(),
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
        let f_p = method.parameters.get(SYMBOLS.f_p);
        let w = method.parameters.get(SYMBOLS.w).as_float();

        let result = super::maximum_flowrate_persons(w);
        f_p.update(Some(result.to_string()))?;

        return Ok(());
    }
}
