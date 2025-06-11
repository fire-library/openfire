pub mod integration_tests;

use framework::method::calculation::Calculation;
use framework::method::calculation::CalculationComponent;
use framework::method::form::{Form, FormStep};
use framework::method::parameter::builder::ParamBuilder;
use framework::method::parameter::{ArcParameter, ParameterTrait};
use framework::method::parameters::Parameters;
use framework::method::runner::MethodRunner;
use framework::method::tag::Tag;
use framework::method::test::IntegrationTests;
use framework::method::validation::ParameterError;
use framework::method::{Method, step::Step};
use std::sync::{Arc, RwLock};
use std::vec;

use crate::BR187;
use crate::chapter_1::Equation;
use crate::chapter_1::equation_1 as br_187_equation_1;

#[derive(Default)]
pub struct BR187Chapter1Equation1Builder;

struct Symbols {
    a_s: &'static str,
    a: &'static str,
    h: &'static str,
    o: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    a_s: "A_s",
    a: "A",
    h: "H",
    o: "O",
};

impl MethodRunner for BR187Chapter1Equation1Builder {
    fn name(&self) -> String {
        "Ventilation factor".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &BR187::ChapterOne(Equation::One)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Ventilation]
    }
    fn description(&self) -> Option<String> {
        Some("Calculates the Ventilation Factor".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let o = params.get(SYMBOLS.o);
        Some(vec![o])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let a_s = params.get(SYMBOLS.a_s);
        let a = params.get(SYMBOLS.a);
        let h = params.get(SYMBOLS.h);
        let o = params.get(SYMBOLS.o);

        let mut step_1 = FormStep::new(
            "Input | Eq. 1",
            "Input required to calculate the ventilation factor.",
        );
        for param in params.values().into_iter() {
            if param.symbol() == SYMBOLS.o {
                continue;
            }
            step_1.add_field(param.to_field())
        }

        let equation = BR187Chapter1Equation1::new(o.clone(), a_s.clone(), a.clone(), h.clone());
        step_1.add_intro();
        step_1.add_equation(equation.generate_with_symbols()[0][0].clone());

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let a_s = ParamBuilder::float(SYMBOLS.a_s)
            .name("Surface Area of Compartment (less openings and floor)")
            .units("m^{2}")
            .min(0.0)
            .max(100.0)
            .required()
            .build();

        let a = ParamBuilder::float(SYMBOLS.a)
            .name("Area of Ventilation Opening")
            .units("m^{2}")
            .min_exclusive(0.0)
            .required()
            .build();

        let h = ParamBuilder::float(SYMBOLS.h)
            .name("Height of Ventilation Opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let o = ParamBuilder::float(SYMBOLS.o)
            .name("Ventilation Factor")
            .units("m^{-1/2}")
            .build();

        params.add(a_s);
        params.add(a);
        params.add(h);
        params.add(o);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let a_s = params.get(SYMBOLS.a_s);
        let a = params.get(SYMBOLS.a);
        let h = params.get(SYMBOLS.h);
        let o = params.get(SYMBOLS.o);
        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let equation = BR187Chapter1Equation1::new(o.clone(), a_s.clone(), a.clone(), h.clone());
        let step = Step {
            name: "Calculate Ventilation Factor".to_string(),
            nomenclature: vec![a.clone(), a_s.clone(), h.clone(), o.clone()],
            input: vec![a.into(), a_s.into(), h.into()],
            process: equation.generate_with_symbols(),
            calculation: equation.generate_with_values(),
            render: true,
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
        let a_s = method.parameters.get(SYMBOLS.a_s).as_float();
        let a = method.parameters.get(SYMBOLS.a).as_float();
        let h = method.parameters.get(SYMBOLS.h).as_float();

        let o = method.parameters.get(SYMBOLS.o);

        let result = br_187_equation_1::calculate_ventilation_factor(a_s, a, h);
        o.update(Some(result.to_string()))?;

        return Ok(());
    }
}

#[derive(Debug)]
pub struct BR187Chapter1Equation1 {
    o: ArcParameter,
    a_s: ArcParameter,
    a: ArcParameter,
    h: ArcParameter,
}

impl BR187Chapter1Equation1 {
    pub fn new(o: ArcParameter, a_s: ArcParameter, a: ArcParameter, h: ArcParameter) -> Self {
        BR187Chapter1Equation1 { o, a_s, a, h }
    }
}

impl BR187Chapter1Equation1 {
    fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq = "O = \\dfrac{A_s}{A \\cdot \\sqrt{H}}".to_string();
        vec![vec![CalculationComponent::Equation(eq)]]
    }

    fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let a_s = &self.a_s;
        let a = &self.a;
        let h = &self.h;

        let eq = format!(
            "O = {} = {}",
            equation_1(a_s.symbol(), a.symbol(), h.symbol()),
            equation_1(
                a_s.as_float().to_string(),
                a.as_float().to_string(),
                h.as_float().to_string()
            ),
        );

        vec![vec![CalculationComponent::EquationWithResult(
            eq,
            self.o.clone(),
        )]]
    }
}

fn equation_1(a_s: String, a: String, h: String) -> String {
    format!(
        "\\dfrac{{{}}}{{{}}}",
        a_s,
        format!("{} \\cdot \\sqrt{{{}}}", a, h)
    )
}
