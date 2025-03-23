mod q_fo_mccaffrey;
mod q_fo_thomas;
mod q_max;

use q_fo_mccaffrey::QFoMcCaffrey;
use q_fo_thomas::QFoThomas;
use q_max::QMax;

pub mod integration_tests;

use super::{equation_28, equation_29, equation_33};
use framework::method::calculation::Calculation;
use framework::method::form::{Form, FormStep};
use framework::method::parameter::Parameters;
use framework::method::parameter::builder::ParamBuilder;
use framework::method::parameter::{ArcParameter, ParameterTrait};
use framework::method::runner::MethodRunner;
use framework::method::tag::Tag;
use framework::method::test::IntegrationTests;
use framework::method::validation::ParameterError;
use framework::method::{Method, step::Step};
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct HRRAtFlashoverBuilder;

use crate::PD7974;
use crate::part_1::Section;
use crate::part_1::section_8::Section8Method;

impl MethodRunner for HRRAtFlashoverBuilder {
    fn name(&self) -> String {
        "HRR at flashover".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &PD7974::One(Section::Eight(Section8Method::HRRAtFlashover))
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::HRR, Tag::FireDynamics]
    }
    fn description(&self) -> Option<String> {
        Some("Calculates the HRR at flashover, comparing methods developed by Thomas, McCaffrey et al., and Kawago".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let q_fo_thomas = params.get("\\dot{Q}_{fo, \\space Thomas}");
        let q_fo_mccaffrey = params.get("\\dot{Q}_{fo, \\space McCaffrey}");
        let q_max = params.get("\\dot{Q}_{max, \\space Kawagoe}");

        Some(vec![q_max, q_fo_thomas, q_fo_mccaffrey])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let a_t = ParamBuilder::float("A_t")
            .name("Total interior surface area, less ventilation openings")
            .units("m^2")
            .min_exclusive(0.0)
            .required()
            .build();

        let a_v = ParamBuilder::float("A_v")
            .name("Area of the vertical ventilation opening")
            .units("m^2")
            .min_exclusive(0.0)
            .required()
            .build();

        let h_v = ParamBuilder::float("H_v")
            .name("Height of ventilation opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let h_k = ParamBuilder::float("h_k")
            .name("Effective heat transfer coefficient of the enclosure")
            .units("kW m^{-2} K^{-1}")
            .min(0.0)
            .build();

        let q_fo_thomas = ParamBuilder::float("\\dot{Q}_{fo, \\space Thomas}")
            .name("Heat release rate to cause flashover temperature rise")
            .units("kW")
            .build();

        let q_fo_mccaffrey = ParamBuilder::float("\\dot{Q}_{fo, \\space McCaffrey}")
            .name("Heat release rate to cause flashover temperature rise")
            .units("kW")
            .build();

        let q_max = ParamBuilder::float("\\dot{Q}_{max, \\space Kawagoe}")
            .name("Max HRR of ventilation controlled fire")
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
    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let a_t = params.get("A_t");
        let a_v = params.get("A_v");
        let h_v = params.get("H_v");
        let h_k = params.get("h_k");
        let q_fo_thomas = params.get("\\dot{Q}_{fo, \\space Thomas}");
        let q_fo_mccaffrey = params.get("\\dot{Q}_{fo, \\space McCaffrey}");
        let q_max = params.get("\\dot{Q}_{max, \\space Kawagoe}");

        let mut step_1 = FormStep::new(
            "Input | Eq. 28",
            "Input required to calculate the HRR at flashover using Method 1 by Thomas",
        );
        for param in params.values().into_iter() {
            if param.symbol() == "\\dot{Q}_{fo, \\space Thomas}"
                || param.symbol() == "\\dot{Q}_{fo, \\space McCaffrey}"
                || param.symbol() == "h_k"
                || param.symbol() == "\\dot{Q}_{max, \\space Kawagoe}"
            {
                continue;
            }
            step_1.add_field(param.to_field())
        }
        let equation =
            QFoThomas::new_boxed(q_fo_thomas.clone(), a_t.clone(), a_v.clone(), h_v.clone());
        step_1.add_intro();
        step_1.add_equation(equation.generate_with_symbols()[0][0].clone());

        let mut step_2 = FormStep::new(
            "Input | Eq. 29 (Optional)",
            "Additional input required to calculate the HRR at flashover using Method 2 by McCaffrey et al.",
        );

        let equation = QFoMcCaffrey::new_boxed(
            q_fo_mccaffrey.clone(),
            a_t.clone(),
            a_v.clone(),
            h_v.clone(),
            h_k.clone(),
        );
        step_2.add_field(h_k.to_field());
        step_2.add_intro();
        step_2.add_equation(equation.generate_with_symbols()[0][0].clone());

        let mut step_3 = FormStep::new(
            "Eq. 33",
            "Max HRR of ventilation controlled fire (based on Kawagoe). No additional input required.",
        );
        let equation = QMax::new_boxed(q_max.clone(), a_v.clone(), h_v.clone());
        step_3.add_intro();
        step_3.add_equation(equation.generate_with_symbols()[0][0].clone());

        Form::new(vec![step_1, step_2, step_3])
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let a_t = params.get("A_t");
        let a_v = params.get("A_v");
        let h_v = params.get("H_v");
        let h_k = params.get("h_k");
        let q_fo_thomas = params.get("\\dot{Q}_{fo, \\space Thomas}");
        let q_fo_mccaffrey = params.get("\\dot{Q}_{fo, \\space McCaffrey}");
        let q_max = params.get("\\dot{Q}_{max, \\space Kawagoe}");

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));
        let equation =
            QFoThomas::new_boxed(q_fo_thomas.clone(), a_t.clone(), a_v.clone(), h_v.clone());
        let step_1 = Step {
            name: "HRR at flashover by Thomas (Method 1)".to_string(),
            nomenclature: equation.dependencies(),
            input: equation.input().into_iter().map(|p| p.into()).collect(),
            process: equation.generate_with_symbols(),
            calculation: equation.generate_with_values(),
            render: true,
        };
        calc_sheet.write().unwrap().add_step(step_1);

        let equation = QFoMcCaffrey::new_boxed(
            q_fo_mccaffrey.clone(),
            a_t.clone(),
            a_v.clone(),
            h_v.clone(),
            h_k.clone(),
        );
        let step_2 = Step {
            name: "HRR at flashover by McCaffrey (Method 2)".to_string(),
            nomenclature: equation.dependencies(),
            input: equation.input().into_iter().map(|p| p.into()).collect(),
            process: equation.generate_with_symbols(),
            calculation: equation.generate_with_values(),
            render: true,
        };
        calc_sheet.write().unwrap().add_step(step_2);

        let equation = QMax::new_boxed(q_max.clone(), a_v.clone(), h_v.clone());
        let step_3 = Step {
            name: "Max HRR of ventilation controlled fire (based on Kawagoe)".to_string(),
            nomenclature: equation.dependencies(),
            input: equation.input().into_iter().map(|p| p.into()).collect(),
            process: equation.generate_with_symbols(),
            calculation: equation.generate_with_values(),
            render: true,
        };
        calc_sheet.write().unwrap().add_step(step_3);

        calc_sheet
    }

    fn tests(&self) -> Option<IntegrationTests> {
        Some(IntegrationTests {
            description: include_str!("./hrr_at_flashover_runner/integration_tests/description.md")
                .to_string(),
            tests: integration_tests::tests(),
        })
    }

    fn evaluate(&self, method: &mut Method) -> Result<(), Vec<ParameterError>> {
        let a_t = method.parameters.get("A_t").as_float();
        let a_v = method.parameters.get("A_v").as_float();
        let h_v = method.parameters.get("H_v").as_float();
        let h_k = method.parameters.get("h_k");

        let q_fo_thomas = method.parameters.get("\\dot{Q}_{fo, \\space Thomas}");

        let q_fo_mccaffrey = method.parameters.get("\\dot{Q}_{fo, \\space McCaffrey}");

        let q_max = method.parameters.get("\\dot{Q}_{max, \\space Kawagoe}");

        let thomas_result = equation_28::q_fo(a_t, a_v, h_v);
        q_fo_thomas.update(Some(thomas_result.to_string()))?;

        let q_max_result = equation_33::q_max(a_v, h_v);
        q_max.update(Some(q_max_result.to_string()))?;

        if let Some(h_k) = h_k.get_float() {
            let mccaffrey_result = equation_29::q_fo(h_k, a_t, a_v, h_v);
            q_fo_mccaffrey.update(Some(mccaffrey_result.to_string()))?;
        } else {
            q_fo_mccaffrey.update(None)?;
        }

        return Ok(());
    }
}
