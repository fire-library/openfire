use crate::domain::method::builder::MethodBuilder;
use crate::domain::method::calculation::Calculation;
use crate::domain::method::calculation::CalculationComponent;
use crate::domain::method::equation::Equation;
use crate::domain::method::form::FormStep;
use crate::domain::method::parameter::builder::ParameterBuilder;
use crate::domain::method::parameter::ArcParameter;
use crate::domain::method::parameter::ParameterValue;
use crate::domain::method::parameter::Parameters;
use crate::domain::method::parameter::{ParameterTrait, ParametersTrait};
use crate::domain::method::MethodType;
use crate::domain::method::{step::Step, Method};
use std::sync::{Arc, RwLock};
use std::vec;

pub fn name() -> String {
    "Heat Content of Plume".to_string()
}

pub fn description() -> String {
    "Calculates the heat content of the plume".to_string()
}

pub fn method() -> Method {
    let args = create_params();

    let mut fields = vec![];
    for param in args.values().into_iter() {
        if param.read().unwrap().id == "Q_c" {
            continue;
        }
        fields.push(param.to_field())
    }

    let step_1 = FormStep {
        name: "Calculate Plume Heat Content".to_string(),
        description: "Calculates the heat content of the plume".to_string(),
        fields: fields,
    };

    let q_c = args.get_parameter("Q_c");
    let calc_sheet = Arc::new(RwLock::new(Calculation::new()));
    let step = Step {
        name: "Calculate Plume Heat Content".to_string(),
        parameters: vec![q_c],
    };
    calc_sheet.write().unwrap().add_step(step);

    MethodBuilder::new(name())
        .calc_sheet(calc_sheet)
        .method_type(MethodType::PD7974Part2Section7Equation1)
        .reference(vec!["PD7974-2:2019", "Section 7.1", "Equation 1"])
        .parameters(args)
        .quick_calc_compatible(true)
        .add_form_step(step_1)
        .build()
}

pub fn create_params() -> Parameters {
    let mut params = Parameters::new();
    let x = ParameterBuilder::float("X")
        .name("Fraction Convected by Plume")
        .default_value(ParameterValue::Float(0.0))
        .range(0.0, 1.0)
        .required()
        .build();

    let q_t = ParameterBuilder::float("Q_t")
        .name("Total Heat Release Rate")
        .units("kW")
        .default_value(ParameterValue::Float(0.0))
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

pub fn evaluate(method: &mut Method) -> Result<(), String> {
    let x = method
        .parameters
        .get_parameter("X")
        .read()
        .unwrap()
        .value
        .to_float()
        .unwrap();
    let q_t = method
        .parameters
        .get_parameter("Q_t")
        .read()
        .unwrap()
        .value
        .to_float()
        .unwrap();

    let q_c = method.parameters.get_parameter("Q_c");

    q_c.write().unwrap().value = ParameterValue::Float(x * q_t);

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
        let eq = format!(
            "Q_c = {} \\cdot {}",
            self.x.read().unwrap().value.to_float().unwrap(),
            self.q_t.read().unwrap().value.to_float().unwrap(),
        );

        vec![vec![CalculationComponent::EquationWithResult(eq)]]
    }
}
