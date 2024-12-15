mod omega;
mod psi;
mod t_g;
mod t_g_max;

use omega::Omega;
use psi::Psi;
use t_g::TG;
use t_g_max::TGMax;

use crate::domain::method::builder::MethodBuilderTrait;
use crate::domain::method::calculation::Calculation;
use crate::domain::method::form::{Form, FormStep};
use crate::domain::method::parameter::builder::ParameterBuilder;
use crate::domain::method::parameter::ParameterValue;
use crate::domain::method::parameter::Parameters;
use crate::domain::method::parameter::{ParameterTrait, ParametersTrait};
use crate::domain::method::MethodType;
use crate::domain::method::{step::Step, Method};
use pd_7974::part_1::section_8::{equation_41, equation_42, equation_43, equation_44};
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
        vec!["PD7974-1".to_string(), "Section 8.6.1".to_string()]
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

        let m_e = ParameterBuilder::float("m_e")
            .name("Equivalent fire load as wood")
            .units("kg")
            .min_exclusive(0.0)
            .build();

        let psi = ParameterBuilder::float("\\Psi")
            .name("Non dimensional input Psi")
            .expression(Psi::new_boxed(a_t.clone(), a_v.clone(), m_e.clone()))
            .build();

        let omega = ParameterBuilder::float("\\Omega")
            .name("Non dimensional input Omega")
            .expression(Omega::new_boxed(a_t.clone(), a_v.clone(), h_v.clone()))
            .build();

        let t_g_max = ParameterBuilder::float("T_{g(max)}")
            .name("Maximum enclosure temperature")
            .expression(TGMax::new_boxed(omega.clone()))
            .units("^{o}C")
            .build();

        let t_g = ParameterBuilder::float("T_{g}")
            .name("Average enclosure temperature")
            .expression(TG::new_boxed(psi.clone(), t_g_max.clone()))
            .units("^{o}C")
            .build();

        params.add(a_t);
        params.add(a_v);
        params.add(h_v);
        params.add(m_e);
        params.add(psi);
        params.add(omega);
        params.add(t_g_max);
        params.add(t_g);

        return params;
    }
    fn form(params: &Parameters) -> crate::domain::method::form::Form {
        let mut step_1 = FormStep::new(
            "Maximum enclosure temperature",
            "Input required to calculate the maximum enclosure temperature",
        );
        for param in params.values().into_iter() {
            if param.read().unwrap().id == "T_{g(max)}"
                || param.read().unwrap().id == "T_{g}"
                || param.read().unwrap().id == "\\Psi"
                || param.read().unwrap().id == "m_e"
                || param.read().unwrap().id == "\\Omega"
            {
                continue;
            }
            step_1.add_field(param.to_field())
        }
        let omega = params.get_parameter("\\Omega");
        let t_g_max = params.get_parameter("T_{g(max)}");
        step_1.add_intro();
        step_1.add_title("Equations");
        step_1.add_intro();
        step_1.add_equation(
            t_g_max
                .read()
                .unwrap()
                .expression
                .as_ref()
                .unwrap()
                .generate_with_symbols()[0][0]
                .clone(),
        );
        step_1.add_text("Where:");
        step_1.add_equation(
            omega
                .read()
                .unwrap()
                .expression
                .as_ref()
                .unwrap()
                .generate_with_symbols()[0][0]
                .clone(),
        );

        let mut step_2 = FormStep::new(
            "Fire load input (optional)",
            "Calculate the impact of fire load on the average temperature in the compartment",
        );
        let psi = params.get_parameter("\\Psi");
        let t_g = params.get_parameter("T_{g}");
        step_2.add_field(params.get_parameter("m_e").to_field());
        step_2.add_intro();
        step_2.add_title("Equations");
        step_2.add_intro();
        step_2.add_equation(
            t_g.read()
                .unwrap()
                .expression
                .as_ref()
                .unwrap()
                .generate_with_symbols()[0][0]
                .clone(),
        );
        step_2.add_text("Where:");
        step_2.add_equation(
            psi.read()
                .unwrap()
                .expression
                .as_ref()
                .unwrap()
                .generate_with_symbols()[0][0]
                .clone(),
        );

        Form {
            steps: vec![step_1, step_2],
        }
    }

    fn calc_sheet(params: &Parameters) -> crate::domain::method::calculation::ArcCalculation {
        let t_max = params.get_parameter("T_{g(max)}");
        let t_g = params.get_parameter("T_{g}");
        let omega = params.get_parameter("\\Omega");
        let psi = params.get_parameter("\\Psi");
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new()));
        let step_1 = Step {
            name: "Calculate the maximum enclosure temperature".to_string(),
            parameters: vec![omega, t_max],
        };
        let step_2 = Step {
            name: "Calculate the average enclosure temperature".to_string(),
            parameters: vec![psi, t_g],
        };
        calc_sheet.write().unwrap().add_step(step_1);
        calc_sheet.write().unwrap().add_step(step_2);

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
    let m_e = method.parameters.get_parameter("m_e");

    let omega = method.parameters.get_parameter("\\Omega");
    let psi = method.parameters.get_parameter("\\Psi");
    let t_max = method.parameters.get_parameter("T_{g(max)}");
    let t_g = method.parameters.get_parameter("T_{g}");

    let omega_result = equation_42::calculate(a_t, a_v, h_v);
    omega.write().unwrap().value = Some(ParameterValue::Float(omega_result));

    let t = equation_41::calculate(omega_result);
    t_max.write().unwrap().value = Some(ParameterValue::Float(t));

    if let Some(ParameterValue::Float(m_e)) = m_e.read().unwrap().value {
        let psi_result = equation_44::calculate(m_e, a_v, a_t);
        psi.write().unwrap().value = Some(ParameterValue::Float(psi_result));

        let t_g_result = equation_43::calculate(t, psi_result);
        t_g.write().unwrap().value = Some(ParameterValue::Float(t_g_result));
    } else {
        psi.write().unwrap().value = None;
        t_g.write().unwrap().value = None;
    }

    return Ok(());
}
