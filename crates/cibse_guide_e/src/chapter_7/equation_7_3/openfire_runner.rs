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
    p: &'static str,
    w: &'static str,
    n: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    p: "P",
    w: "w",
    n: "n",
};

#[derive(Default)]
pub struct Chapter7Equation3Runner;

impl MethodRunner for Chapter7Equation3Runner {
    fn name(&self) -> String {
        "Required width of the stair".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterSeven(crate::chapter_7::Chapter7Method::Equation7_3)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Evacuation]
    }
    fn description(&self) -> Option<String> {
        Some("Required width of the stair for simultaneous evacuation".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let w = params.get(SYMBOLS.w);

        Some(vec![w])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let w = params.get(SYMBOLS.w);
        let p = params.get(SYMBOLS.p);
        let n = params.get(SYMBOLS.n);

        let mut step_1 = FormStep::new(
            "Input | Eq. 7.3",
            "Calculate the required width of the stair",
        );
        step_1.add_field(p.to_field());
        step_1.add_field(n.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(super::equation(
            w.symbol(),
            p.symbol(),
            n.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let w = ParamBuilder::float(SYMBOLS.w)
            .name("Width of the stair")
            .units("m")
            .build();

        let p = ParamBuilder::integer(&SYMBOLS.p)
            .name("Number of people that can be served by a stair")
            .units("persons")
            .min_exclusive(0.0)
            .required()
            .build();

        let n = ParamBuilder::integer(SYMBOLS.n)
            .name("Number of storeys served")
            .min_exclusive(0.0)
            .required()
            .build();

        params.add(w);
        params.add(p);
        params.add(n);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let w = params.get(SYMBOLS.w);
        let p = params.get(SYMBOLS.p);
        let n = params.get(SYMBOLS.n);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let input = vec![p.clone(), n.clone()];
        let mut nomenclature = input.clone();
        nomenclature.push(w.clone());

        let step = Step {
            name: "Required width of the stair  | Eq. 7.3".to_string(),
            nomenclature: nomenclature,
            input: input.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(super::equation(
                w.symbol(),
                p.symbol(),
                n.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::equation(w.symbol(), p.display_value(), n.display_value()),
                w.clone(),
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
        let w = method.parameters.get(SYMBOLS.w);
        let p = method.parameters.get(SYMBOLS.p).as_integer();
        let n = method.parameters.get(SYMBOLS.n).as_integer();

        let result = super::required_width_stair(p, n);
        w.update(Some(result.to_string()))?;

        return Ok(());
    }
}
