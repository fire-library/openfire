pub mod integration_tests;

use framework::method::calculation::Calculation;
use framework::method::calculation::CalculationComponent;
use framework::method::form::{Form, FormStep};
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;
use framework::method::parameter::builder::ParamBuilder;
use framework::method::parameters::Parameters;
use framework::method::runner::MethodRunner;
use framework::method::tag::Tag;
use framework::method::test::IntegrationTests;
use framework::method::validation::ParameterError;
use framework::method::{Method, step::Step};

use crate::CIBSEGuideE;

use std::sync::{Arc, RwLock};
use std::vec;

struct Symbols {
    n_c: &'static str,
    p: &'static str,
    a: &'static str,
    s: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    n_c: "N_c",
    p: "p",
    a: "A",
    s: "S",
};

#[derive(Default)]
pub struct Chapter7Equation7Runner;

impl MethodRunner for Chapter7Equation7Runner {
    fn name(&self) -> String {
        "Maximum number of people that can be accomodated within a stairway".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterSeven(crate::chapter_7::Chapter7Method::Equation7_7)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Evacuation]
    }
    fn description(&self) -> Option<String> {
        Some(
            "Maximum number of people that can me accomodated within a stairway at any one time"
                .to_string(),
        )
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let n_c = params.get(SYMBOLS.n_c);

        Some(vec![n_c])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let n_c = params.get(SYMBOLS.n_c);
        let p = params.get(SYMBOLS.p);
        let a = params.get(SYMBOLS.a);
        let s = params.get(SYMBOLS.s);

        let mut step_1 = FormStep::new(
            "Input | Eq. 7.7",
            "Calculate the maximum number of people that can be accommodated within a stairway at any one time",
        );
        step_1.add_field(p.to_field());
        step_1.add_field(a.to_field());
        step_1.add_field(s.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(super::equation(
            n_c.symbol(),
            p.symbol(),
            a.symbol(),
            s.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let n_c = ParamBuilder::integer(SYMBOLS.n_c)
            .name("Maximum number of people within a stairway")
            .units("persons")
            .build();

        let p = ParamBuilder::float(&SYMBOLS.p)
            .name("Maximum occupant density of the stair")
            .units("persons/m^2")
            .min_exclusive(0.0)
            .required()
            .build();

        let a = ParamBuilder::float(&SYMBOLS.a)
            .name("Horizontal area of the stair and landings per storey")
            .units("m^2")
            .min_exclusive(0.0)
            .required()
            .build();

        let s = ParamBuilder::integer(&SYMBOLS.s)
            .name("Number of storeys")
            .min_exclusive(0.0)
            .required()
            .build();

        params.add(n_c);
        params.add(p);
        params.add(a);
        params.add(s);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let n_c = params.get(SYMBOLS.n_c);
        let p = params.get(SYMBOLS.p);
        let a = params.get(SYMBOLS.a);
        let s = params.get(SYMBOLS.s);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let input = vec![p.clone(), a.clone(), s.clone()];
        let mut nomenclature = input.clone();
        nomenclature.push(n_c.clone());

        let step = Step {
            name: "Maximum number of people in stairway  | Eq. 7.7".to_string(),
            nomenclature: nomenclature,
            input: input.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(super::equation(
                n_c.symbol(),
                p.symbol(),
                a.symbol(),
                s.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::equation(
                    n_c.symbol(),
                    p.display_value(),
                    a.display_value(),
                    s.display_value(),
                ),
                n_c.clone(),
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
        let n_c = method.parameters.get(SYMBOLS.n_c);
        let p = method.parameters.get(SYMBOLS.p).as_float();
        let a = method.parameters.get(SYMBOLS.a).as_float();
        let s = method.parameters.get(SYMBOLS.s).as_integer();

        let result = super::maximum_people_in_stair(p, a, s);
        n_c.update(Some(result.to_string()))?;

        return Ok(());
    }
}
