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

        let s = ParamBuilder::float(&SYMBOLS.s)
            .name("Visibiility - Distance at which an object an object can be perceived in smoke")
            .units("m")
            .build();

        let k = ParamBuilder::float(SYMBOLS.k)
            .name("Visibility coefficient")
            .units("m^{-1}")
            .min_exclusive(0.0)
            .default_value(Some(ParameterValue::Float(8.0)))
            .required()
            .build();

        let d = ParamBuilder::float(SYMBOLS.d)
            .name("Optical density per unit length")
            .units("m^{-1}")
            .min_exclusive(0.0)
            .required()
            .build();

        params.add(s);
        params.add(k);
        params.add(d);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let s = params.get(SYMBOLS.s);
        let k = params.get(SYMBOLS.k);
        let d = params.get(SYMBOLS.d);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));
        let step_1_deps = vec![k.clone(), d.clone()];
        let mut nomenclature = step_1_deps.clone();
        nomenclature.push(s.clone());

        let step = Step {
            name: "Visibility - Distance at which an object can be perceived in smoke".to_string(),
            nomenclature: nomenclature,
            input: step_1_deps.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(equation_1(
                s.symbol(),
                k.symbol(),
                d.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                equation_1(
                    s.symbol(),
                    k.display_value(),
                    d.display_value(),
                ),
                s.clone(),
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
        let s = method.parameters.get(SYMBOLS.s);
        let k = method.parameters.get(SYMBOLS.k).as_float();
        let d = method.parameters.get(SYMBOLS.d).as_float();

        let result = super::visibility(k, d);
        s.update(Some(result.to_string()))?;

        return Ok(());
    }
}

fn equation_1(fed: String, m_f: String, t: String, lc_50: String) -> String {
    format!(
        "{} = \\frac{{{} \\dot {}}}{{{}}}",
        fed, m_f, t, lc_50,
    )
}
