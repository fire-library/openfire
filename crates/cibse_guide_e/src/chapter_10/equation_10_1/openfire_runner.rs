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
    v_max: &'static str,
    gamma: &'static str,
    d: &'static str,
    t_s: &'static str,
    t_0: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    v_max: "V_{max}",
    gamma: "\\gamma",
    d: "d",
    t_s: "T_{s}",
    t_0: "T_{0}",
};

#[derive(Default)]
pub struct Chapter10Equation1Runner;

impl MethodRunner for Chapter10Equation1Runner {
    fn name(&self) -> String {
        "Maximum volumetric flow rate".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterTen(crate::chapter_10::Chapter10Method::Equation10_1)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Ventilation]
    }
    fn description(&self) -> Option<String> {
        Some("Calculates the maximum volumetric flow rate, without plug-holing, by a single exhaust vent".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let v_max = params.get(SYMBOLS.v_max);

        Some(vec![v_max])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let v_max = params.get(SYMBOLS.v_max);
        let gamma = params.get(SYMBOLS.gamma);
        let d = params.get(SYMBOLS.d);
        let t_s = params.get(SYMBOLS.t_s);
        let t_0 = params.get(SYMBOLS.t_0);

        let mut step_1 = FormStep::new(
            "Input | Eq. 10.1",
            "Input required to calculate maximum volumetric flow rate.",
        );
        step_1.add_field(gamma.to_field());
        step_1.add_field(d.to_field());
        step_1.add_field(t_s.to_field());
        step_1.add_field(t_0.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(equation_1(
            v_max.symbol(),
            gamma.symbol(),
            d.symbol(),
            t_s.symbol(),
            t_0.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let v_max = ParamBuilder::float(&SYMBOLS.v_max)
            .name("Maximum volumetric flow rate")
            .units("m^{3}\\cdot s^{-1}")
            .build();

        let gamma = ParamBuilder::float(SYMBOLS.gamma)
            .name("Exhaust vent factor (see About section)")
            .min_exclusive(0.0)
            .max(1.0)
            .required()
            .build();

        let d = ParamBuilder::float(SYMBOLS.d)
            .name("Depth of smoke layer below the lowest point of exhaust")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let t_s = ParamBuilder::float(SYMBOLS.t_s)
            .name("Temperature of the smoke layer")
            .units("K")
            .min(-273.15)
            .required()
            .build();

        let t_0 = ParamBuilder::float(SYMBOLS.t_0)
            .name("Ambient temperature")
            .units("K")
            .min(-273.15)
            .min_exclusive(0.0)
            .less_than_or_equal_to_parameter(&t_s)
            .required()
            .build();

        params.add(v_max);
        params.add(gamma);
        params.add(d);
        params.add(t_0);
        params.add(t_s);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let v_max = params.get(SYMBOLS.v_max);
        let gamma = params.get(SYMBOLS.gamma);
        let d = params.get(SYMBOLS.d);
        let t_s = params.get(SYMBOLS.t_s);
        let t_0 = params.get(SYMBOLS.t_0);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));
        let step_1_deps = vec![gamma.clone(), d.clone(), t_s.clone(), t_0.clone()];
        let mut nomenclature = step_1_deps.clone();
        nomenclature.push(v_max.clone());

        let step = Step {
            name: "Maximum Volumetric Flow Rate".to_string(),
            nomenclature: nomenclature,
            input: step_1_deps.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(equation_1(
                v_max.symbol(),
                gamma.symbol(),
                d.symbol(),
                t_s.symbol(),
                t_0.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                equation_1(
                    v_max.symbol(),
                    gamma.display_value(),
                    d.display_value(),
                    t_s.display_value(),
                    t_0.display_value(),
                ),
                v_max.clone(),
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
        let v_max = method.parameters.get(SYMBOLS.v_max);
        let gamma = method.parameters.get(SYMBOLS.gamma).as_float();
        let d = method.parameters.get(SYMBOLS.d).as_float();
        let t_s = method.parameters.get(SYMBOLS.t_s).as_float();
        let t_0 = method.parameters.get(SYMBOLS.t_0).as_float();

        let result = super::max_volumetric_flow_rate(gamma, d, t_s, t_0);
        v_max.update(Some(result.to_string()))?;

        return Ok(());
    }
}

fn equation_1(v_max: String, gamma: String, d: String, t_s: String, t_0: String) -> String {
    format!(
        "{} = 4.16\\cdot{}\\cdot {}^{{5/2}}\\left(\\dfrac{{{}-{}}}{{{}}}\\right)^{{1/2}}",
        v_max, gamma, d, t_s, t_0, t_0
    )
}
