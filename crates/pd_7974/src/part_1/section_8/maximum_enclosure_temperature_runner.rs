mod omega;
mod psi;
mod t_g;
mod t_g_max;

pub mod integration_tests;

use omega::Omega;
use psi::Psi;
use t_g::TG;
use t_g_max::TGMax;

use super::{equation_41, equation_42, equation_43, equation_44};
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

use crate::PD7974;
use crate::part_1::Section;
use crate::part_1::section_8::Section8Method;

#[derive(Default)]
pub struct MaximumEnclosureTemperatureBuilder;

impl MethodRunner for MaximumEnclosureTemperatureBuilder {
    fn name(&self) -> String {
        "Maximum enclosure temperature".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &PD7974::One(Section::Eight(Section8Method::MaximumEnclosureTemperature))
    }
    fn description(&self) -> Option<String> {
        Some("Calculates the maximum enclosure temperature after flashover".to_string())
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::FireScenario, Tag::FireDynamics]
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let omega = params.get("\\Omega");
        let psi = params.get("\\Psi");
        let t_max = params.get("T_{g(max)}");
        let t_g = params.get("T_{g}");

        Some(vec![omega, psi, t_max, t_g])
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

        let m_e = ParamBuilder::float("m_e")
            .name("Equivalent fire load as wood")
            .units("kg")
            .min_exclusive(0.0)
            .build();

        let psi = ParamBuilder::float("\\Psi")
            .name("Non dimensional input Psi")
            .build();

        let omega = ParamBuilder::float("\\Omega")
            .name("Non dimensional input Omega")
            .build();

        let t_g_max = ParamBuilder::float("T_{g(max)}")
            .name("Maximum enclosure temperature")
            .units("^{o}C")
            .build();

        let t_g = ParamBuilder::float("T_{g}")
            .name("Average enclosure temperature")
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
    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let a_t = params.get("A_t");
        let a_v = params.get("A_v");
        let h_v = params.get("H_v");
        let m_e = params.get("m_e");

        let omega = params.get("\\Omega");
        let psi = params.get("\\Psi");
        let t_max = params.get("T_{g(max)}");
        let t_g = params.get("T_{g}");

        let omega_equation = Omega::new_boxed(omega.clone(), a_t.clone(), a_v.clone(), h_v.clone());
        let t_max_equation = TGMax::new_boxed(t_max.clone(), omega.clone());

        let psi_equation = Psi::new_boxed(psi.clone(), a_t.clone(), a_v.clone(), m_e.clone());
        let t_g_equation = TG::new_boxed(t_g.clone(), psi.clone(), t_max.clone());

        let mut step_1 = FormStep::new(
            "Input | Eq. 41",
            "Input required to calculate the maximum enclosure temperature, based on Thomas and Heselden and Law.",
        );
        for param in params.values().into_iter() {
            if param.symbol() == "T_{g(max)}"
                || param.symbol() == "T_{g}"
                || param.symbol() == "\\Psi"
                || param.symbol() == "m_e"
                || param.symbol() == "\\Omega"
            {
                continue;
            }
            step_1.add_field(param.to_field())
        }

        step_1.add_intro();
        step_1.add_equation(t_max_equation.generate_with_symbols()[0][0].clone());
        step_1.add_text("Where:");
        step_1.add_equation(omega_equation.generate_with_symbols()[0][0].clone());

        let mut step_2 = FormStep::new(
            "Input | Eq. 43 (Optional)",
            "Input required to calculate the impact of fire load on the average temperature in the compartment, for low fire loads.",
        );

        step_2.add_field(params.get("m_e").to_field());
        step_2.add_intro();
        step_2.add_equation(t_g_equation.generate_with_symbols()[0][0].clone());
        step_2.add_text("Where:");
        step_2.add_equation(psi_equation.generate_with_symbols()[0][0].clone());

        Form::new(vec![step_1, step_2])
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let a_t = params.get("A_t");
        let a_v = params.get("A_v");
        let h_v = params.get("H_v");
        let m_e = params.get("m_e");

        let omega = params.get("\\Omega");
        let psi = params.get("\\Psi");
        let t_max = params.get("T_{g(max)}");
        let t_g = params.get("T_{g}");

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let omega_equation = Omega::new_boxed(omega.clone(), a_t.clone(), a_v.clone(), h_v.clone());
        let t_max_equation = TGMax::new_boxed(t_max.clone(), omega.clone());
        let nomenclatute = vec![
            a_t.clone(),
            a_v.clone(),
            h_v.clone(),
            omega.clone(),
            t_max.clone(),
        ];
        let mut process = omega_equation.generate_with_symbols();
        process.extend(t_max_equation.generate_with_symbols());

        let mut calc = omega_equation.generate_with_values();
        calc.extend(t_max_equation.generate_with_values());

        let step_1 = Step {
            name: "Calculate the maximum enclosure temperature".to_string(),
            nomenclature: nomenclatute,
            input: vec![a_t.clone().into(), a_v.clone().into(), h_v.into()],
            process: process,
            calculation: calc,
            render: true,
        };

        let psi_equation = Psi::new_boxed(psi.clone(), a_t.clone(), a_v.clone(), m_e.clone());
        let t_g_equation = TG::new_boxed(t_g.clone(), psi.clone(), t_max.clone());
        let mut process = psi_equation.generate_with_symbols();
        process.extend(t_g_equation.generate_with_symbols());

        let mut calc = psi_equation.generate_with_values();
        calc.extend(t_g_equation.generate_with_values());
        let step_2 = Step {
            name: "Calculate the average enclosure temperature".to_string(),
            nomenclature: vec![
                a_t.clone(),
                a_v.clone(),
                m_e.clone(),
                psi.clone(),
                t_g.clone(),
                t_max.clone(),
            ],
            input: vec![a_t.into(), a_v.into(), m_e.into(), t_max.into()],
            process: process,
            calculation: calc,
            render: true,
        };
        calc_sheet.write().unwrap().add_step(step_1);
        calc_sheet.write().unwrap().add_step(step_2);

        calc_sheet
    }

    fn tests(&self) -> Option<IntegrationTests> {
        Some(IntegrationTests {
            description: include_str!(
                "./maximum_enclosure_temperature_runner/integration_tests/description.md"
            )
            .to_string(),
            tests: integration_tests::tests(),
        })
    }

    fn evaluate(&self, method: &mut Method) -> Result<(), Vec<ParameterError>> {
        let a_t = method.parameters.get("A_t").as_float();
        let a_v = method.parameters.get("A_v").as_float();
        let h_v = method.parameters.get("H_v").as_float();
        let m_e = method.parameters.get("m_e");

        let omega = method.parameters.get("\\Omega");
        let psi = method.parameters.get("\\Psi");
        let t_max = method.parameters.get("T_{g(max)}");
        let t_g = method.parameters.get("T_{g}");

        let omega_result = equation_42::calculate(a_t, a_v, h_v);
        omega.update(Some(omega_result.to_string()))?;

        let t = equation_41::calculate(omega_result);
        t_max.update(Some(t.to_string()))?;

        if let Some(m_e) = m_e.get_float() {
            let psi_result = equation_44::calculate(m_e, a_v, a_t);
            psi.update(Some(psi_result.to_string()))?;

            let t_g_result = equation_43::calculate(t, psi_result);
            t_g.update(Some(t_g_result.to_string()))?;
        } else {
            psi.update(None)?;
            t_g.update(None)?;
        }

        return Ok(());
    }
}
