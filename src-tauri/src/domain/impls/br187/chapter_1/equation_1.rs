use crate::domain::method::builder::MethodBuilder;
use crate::domain::method::calculation::Calculation;
use crate::domain::method::calculation::CalculationComponent;
use crate::domain::method::equation::Equation;
use crate::domain::method::form::FormStep;
use crate::domain::method::parameter::builder::ParameterBuilder;
use crate::domain::method::parameter::ParameterValue;
use crate::domain::method::parameter::Parameters;
use crate::domain::method::parameter::{ArcParameter, ParameterTrait, ParametersTrait};
use crate::domain::method::MethodType;
use crate::domain::method::{step::Step, Method};
use std::sync::{Arc, RwLock};
use std::vec;

pub fn method() -> Method {
    let args = create_params();

    let mut fields = vec![];
    for param in args.values().into_iter() {
        if param.read().unwrap().id == "O" {
            continue;
        }
        fields.push(param.to_field())
    }

    let step_1 = FormStep {
        name: "Ventilation Factor Input".to_string(),
        description: "Input required to calculate the ventilation factor".to_string(),
        fields: fields,
    };

    let o = args.get_parameter("O");
    let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new()));
    let step = Step {
        name: "Calculate Ventilation Factor".to_string(),
        parameters: vec![o],
    };
    calc_sheet.write().unwrap().add_step(step);

    MethodBuilder::new("BR187 Ventilation Factor".to_string())
        .calc_sheet(calc_sheet)
        .reference(vec!["BR187", "Chapter 1", "Equation 1"])
        .method_type(MethodType::BR187Chapter1Equation1)
        .parameters(args)
        .quick_calc_compatible(true)
        .add_form_step(step_1)
        .build()
}

pub fn create_params() -> Parameters {
    let mut params = Parameters::new();

    let a_s = ParameterBuilder::float("A_s")
        .name("Surface Area of Compartment (less openings and floor)")
        .units("m^{2}")
        .default_value(ParameterValue::Float(0.0))
        .min(0.0)
        .max(100.0)
        .required()
        .build();

    let a = ParameterBuilder::float("A")
        .name("Area of Ventilation Opening")
        .units("m^{2}")
        .default_value(ParameterValue::Float(1.0))
        .min_exclusive(0.0)
        .required()
        .build();

    let h = ParameterBuilder::float("H")
        .name("Height of Ventilation Opening")
        .units("m")
        .default_value(ParameterValue::Float(1.0))
        .min_exclusive(0.0)
        .required()
        .build();

    let o = ParameterBuilder::float("O")
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

pub fn evaluate(method: &mut Method) -> Result<(), String> {
    let a_s = method
        .parameters
        .get_parameter("A_s")
        .read()
        .unwrap()
        .value
        .to_float()
        .unwrap();
    let a = method
        .parameters
        .get_parameter("A")
        .read()
        .unwrap()
        .value
        .to_float()
        .unwrap();
    let h = method
        .parameters
        .get_parameter("H")
        .read()
        .unwrap()
        .value
        .to_float()
        .unwrap();

    let o = method.parameters.get_parameter("O");

    let result = calculate_ventilation_factor(a_s, a, h);
    o.write().unwrap().value = ParameterValue::Float(result);

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
        vec![vec![CalculationComponent::EquationWithResult(eq)]]
    }

    fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let a_s = self.a_s.read().unwrap().value.to_float().unwrap();
        let a = self.a.read().unwrap().value.to_float().unwrap();
        let h = self.h.read().unwrap().value.to_float().unwrap();

        let eq = format!(
            "O = \\frac{{{}}}{{{}}}",
            a_s,
            format!("{} \\cdot \\sqrt{{{}}}", a, h),
        );

        vec![vec![CalculationComponent::EquationWithResult(eq)]]
    }
}
