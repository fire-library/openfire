use crate::domain::method::builder::MethodBuilderTrait;
use crate::domain::method::calculation::Calculation;
use crate::domain::method::calculation::CalculationComponent;
use crate::domain::method::equation::Equation;
use crate::domain::method::form::{Form, FormStep};
use crate::domain::method::parameter::builder::ParameterBuilder;
use crate::domain::method::parameter::ArcParameter;
use crate::domain::method::parameter::ParameterValue;
use crate::domain::method::parameter::Parameters;
use crate::domain::method::parameter::{ParameterTrait, ParametersTrait};
use crate::domain::method::MethodType;
use crate::domain::method::{step::Step, Method};
use std::sync::{Arc, RwLock};
use std::vec;

pub struct PD7974Part2Section7Equation1Builder;

impl MethodBuilderTrait for PD7974Part2Section7Equation1Builder {
    fn name() -> String {
        "Heat Content of Plume".to_string()
    }
    fn description() -> Option<String> {
        Some("Calculates the heat content of the plume".to_string())
    }
    fn quick_calc_compatible() -> bool {
        true
    }
    fn reference() -> Vec<String> {
        vec!["SFPE Handbook".to_string()]
    }
    fn form(params: &Parameters) -> crate::domain::method::form::Form {
        let mut fields = vec![];
        for param in params.values().into_iter() {
            if param.read().unwrap().id == "Q_c" {
                continue;
            }
            fields.push(param.to_field())
        }

        let step_1 = FormStep {
            name: "Calculate Plume Heat Content".to_string(),
            description: "Calculates the heat content of the plume".to_string(),
            fields: fields,
            introduction: vec![],
        };

        Form {
            steps: vec![step_1],
        }
    }
    fn parameters() -> Parameters {
        let mut params = Parameters::new();
        let x = ParameterBuilder::float("X")
            .name("Fraction Convected by Plume")
            .default_value(Some(ParameterValue::Float(0.0)))
            .range(0.0, 1.0)
            .required()
            .build();

        let q_t = ParameterBuilder::float("Q_t")
            .name("Total Heat Release Rate")
            .units("kW")
            .default_value(Some(ParameterValue::Float(0.0)))
            .min(0.0)
            .required()
            .build();

        params.add(x.clone());
        params.add(q_t.clone());

        params.add(
            ParameterBuilder::float("Q_c")
                .name("Fraction Convected by Plume")
                .units("kW")
                .expression(Box::new(PD7974Part2Section7Equation1::new(
                    x.clone(),
                    q_t.clone(),
                )))
                .build(),
        );

        return params;
    }

    fn calc_sheet(params: &Parameters) -> crate::domain::method::calculation::ArcCalculation {
        let q_c = params.get_parameter("Q_c");
        let calc_sheet = Arc::new(RwLock::new(Calculation::new()));
        let step = Step {
            name: "Calculate Plume Heat Content".to_string(),
            parameters: vec![q_c],
        };
        calc_sheet.write().unwrap().add_step(step);

        calc_sheet
    }

    fn method_type() -> MethodType {
        MethodType::PD7974Part2Section7Equation1
    }
}

pub fn evaluate(method: &mut Method) -> Result<(), String> {
    let x = method.parameters.get_parameter("X").as_float();
    let q_t = method.parameters.get_parameter("Q_t").as_float();

    let q_c = method.parameters.get_parameter("Q_c");

    q_c.write().unwrap().value = Some(ParameterValue::Float(x * q_t));

    return Ok(());
}

#[derive(Debug)]
pub struct PD7974Part2Section7Equation1 {
    x: ArcParameter,
    q_t: ArcParameter,
}

impl PD7974Part2Section7Equation1 {
    pub fn new(x: ArcParameter, q_t: ArcParameter) -> Self {
        PD7974Part2Section7Equation1 { x, q_t }
    }
}

impl Equation for PD7974Part2Section7Equation1 {
    fn dependencies(&self) -> Vec<ArcParameter> {
        vec![self.x.clone(), self.q_t.clone()]
    }

    fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq = "Q_c = X \\cdot Q_t".to_string();
        vec![vec![CalculationComponent::EquationWithResult(eq)]]
    }

    fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq = format!("Q_c = {} \\cdot {}", self.x.as_float(), self.q_t.as_float(),);

        vec![vec![CalculationComponent::EquationWithResult(eq)]]
    }
}
