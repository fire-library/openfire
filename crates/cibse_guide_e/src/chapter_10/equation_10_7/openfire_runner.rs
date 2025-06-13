pub mod integration_tests;

use framework::method::calculation::Calculation;
use framework::method::calculation::CalculationComponent;
use framework::method::form::{Form, FormStep};
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;
use framework::method::parameter::ParameterValue;
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
    s: &'static str,
    k: &'static str,
    d: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    s: "S",
    k: "K",
    d: "D",
};

#[derive(Default)]
pub struct Chapter10Equation7Runner;

impl MethodRunner for Chapter10Equation7Runner {
    fn name(&self) -> String {
        "Distance at which an object can be perceived in smoke".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterTen(crate::chapter_10::Chapter10Method::Equation10_7)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Evacuation]
    }
    fn description(&self) -> Option<String> {
        Some("Calculates the visibility in smoke as a function optical density and a visibility coefficient".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let s = params.get(SYMBOLS.s);

        Some(vec![s])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let s = params.get(SYMBOLS.s);
        let k = params.get(SYMBOLS.k);
        let d = params.get(SYMBOLS.d);

        let mut step_1 = FormStep::new(
            "Input | Eq. 10.7",
            "Input required to calculate the furthest distance at which an object can be perceived in smoke.",
        );
        step_1.add_field(k.to_field());
        step_1.add_field(d.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(equation_1(
            s.symbol(),
            k.symbol(),
            d.symbol(),
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
                equation_1(s.symbol(), k.display_value(), d.display_value()),
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

fn equation_1(s: String, k: String, d: String) -> String {
    format!("{} = \\frac{{{}}}{{2.303 {}}}", s, k, d,)
}
