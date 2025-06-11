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
    v_e: &'static str,
    s_min: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    v_e: "V_{e}",
    s_min: "S_{min}",
};

#[derive(Default)]
pub struct Chapter10Equation2Runner;

impl MethodRunner for Chapter10Equation2Runner {
    fn name(&self) -> String {
        "Minimum separation distance between exhaust vents".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterTen(crate::chapter_10::Chapter10Method::Equation10_2)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Ventilation]
    }
    fn description(&self) -> Option<String> {
        Some("Calculates the miminmum separation distance between exhaust vents to prevent plug-holing".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let s_min = params.get(SYMBOLS.s_min);

        Some(vec![s_min])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let s_min = params.get(SYMBOLS.s_min);
        let v_e = params.get(SYMBOLS.v_e);

        let mut step_1 = FormStep::new(
            "Input | Eq. 10.2",
            "Input required to calculate minimum separation distance between exhaust vents.",
        );
        step_1.add_field(v_e.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(equation_1(
            s_min.symbol(),
            v_e.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let s_min = ParamBuilder::float(&SYMBOLS.s_min)
            .name("Minimum edge-to-edge separation distance between vents")
            .units("m")
            .build();

        let v_e = ParamBuilder::float(SYMBOLS.v_e)
            .name("Volumetric flow rate of one exhaust vent")
            .units("m^3/s")
            .min_exclusive(0.0)
            .required()
            .build();

        params.add(s_min);
        params.add(v_e);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let s_min = params.get(SYMBOLS.s_min);
        let v_e = params.get(SYMBOLS.v_e);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));
        let step_1_deps = vec![v_e.clone()];
        let mut nomenclature = step_1_deps.clone();
        nomenclature.push(s_min.clone());

        let step = Step {
            name: "Minimum separation distance between exhaust vents".to_string(),
            nomenclature: nomenclature,
            input: step_1_deps.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(equation_1(
                s_min.symbol(),
                v_e.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                equation_1(s_min.symbol(), v_e.display_value()),
                s_min.clone(),
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
        let s_min = method.parameters.get(SYMBOLS.s_min);
        let v_e = method.parameters.get(SYMBOLS.v_e).as_float();

        let result = super::min_separation_dist(v_e);
        s_min.update(Some(result.to_string()))?;

        return Ok(());
    }
}

fn equation_1(s_min: String, v_e: String) -> String {
    format!("{} = 0.9 \\cdot {} ^ {{1/2}}", s_min, v_e,)
}
