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
    q_f: &'static str,
    a_vo: &'static str,
    h_o: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    q_f: "Q_f",
    a_vo: "A_{vo}",
    h_o: "h_o",
};

#[derive(Default)]
pub struct Chapter6Equation7Runner;

impl MethodRunner for Chapter6Equation7Runner {
    fn name(&self) -> String {
        "Heat Release Rate required for flashover".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterSix(crate::chapter_6::Chapter6Method::Equation6_7)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::HRR, Tag::FireDynamics]
    }
    fn description(&self) -> Option<String> {
        Some(
            "Heat Release Rate required for flashover. Most simple expression from SFPE Handbook"
                .to_string(),
        )
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let q_f = params.get(SYMBOLS.q_f);

        Some(vec![q_f])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let q_f = params.get(SYMBOLS.q_f);
        let a_vo = params.get(SYMBOLS.a_vo);
        let h_o = params.get(SYMBOLS.h_o);

        let mut step_1 = FormStep::new(
            "Input | Eq. 6.7",
            "Calculate the HRR required for flashover",
        );
        step_1.add_field(a_vo.to_field());
        step_1.add_field(h_o.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(super::equation(
            q_f.symbol(),
            a_vo.symbol(),
            h_o.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let q_f = ParamBuilder::float(&SYMBOLS.q_f)
            .name("HRR required for flashover")
            .units("kW")
            .build();

        let a_vo = ParamBuilder::float(SYMBOLS.a_vo)
            .name("Area of the opening to the compartment")
            .units("m^2")
            .min_exclusive(0.0)
            .required()
            .build();

        let h_o = ParamBuilder::float(SYMBOLS.h_o)
            .name("Height of the opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        params.add(q_f);
        params.add(a_vo);
        params.add(h_o);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let q_f = params.get(SYMBOLS.q_f);
        let a_vo = params.get(SYMBOLS.a_vo);
        let h_o = params.get(SYMBOLS.h_o);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let step = vec![a_vo.clone(), h_o.clone()];
        let mut nomenclature = step.clone();
        nomenclature.push(q_f.clone());

        let step = Step {
            name: "HRR at flashover | Eq. 6.7".to_string(),
            nomenclature: nomenclature,
            input: step.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(super::equation(
                q_f.symbol(),
                a_vo.symbol(),
                h_o.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::equation(q_f.symbol(), a_vo.display_value(), h_o.display_value()),
                q_f.clone(),
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
        let q_f = method.parameters.get(SYMBOLS.q_f);
        let a_vo = method.parameters.get(SYMBOLS.a_vo).as_float();
        let h_o = method.parameters.get(SYMBOLS.h_o).as_float();

        let result = super::heat_release_rate_flashover(a_vo, h_o);
        q_f.update(Some(result.to_string()))?;

        return Ok(());
    }
}
