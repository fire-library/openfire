mod q_fo_mccaffrey;
mod q_fo_thomas;
mod q_max;

use q_fo_mccaffrey::QFoMcCaffrey;
use q_fo_thomas::QFoThomas;
use q_max::QMax;

use crate::domain::method::builder::MethodBuilderTrait;
use crate::domain::method::calculation::Calculation;
use crate::domain::method::form::{Form, FormStep};
use crate::domain::method::parameter::builder::ParameterBuilder;
use crate::domain::method::parameter::ParameterValue;
use crate::domain::method::parameter::Parameters;
use crate::domain::method::parameter::{ParameterTrait, ParametersTrait};
use crate::domain::method::MethodType;
use crate::domain::method::{step::Step, Method};
use pd_7974::part_1::section_8::{equation_28, equation_29, equation_33};
use std::sync::{Arc, RwLock};

pub struct HRRAtFlashoverBuilder;

impl MethodBuilderTrait for HRRAtFlashoverBuilder {
    fn name() -> String {
        "HRR at Flashover".to_string()
    }
    fn description() -> Option<String> {
        Some("Calculates the HRR at flashover, comparing methods developed by Thomas, McCaffrey et al., and Kawago".to_string())
    }
    fn quick_calc_compatible() -> bool {
        true
    }
    fn reference() -> Vec<String> {
        vec![
            "PD7974-1".to_string(),
            "Section 8.5.1.2 and 8.5.2.2".to_string(),
        ]
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

        let h_k = ParameterBuilder::float("h_k")
            .name("Effective heat transfer coefficient of the enclosure")
            .units("kW m^{-2} K^{-1}")
            .min(0.0)
            .build();

        let q_fo_thomas = ParameterBuilder::float("\\dot{Q}_{fo, \\space Thomas}")
            .name("Heat release rate to cause flashover temperature rise")
            .expression(QFoThomas::new_boxed(a_t.clone(), a_v.clone(), h_v.clone()))
            .units("kW")
            .build();

        let q_fo_mccaffrey = ParameterBuilder::float("\\dot{Q}_{fo, \\space McCaffrey}")
            .name("Heat release rate to cause flashover temperature rise")
            .expression(QFoMcCaffrey::new_boxed(
                a_t.clone(),
                a_v.clone(),
                h_v.clone(),
                h_k.clone(),
            ))
            .units("kW")
            .build();

        let q_max = ParameterBuilder::float("\\dot{Q}_{max}")
            .name("Max HRR of ventilation controlled fire")
            .expression(QMax::new_boxed(a_v.clone(), h_v.clone()))
            .units("kW")
            .build();

        params.add(a_t);
        params.add(a_v);
        params.add(h_v);
        params.add(h_k);
        params.add(q_max);
        params.add(q_fo_thomas);
        params.add(q_fo_mccaffrey);

        return params;
    }
    fn form(params: &Parameters) -> crate::domain::method::form::Form {
        let mut step_1 = FormStep::new(
            "HRR at flashover by Thomas",
            "Input required to calculate the HRR at flashover using Method 1 by Thomas",
        );
        for param in params.values().into_iter() {
            if param.read().unwrap().id == "\\dot{Q}_{fo, \\space Thomas}"
                || param.read().unwrap().id == "\\dot{Q}_{fo, \\space McCaffrey}"
                || param.read().unwrap().id == "h_k"
                || param.read().unwrap().id == "\\dot{Q}_{max}"
            {
                continue;
            }
            step_1.add_field(param.to_field())
        }
        let q_fo_thomas = params.get_parameter("\\dot{Q}_{fo, \\space Thomas}");
        step_1.add_intro();
        step_1.add_equation(
            q_fo_thomas
                .read()
                .unwrap()
                .expression
                .as_ref()
                .unwrap()
                .generate_with_symbols()[0][0]
                .clone(),
        );

        let mut step_2 = FormStep::new(
            "HRR at flashover by McCaffrey et al. (Optional)",
            "Additional input required to calculate the HRR at flashover using Method 2 by McCaffrey et al.",
        );

        let h_k = params.get_parameter("h_k");
        step_2.add_field(h_k.to_field());

        let q_fo_mccaffrey = params.get_parameter("\\dot{Q}_{fo, \\space McCaffrey}");
        step_2.add_intro();
        step_2.add_equation(
            q_fo_mccaffrey
                .read()
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
        let q_fo_thomas = params.get_parameter("\\dot{Q}_{fo, \\space Thomas}");
        let q_fo_mccaffrey = params.get_parameter("\\dot{Q}_{fo, \\space McCaffrey}");
        let q_max = params.get_parameter("\\dot{Q}_{max}");

        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new()));
        let step_1 = Step {
            name: "HRR at flashover by Thomas".to_string(),
            parameters: vec![q_fo_thomas],
        };
        calc_sheet.write().unwrap().add_step(step_1);

        let step_2 = Step {
            name: "HRR at flashover by McCaffrey".to_string(),
            parameters: vec![q_fo_mccaffrey],
        };
        calc_sheet.write().unwrap().add_step(step_2);

        let step_3 = Step {
            name: "Max HRR of ventilation controlled fire".to_string(),
            parameters: vec![q_max],
        };
        calc_sheet.write().unwrap().add_step(step_3);

        calc_sheet
    }

    fn method_type() -> MethodType {
        MethodType::PD7974Part1Section8HRRAtFlashover
    }
}

pub fn evaluate(method: &mut Method) -> Result<(), String> {
    let a_t = method.parameters.get_parameter("A_t").as_float();
    let a_v = method.parameters.get_parameter("A_v").as_float();
    let h_v = method.parameters.get_parameter("H_v").as_float();
    let h_k = method.parameters.get_parameter("h_k");

    let q_fo_thomas = method
        .parameters
        .get_parameter("\\dot{Q}_{fo, \\space Thomas}");

    let q_fo_mccaffrey = method
        .parameters
        .get_parameter("\\dot{Q}_{fo, \\space McCaffrey}");

    let q_max = method.parameters.get_parameter("\\dot{Q}_{max}");

    let thomas_result = equation_28::q_fo(a_t, a_v, h_v);
    q_fo_thomas.write().unwrap().value = Some(ParameterValue::Float(thomas_result));

    let q_max_result = equation_33::q_max(a_v, h_v);
    q_max.write().unwrap().value = Some(ParameterValue::Float(q_max_result));

    if let Some(ParameterValue::Float(h_k)) = h_k.read().unwrap().value {
        let mccaffrey_result = equation_29::q_fo(a_t, a_v, h_v, h_k);
        q_fo_mccaffrey.write().unwrap().value = Some(ParameterValue::Float(mccaffrey_result));
    } else {
        q_fo_mccaffrey.write().unwrap().value = None;
    }

    return Ok(());
}