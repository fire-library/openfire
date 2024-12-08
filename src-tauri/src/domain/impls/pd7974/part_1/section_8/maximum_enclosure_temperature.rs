use crate::domain::method::builder::MethodBuilderTrait;
use crate::domain::method::calculation::{Calculation, CalculationComponent};
use crate::domain::method::equation::Equation;
use crate::domain::method::form::{Form, FormStep};
use crate::domain::method::parameter::builder::ParameterBuilder;
use crate::domain::method::parameter::ArcParameter;
use crate::domain::method::parameter::ParameterValue;
use crate::domain::method::parameter::Parameters;
use crate::domain::method::parameter::{ParameterTrait, ParametersTrait};
use crate::domain::method::MethodType;
use crate::domain::method::{step::Step, Method};
use pd_7974::part_1::section_8::{equation_41, equation_42};
use std::sync::{Arc, RwLock};

pub struct MaximumEnclosureTemperatureBuilder;

impl MethodBuilderTrait for MaximumEnclosureTemperatureBuilder {
    fn name() -> String {
        "Maximum Enclosure Temperature".to_string()
    }
    fn description() -> Option<String> {
        Some("Calculates the maximum enclosure temperature after flashover".to_string())
    }
    fn quick_calc_compatible() -> bool {
        true
    }
    fn reference() -> Vec<String> {
        vec!["PD7974-1".to_string(), "Section 8.6".to_string()]
    }
    fn parameters() -> Parameters {
        let mut params = Parameters::new();

        let a_t = ParameterBuilder::float("A_t")
            .name("Total interior surface area, less ventilation openings")
            .units("m^2")
            .min_exclusive(0.0)
            .required()
            .build();

        let a_v = ParameterBuilder::float("A_v")
            .name("Area of the vertical ventilation opening")
            .units("m^2")
            .min_exclusive(0.0)
            .required()
            .build();

        let h_v = ParameterBuilder::float("H_v")
            .name("Height of ventilation opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let t_g = ParameterBuilder::float("T_{g(max)}")
            .name("Maximum enclosure temperature")
            .expression(Box::new(MaximumEnclosureTemperature::new(
                a_t.clone(),
                a_v.clone(),
                h_v.clone(),
            )))
            .units("^{o}C")
            .build();

        params.add(a_t);
        params.add(a_v);
        params.add(h_v);
        params.add(t_g);

        return params;
    }
    fn form(params: &Parameters) -> crate::domain::method::form::Form {
        let mut fields = vec![];
        for param in params.values().into_iter() {
            if param.read().unwrap().id == "T_{g(max)}" {
                continue;
            }
            fields.push(param.to_field())
        }
        let step_1 = FormStep {
            name: "Ceiling Jet Correlation Input".to_string(),
            description: "Input required to calculate the heat release rate".to_string(),
            fields: fields,
        };

        Form {
            steps: vec![step_1],
        }
    }

    fn calc_sheet(params: &Parameters) -> crate::domain::method::calculation::ArcCalculation {
        let q = params.get_parameter("T_{g(max)}");
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new()));
        let step = Step {
            name: "Calculate Heat Release Rate from Point of Interest".to_string(),
            parameters: vec![q],
        };
        calc_sheet.write().unwrap().add_step(step);

        calc_sheet
    }

    fn method_type() -> MethodType {
        MethodType::PD7974Part1Section8MaximumEnclosureTemperature
    }
}

pub fn evaluate(method: &mut Method) -> Result<(), String> {
    let a_t = method.parameters.get_parameter("A_t").as_float();
    let a_v = method.parameters.get_parameter("A_v").as_float();
    let h_v = method.parameters.get_parameter("H_v").as_float();

    let temp = method.parameters.get_parameter("T_{g(max)}");

    let omega = equation_42::calculate(a_t, a_v, h_v);
    let t = equation_41::calculate(omega);

    temp.write().unwrap().value = Some(ParameterValue::Float(t));

    return Ok(());
}

#[derive(Debug)]
pub struct MaximumEnclosureTemperature {
    pub a_t: ArcParameter,
    pub a_v: ArcParameter,
    pub h_v: ArcParameter,
}

impl MaximumEnclosureTemperature {
    pub fn new(a_t: ArcParameter, a_v: ArcParameter, h_v: ArcParameter) -> Self {
        Self { a_t, a_v, h_v }
    }
}

impl Equation for MaximumEnclosureTemperature {
    fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!("\\dot{{Q}} =");

        vec![vec![CalculationComponent::Equation(eq_1)]]
    }
    fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!("\\dot{{Q}} =");

        vec![vec![CalculationComponent::Equation(eq_1)]]
    }

    fn dependencies(&self) -> Vec<ArcParameter> {
        vec![self.a_t.clone(), self.a_v.clone(), self.h_v.clone()]
    }
}
