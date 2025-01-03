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
use super::super::CIBSEEChapter;
use super::ExtractPoints;

pub struct CIBSEEChapter10ExtractPointsBuilder;

impl MethodBuilderTrait for CIBSEEChapter10ExtractPointsBuilder {
    fn name() -> String {
        "Exhaust flow rate".to_string()
    }
    fn tags() -> Vec<Tag> {
        vec![Tag::Ventilation]
    }
    fn description() -> Option<String> {
        Some("Calculates the maximum volumetric flow rate though a single exhaust vent".to_string())
    }
    fn quick_calc_compatible() -> bool {
        true
    }
    fn reference() -> Reference {
        // Note from SS. No idea what R187ter is. Needs updating.
        Reference(Document::CIBSEE(Some(CIBSEEChapter::Ten(R187ter::One(
            Chapter1Equation::One,
        ))))
    }

    fn form(params: &Parameters) -> crate::domain::method::form::Form {
        let mut step_1 = FormStep::new(
            "Input | Eq. 10.1",
            "Input required to calculate the volumetric flow rate.",
        );
        for param in params.values().into_iter() {
            if param.id() == "V" {
                continue;
            }
            step_1.add_field(param.to_field())
        }
        let factor = params.get_parameter("V");
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

        let gamma = ParamBuilder::float("\\Gamma")
            .name("Exhaust location factor (1 for vents far from and, 0.5 for vents close to, the wall)")
            .units("(-)")
            .min(0.5)
            .max(1.0)
            .required()
            .build();

        let d = ParamBuilder::float("d")
            .name("Depth of smoke layer below exhaust")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let t_s = ParamBuilder::float("T_s")
            .name("Temperature of the smoke layer")
            .units("K")
            .min_exclusive(0.0)
            .required()
            .build();

        let t_o = ParamBuilder::float("T_o")
            .name("Ambient temperature")
            .units("K")
            .min_exclusive(0.0)
            .required()
            .build();

        let v = ParamBuilder::float("V")
            .name("Volumetric flow rate")
            .units("m^3/s")
            .expression(Box::new(CIBSEEChapter10ExtractPoints::new(
                gamma.clone(),
                d.clone(),
                t_s.clone(),
                t_o.clone(),
            )))
            .build();

        params.add(gamma);
        params.add(d);
        params.add(t_s);
        params.add(t_o);
        params.add(v);

        return params;
    }

    fn calc_sheet(params: &Parameters) -> crate::domain::method::calculation::ArcCalculation {
        let v = params.get_parameter("V");
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new()));
        let step = Step {
            name: "Calculate maximum volumetric flow rate".to_string(),
            parameters: vec![v],
        };
        calc_sheet.write().unwrap().add_step(step);

        calc_sheet
    }

    fn method_type() -> MethodType {
        MethodType::CIBSEEChapter10ExtractPoints
    }
}

pub fn evaluate(method: &mut Method) -> Result<(), ParameterError> {
    let gamma = method.parameters.get_parameter("\\Gamma").as_float();
    let d = method.parameters.get_parameter("d").as_float();
    let t_s = method.parameters.get_parameter("T_s").as_float();
    let t_o = method.parameters.get_parameter("T_o").as_float();

    let v = method.parameters.get_parameter("V");

    let result = calculate_volumetric_flow_rate(gamma, d, t_s, t_o);
    v.update(Some(result.to_string()))?;

    return Ok(());
}

pub fn calculate_volumetric_flow_rate(gamma: f64, d: f64, t_s: f64, t_o: f64) -> f64 {
    return 4.16 * gamma * d^(5/2) * ((t_s - t_o)/t_o) ^ (1/2);
}

#[derive(Debug)]
pub struct CIBSEEChapter10ExtractPoints {
    gamma: ArcParameter,
    d: ArcParameter,
    t_s: ArcParameter,
    t_o: ArcParameter,
}

impl CIBSEEChapter10ExtractPoints {
    pub fn new(gamma: ArcParameter, d: ArcParameter, t_s: ArcParameter, t_o: ArcParameter) -> Self {
        CIBSEEChapter10ExtractPoints { gamma, d, t_s, t_o }
    }
}

impl Equation for CIBSEEChapter10ExtractPoints {
    fn dependencies(&self) -> Vec<ArcParameter> {
        vec![self.gamma.clone(), self.d.clone(), self.t_s.clone(), self.t_o.clone()]
    }
    // this is another place where I don't kow what is happening below. Why do we define the equation again?
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
