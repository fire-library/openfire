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
    v_e: &'static str,
    g: &'static str,
    h: &'static str,
    t_f: &'static str,
    t_0: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    v_e: "v_e",
    g: "g",
    h: "H",
    t_f: "T_f",
    t_0: "T_0{50}",
};

#[derive(Default)]
pub struct Chapter10Equation10Runner;

impl MethodRunner for Chapter10Equation10Runner {
    fn name(&self) -> String {
        "Calculates the limiting average air velocity | Eq. 10.10".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterTen(crate::chapter_10::Chapter10Method::Equation10_10)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Ventilation]
    }
    fn description(&self) -> Option<String> {
        Some("Liminting average air velocity to prevent smoke spread into a large volume where opposed air flow ventilation is used".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let v_e = params.get(SYMBOLS.v_e);

        Some(vec![v_e])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let v_e = params.get(SYMBOLS.v_e);
        let g = params.get(SYMBOLS.g);
        let h = params.get(SYMBOLS.h);
        let t_f = params.get(SYMBOLS.t_f);
        let t_0 = params.get(SYMBOLS.t_0);

        let mut step_1 = FormStep::new(
            "Input | Eq. 10.10",
            "Input required to calculate the limiting average air velocity | Eq. 10.10.",
        );
        step_1.add_field(g.to_field());
        step_1.add_field(h.to_field());
        step_1.add_field(t_f.to_field());
        step_1.add_field(t_0.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(equation_10_10(
            v_e.symbol(),
            g.symbol(),
            h.symbol(),
            t_f.symbol(),
            t_0.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let fed = ParamBuilder::float(&SYMBOLS.fed)
            .name("Fractional Effective Dose (FED)")
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
                m_f.display_value(),
                t.display_value(),
                lc_50.display_value(),
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
        let lc_50= method.parameters.get(SYMBOLS.lc_50).as_float();

        let result = super::fractional_effective_dose(m_f, t, lc_50);
        fed.update(Some(result.to_string()))?;

        return Ok(());
    }
}

fn equation_10_10(v_e: String, g: String, h: String, t_f: String, t_0: String) -> String {
    format!(
        "{} = 0.64 ({} {} \\frac{{{} - {}}}{{{}}}) & (0.5)",
        v_e, g, h, t_f, t_0, t_f,
    )
}
