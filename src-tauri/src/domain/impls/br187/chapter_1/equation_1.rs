use crate::domain::impls::tag::Tag;
use crate::domain::method::builder::MethodBuilderTrait;
use crate::domain::method::calculation::Calculation;
use crate::domain::method::calculation::CalculationComponent;
use crate::domain::method::equation::Equation;
use crate::domain::method::form::{Form, FormStep};
use crate::domain::method::parameter::builder::ParamBuilder;
use crate::domain::method::parameter::Parameters;
use crate::domain::method::parameter::{ArcParameter, ParameterTrait, ParametersTrait};
use crate::domain::method::validation::ParameterError;
use crate::domain::method::MethodType;
use crate::domain::method::Reference;
use crate::domain::method::{step::Step, Method};
use std::sync::{Arc, RwLock};
use std::vec;

use super::super::super::Document;
use super::super::BR187Chapter;
use super::Chapter1Equation;

pub struct BR187Chapter1Equation1Builder;

impl MethodBuilderTrait for BR187Chapter1Equation1Builder {
    fn name() -> String {
        "Ventilation Factor".to_string()
    }
    fn tags() -> Vec<Tag> {
        vec![Tag::Ventilation]
    }
    fn description() -> Option<String> {
        Some("Calculates the Ventilation Factor".to_string())
    }
    fn quick_calc_compatible() -> bool {
        true
    }
    fn reference() -> Reference {
        Reference(Document::BR187(Some(BR187Chapter::One(
            Chapter1Equation::One,
        ))))
    }

    fn form(params: &Parameters) -> crate::domain::method::form::Form {
        let mut step_1 = FormStep::new(
            "Input | Eq. 1",
            "Input required to calculate the ventilation factor.",
        );
        for param in params.values().into_iter() {
            if param.id() == "O" {
                continue;
            }
            step_1.add_field(param.to_field())
        }
        let factor = params.get_parameter("O");
        step_1.add_intro();
        step_1.add_title("Equations");
        step_1.add_intro();
        step_1.add_equation(
            factor
                .read()
                .unwrap()
                .expression()
                .as_ref()
                .unwrap()
                .generate_with_symbols()[0][0]
                .clone(),
        );
        Form {
            steps: vec![step_1],
        }
    }
    fn parameters() -> Parameters {
        let mut params = Parameters::new();

        let a_s = ParamBuilder::float("A_s")
            .name("Surface Area of Compartment (less openings and floor)")
            .units("m^{2}")
            .min(0.0)
            .max(100.0)
            .required()
            .build();

        let a = ParamBuilder::float("A")
            .name("Area of Ventilation Opening")
            .units("m^{2}")
            .min_exclusive(0.0)
            .required()
            .build();

        let h = ParamBuilder::float("H")
            .name("Height of Ventilation Opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let o = ParamBuilder::float("O")
            .name("Ventilation Factor")
            .units("m^{-1/2}")
            .expression(Box::new(BR187Chapter1Equation1::new(
                a_s.clone(),
                a.clone(),
                h.clone(),
            )))
            .build();

        params.add(a_s);
        params.add(a);
        params.add(h);
        params.add(o);

        return params;
    }

    fn calc_sheet(params: &Parameters) -> crate::domain::method::calculation::ArcCalculation {
        let o = params.get_parameter("O");
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new()));
        let step = Step {
            name: "Calculate Ventilation Factor".to_string(),
            parameters: vec![o],
        };
        calc_sheet.write().unwrap().add_step(step);

        calc_sheet
    }

    fn method_type() -> MethodType {
        MethodType::BR187Chapter1Equation1
    }
}

pub fn evaluate(method: &mut Method) -> Result<(), ParameterError> {
    let a_s = method.parameters.get_parameter("A_s").as_float();
    let a = method.parameters.get_parameter("A").as_float();
    let h = method.parameters.get_parameter("H").as_float();

    let o = method.parameters.get_parameter("O");

    let result = calculate_ventilation_factor(a_s, a, h);
    o.update(Some(result.to_string()))?;

    return Ok(());
}

pub fn calculate_ventilation_factor(a_s: f64, a: f64, h: f64) -> f64 {
    return a_s / (a * h.sqrt());
}

#[derive(Debug)]
pub struct BR187Chapter1Equation1 {
    a_s: ArcParameter,
    a: ArcParameter,
    h: ArcParameter,
}

impl BR187Chapter1Equation1 {
    pub fn new(a_s: ArcParameter, a: ArcParameter, h: ArcParameter) -> Self {
        BR187Chapter1Equation1 { a_s, a, h }
    }
}

impl Equation for BR187Chapter1Equation1 {
    fn dependencies(&self) -> Vec<ArcParameter> {
        vec![self.a_s.clone(), self.a.clone(), self.h.clone()]
    }

    fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq = "O = \\frac{A_s}{A \\cdot \\sqrt{H}}".to_string();
        vec![vec![CalculationComponent::Equation(eq)]]
    }

    fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let a_s = &self.a_s;
        let a = &self.a;
        let h = &self.h;

        let eq = format!(
            "O = {} = {}",
            equation_1(a_s.id(), a.id(), h.id()),
            equation_1(
                a_s.as_float().to_string(),
                a.as_float().to_string(),
                h.as_float().to_string()
            ),
        );

        vec![vec![CalculationComponent::EquationWithResult(eq)]]
    }
}

fn equation_1(a_s: String, a: String, h: String) -> String {
    format!(
        "\\frac{{{}}}{{{}}}",
        a_s,
        format!("{} \\cdot \\sqrt{{{}}}", a, h)
    )
}
