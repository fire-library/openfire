mod q_max_fc;
mod q_max_vc;

use q_max_fc::QMaxFC;
use q_max_vc::QMaxVC;

pub mod integration_tests;

use super::{equation_4, equation_33};
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
pub struct MaximumHRRBuilder;

use crate::PD7974;
use crate::part_1::Section;
use crate::part_1::section_8::Section8Method;

impl MethodRunner for MaximumHRRBuilder {
    fn name(&self) -> String {
        "Maximum HRR in a compartment fire".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &PD7974::One(Section::Eight(Section8Method::MaximumHRR))
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::HRR, Tag::FireDynamics]
    }
    fn description(&self) -> Option<String> {
        Some("Calculates the maximum HRR for a compartment fire, comparing fuel-controlled and ventilation-controlled conditions".to_string())
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let q_max_vc = ParamBuilder::float("\\dot{Q}_{max, \\space VC}")
            .name("Max HRR for ventilation-controlled scenario")
            .units("kW")
            .build();

        let q_max_fc = ParamBuilder::float("\\dot{Q}_{max, \\space FC}")
            .name("Max HRR for fuel-controlled scenario")
            .units("kW")
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

        let hrrpua = ParamBuilder::float("HRRPUA")
            .name("Heat Release Rate Per Unit Area")
            .units("kW/m^{2} ")
            .min(0.0)
            .build();

        let a_f = ParamBuilder::float("A_f")
            .name("Area of the fire")
            .units("m^2")
            .build();

        params.add(q_max_vc);
        params.add(q_max_fc);
        params.add(a_v);
        params.add(h_v);
        params.add(hrrpua);
        params.add(a_f);

        return params;
    }

    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let q_max_vc = params.get("\\dot{Q}_{max, \\space FC}");
        let q_max_fc = params.get("\\dot{Q}_{max, \\space VC}");

        Some(vec![q_max_vc, q_max_fc])
    }
    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let a_v = params.get("A_v");
        let h_v = params.get("H_v");
        let a_f = params.get("A_f");
        let hrrpua = params.get("HRRPUA");
        let q_max_vc = params.get("\\dot{Q}_{max, \\space VC}");
        let q_max_fc = params.get("\\dot{Q}_{max, \\space FC}");

        let mut step_1 = FormStep::new(
            "Input | Eq. 33",
            "Calculate the max HRR of a ventilation-controlled fire, based on Kawagoe",
        );

        let equation = QMaxVC::new_boxed(q_max_vc.clone(), a_v.clone(), h_v.clone());
        step_1.add_intro();
        step_1.add_equation(equation.generate_with_symbols()[0][0].clone());

        for param in params.values().into_iter() {
            if param.symbol() == "\\dot{Q}_{max, \\space VC}"
                || param.symbol() == "HRRPUA"
                || param.symbol() == "A_f"
                || param.symbol() == "\\dot{Q}_{max, \\space FC}"
            {
                continue;
            }
            step_1.add_field(param.to_field())
        }

        let mut step_2 = FormStep::new(
            "Input | Eq. 4 (Optional)",
            "Calculate the max HRR of a fuel-controlled fire",
        );

        let equation = QMaxFC::new_boxed(q_max_fc.clone(), a_f.clone(), hrrpua.clone());

        step_2.add_intro();
        step_2.add_equation(equation.generate_with_symbols()[0][0].clone());
        step_2.add_field(a_f.to_field());
        step_2.add_field(hrrpua.to_field());

        Form::new(vec![step_1, step_2])
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let a_v = params.get("A_v");
        let h_v = params.get("H_v");
        let a_f = params.get("A_f");
        let hrrpua = params.get("HRRPUA");
        let q_max_vc = params.get("\\dot{Q}_{max, \\space VC}");
        let q_max_fc = params.get("\\dot{Q}_{max, \\space FC}");

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));
        let equation = QMaxVC::new_boxed(q_max_vc.clone(), a_v.clone(), h_v.clone());

        let step_1 = Step {
            name: "Max HRR for ventilation-controlled fire".to_string(),
            nomenclature: equation.dependencies(),
            input: equation.input().into_iter().map(|p| p.into()).collect(),
            process: equation.generate_with_symbols(),
            calculation: equation.generate_with_values(),
            render: true,
        };
        calc_sheet.write().unwrap().add_step(step_1);

        let equation = QMaxFC::new_boxed(q_max_fc.clone(), a_f.clone(), hrrpua.clone());
        let step_2 = Step {
            name: "Max HRR for fuel-controlled fire".to_string(),
            nomenclature: equation.dependencies(),
            input: equation.input().into_iter().map(|p| p.into()).collect(),
            process: equation.generate_with_symbols(),
            calculation: equation.generate_with_values(),
            render: true,
        };
        calc_sheet.write().unwrap().add_step(step_2);

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
        let a_v = method.parameters.get("A_v").as_float();
        let h_v = method.parameters.get("H_v").as_float();

        let q_max_vc = method.parameters.get("\\dot{Q}_{max, \\space VC}");
        let q_max_fc = method.parameters.get("\\dot{Q}_{max, \\space FC}");

        let q_max_vc_result = equation_33::q_max_vc(a_v, h_v);
        q_max_vc.update(Some(q_max_vc_result.to_string()))?;

        let a_f = method.parameters.get("A_f");
        let hrrpua = method.parameters.get("HRRPUA");

        if let Some(a_f) = a_f.get_float() {
            if let Some(hrrpua) = hrrpua.get_float() {
                let q_max_fc_result = equation_4::q_max_fc(a_f, hrrpua);
                q_max_fc.update(Some(q_max_fc_result.to_string()))?;
            } else {
                q_max_fc.update(None)?;
            }
        } else {
            q_max_fc.update(None)?;
        }

        return Ok(());
    }
}
