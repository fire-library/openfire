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
    fed: &'static str,
    m_f: &'static str,
    t: &'static str,
    lc_50: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    fed: "FED",
    m_f: "m_{f}",
    t: "t",
    lc_50: "LC_{50}",
};

#[derive(Default)]
pub struct Chapter10Equation8Runner;

impl MethodRunner for Chapter10Equation8Runner {
    fn name(&self) -> String {
        "Calculates the Fractional Effective Dose (FED)".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterTen(crate::chapter_10::Chapter10Method::Equation10_8)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Evacuation]
    }
    fn description(&self) -> Option<String> {
        Some("Calculates the fractional effective dose".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let fed = params.get(SYMBOLS.fed);

        Some(vec![fed])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let fed = params.get(SYMBOLS.fed);
        let m_f = params.get(SYMBOLS.m_f);
        let t = params.get(SYMBOLS.t);
        let lc_50 = params.get(SYMBOLS.lc_50);

        let mut step_1 = FormStep::new(
            "Input | Eq. 10.8",
            "Input required to calculate the Fractional Effective Dose (FED).",
        );
        step_1.add_field(m_f.to_field());
        step_1.add_field(t.to_field());
        step_1.add_field(lc_50.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(equation_1(
            fed.symbol(),
            m_f.symbol(),
            t.symbol(),
            lc_50.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let fed = ParamBuilder::float(&SYMBOLS.fed)
            .name("Fractional Effective Dose (FED)")
            .decimal_places(4)
            .build();

        let m_f = ParamBuilder::float(SYMBOLS.m_f)
            .name("Mass concentration of fuel burned")
            .units("g/m^3")
            .min_exclusive(0.0)
            .required()
            .build();

        let t = ParamBuilder::float(SYMBOLS.t)
            .name("Exposure time")
            .units("min")
            .min_exclusive(0.0)
            .required()
            .build();

        let lc_50 = ParamBuilder::float(SYMBOLS.lc_50)
            .name("Lethal exposure dose from the test subject")
            .units("g/m^3 min")
            .min_exclusive(0.0)
            .required()
            .build();

        params.add(fed);
        params.add(m_f);
        params.add(t);
        params.add(lc_50);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let fed = params.get(SYMBOLS.fed);
        let m_f = params.get(SYMBOLS.m_f);
        let t = params.get(SYMBOLS.t);
        let lc_50 = params.get(SYMBOLS.lc_50);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));
        let step_1_deps = vec![m_f.clone(), t.clone(), lc_50.clone()];
        let mut nomenclature = step_1_deps.clone();
        nomenclature.push(fed.clone());

        let step = Step {
            name: "Fractional Effective Dose".to_string(),
            nomenclature: nomenclature,
            input: step_1_deps.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(equation_1(
                fed.symbol(),
                m_f.symbol(),
                t.symbol(),
                lc_50.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                equation_1(
                    fed.symbol(),
                    m_f.display_value(),
                    t.display_value(),
                    lc_50.display_value(),
                ),
                fed.clone(),
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
        let fed = method.parameters.get(SYMBOLS.fed);
        let m_f = method.parameters.get(SYMBOLS.m_f).as_float();
        let t = method.parameters.get(SYMBOLS.t).as_float();
        let lc_50 = method.parameters.get(SYMBOLS.lc_50).as_float();

        let result = super::fractional_effective_dose(m_f, t, lc_50);
        fed.update(Some(result.to_string()))?;

        return Ok(());
    }
}

fn equation_1(fed: String, m_f: String, t: String, lc_50: String) -> String {
    format!("{} = \\frac{{{} \\cdot {}}}{{{}}}", fed, m_f, t, lc_50,)
}
