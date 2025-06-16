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
    n_in: &'static str,
    w_s: &'static str,
    t: &'static str,
    a: &'static str,
    s: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    n_in: "N_{in_{max}}",
    w_s: "W_s",
    t: "t",
    a: "A",
    s: "S",
};

#[derive(Default)]
pub struct Chapter7Equation8Runner;

impl MethodRunner for Chapter7Equation8Runner {
    fn name(&self) -> String {
        "Exit capacity of a stairway".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterSeven(crate::chapter_7::Chapter7Method::Equation7_8)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Evacuation]
    }
    fn description(&self) -> Option<String> {
        Some(
            "Exit capacity of a stairway, limited primarily by the width of the final exit"
                .to_string(),
        )
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let n_in = params.get(SYMBOLS.n_in);

        Some(vec![n_in])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let n_in = params.get(SYMBOLS.n_in);
        let w_s = params.get(SYMBOLS.w_s);
        let t = params.get(SYMBOLS.t);
        let a = params.get(SYMBOLS.a);
        let s = params.get(SYMBOLS.s);

        let mut step_1 = FormStep::new("Input | Eq. 7.8", "Exit capacity of a stairway");
        step_1.add_field(w_s.to_field());
        step_1.add_field(t.to_field());
        step_1.add_field(a.to_field());
        step_1.add_field(s.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(super::equation(
            n_in.symbol(),
            w_s.symbol(),
            t.symbol(),
            a.symbol(),
            s.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let n_in = ParamBuilder::integer(SYMBOLS.n_in)
            .name("Maximum number of people able to enter the stair within a specified period")
            .units("persons")
            .build();

        let w_s = ParamBuilder::float(&SYMBOLS.w_s)
            .name("Width of the stair")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let t = ParamBuilder::float(&SYMBOLS.t)
            .name("Time available for escape")
            .units("s")
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

        params.add(n_in);
        params.add(w_s);
        params.add(t);
        params.add(a);
        params.add(s);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let n_in = params.get(SYMBOLS.n_in);
        let w_s = params.get(SYMBOLS.w_s);
        let t = params.get(SYMBOLS.t);
        let a = params.get(SYMBOLS.a);
        let s = params.get(SYMBOLS.s);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let input = vec![w_s.clone(), t.clone(), a.clone(), s.clone()];
        let mut nomenclature = input.clone();
        nomenclature.push(n_in.clone());

        let step = Step {
            name: "Exit capacity of a stairway (max. number of people in a specified period)  | Eq. 7.8".to_string(),
            nomenclature: nomenclature,
            input: input.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(super::equation(
                n_in.symbol(),
                w_s.symbol(),
                t.symbol(),
                a.symbol(),
                s.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::equation(
                    n_in.symbol(),
                    w_s.display_value(),
                    t.display_value(),
                    a.display_value(),
                    s.display_value(),
                ),
                n_in.clone(),
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
        let n_in = method.parameters.get(SYMBOLS.n_in);
        let w_s = method.parameters.get(SYMBOLS.w_s).as_float();
        let t = method.parameters.get(SYMBOLS.t).as_float();
        let a = method.parameters.get(SYMBOLS.a).as_float();
        let s = method.parameters.get(SYMBOLS.s).as_integer();

        let result = super::exit_capacity_stair(w_s, t, a, s);
        n_in.update(Some(result.to_string()))?;

        return Ok(());
    }
}
