pub mod integration_tests;

use framework::method::calculation::Calculation;
use framework::method::calculation::CalculationComponent;
use framework::method::form::{Form, FormStep};
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;
use framework::method::parameter::ParameterValue;
use framework::method::parameter::builder::ParamBuilder;
use framework::method::parameters::Parameters;
use framework::method::runner::MethodRunner;
use framework::method::tag::Tag;
use framework::method::test::IntegrationTests;
use framework::method::validation::ParameterError;

use framework::method::{Method, step::Step};
use std::sync::{Arc, RwLock};
use std::vec;

use crate::TR17;
use crate::section_2::Section2Method;
use crate::section_2::equation_1 as tr17_equation1;

struct Symbols {
    q_dot: &'static str,
    rho_a: &'static str,
    c_p: &'static str,
    t_a: &'static str,
    g: &'static str,
    h_e: &'static str,
    q_asterisk: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    q_dot: "\\dot{Q}",
    rho_a: "\\rho_a",
    c_p: "c_p",
    t_a: "T_a",
    g: "g",
    h_e: "H_e",
    q_asterisk: "Q^{*}",
};

#[derive(Default)]
pub struct TR17Section2Equation1Builder;

impl MethodRunner for TR17Section2Equation1Builder {
    fn name(&self) -> String {
        "Non-dimensional HRR (compartment)".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &TR17::SectionTwo(Section2Method::EquationOne)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::HRR]
    }
    fn description(&self) -> Option<String> {
        Some("Calulates the non-dimensional Heat Release Rate".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let q_asterisk = params.get(SYMBOLS.q_asterisk);

        Some(vec![q_asterisk])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let q_asterisk = params.get(SYMBOLS.q_asterisk);
        let q_dot = params.get(SYMBOLS.q_dot);
        let rho_a = params.get(SYMBOLS.rho_a);
        let c_p = params.get(SYMBOLS.c_p);
        let t_a = params.get(SYMBOLS.t_a);
        let g = params.get(SYMBOLS.g);
        let h_e = params.get(SYMBOLS.h_e);

        let mut step_1 = FormStep::new(
            "Input | Eq. 1",
            "Input required to calculate the non-dimensional heat release rate.",
        );
        for param in params.values().into_iter() {
            if param.symbol() == SYMBOLS.q_asterisk {
                continue;
            }
            step_1.add_field(param.to_field())
        }

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(equation_1(
            q_asterisk.symbol(),
            q_dot.symbol(),
            rho_a.symbol(),
            c_p.symbol(),
            t_a.symbol(),
            g.symbol(),
            h_e.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let q_dot = ParamBuilder::float(SYMBOLS.q_dot)
            .name("Heat Release Rate")
            .units("kW")
            .min(0.0)
            .max(1000000.0)
            .required()
            .build();

        let rho_a = ParamBuilder::float(SYMBOLS.rho_a)
            .name("Density of air at ambient temperature")
            .units("kg/m^{3}")
            .min_exclusive(0.0)
            .max(100.0)
            .required()
            .default_value(Some(ParameterValue::Float(1.2)))
            .build();

        let c_p = ParamBuilder::float(SYMBOLS.c_p)
            .name("Heat capacity of air at ambient temperature")
            .units("kJ/kgK")
            .min_exclusive(0.0)
            .max(100.0)
            .required()
            .default_value(Some(ParameterValue::Float(1.0)))
            .build();

        let t_a = ParamBuilder::float(SYMBOLS.t_a)
            .name("Ambient temperature")
            .units("K")
            .min(0.0)
            .max(1000.0)
            .required()
            .default_value(Some(ParameterValue::Float(293.0)))
            .build();

        let g = ParamBuilder::float(SYMBOLS.g)
            .name("Gravity acceleration")
            .units("m/s^{2}")
            .min_exclusive(0.0)
            .max(100.0)
            .required()
            .default_value(Some(ParameterValue::Float(9.8)))
            .build();

        let h_e = ParamBuilder::float(SYMBOLS.h_e)
            .name("Enclosure height")
            .units("m")
            .min_exclusive(0.0)
            .max(10000.0)
            .required()
            .build();

        let q_asterisk = ParamBuilder::float(SYMBOLS.q_asterisk)
            .name("Non-dimensional Heat Release Rate")
            .build();

        params.add(q_dot);
        params.add(rho_a);
        params.add(c_p);
        params.add(t_a);
        params.add(g);
        params.add(h_e);
        params.add(q_asterisk);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let q_asterisk = params.get(SYMBOLS.q_asterisk);
        let q_dot = params.get(SYMBOLS.q_dot);
        let rho_a = params.get(SYMBOLS.rho_a);
        let c_p = params.get(SYMBOLS.c_p);
        let t_a = params.get(SYMBOLS.t_a);
        let g = params.get(SYMBOLS.g);
        let h_e = params.get(SYMBOLS.h_e);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));
        let step_1_deps = vec![
            q_dot.clone(),
            rho_a.clone(),
            c_p.clone(),
            t_a.clone(),
            g.clone(),
            h_e.clone(),
        ];
        let mut nomenclature = step_1_deps.clone();
        nomenclature.push(q_asterisk.clone());

        let step = Step {
            name: "Non-dimensional Heat Release Rate".to_string(),
            nomenclature: nomenclature,
            input: step_1_deps.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(equation_1(
                q_asterisk.symbol(),
                q_dot.symbol(),
                rho_a.symbol(),
                c_p.symbol(),
                t_a.symbol(),
                g.symbol(),
                h_e.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                equation_1(
                    q_asterisk.symbol(),
                    q_dot.display_value(),
                    rho_a.display_value(),
                    c_p.display_value(),
                    t_a.display_value(),
                    g.display_value(),
                    h_e.display_value(),
                ),
                q_asterisk.clone(),
            )]],
        };
        calc_sheet.write().unwrap().add_step(step);

        calc_sheet
    }

    fn tests(&self) -> Option<IntegrationTests> {
        Some(IntegrationTests {
            description: include_str!("./openfire_runner/integration_tests/description.md")
                .to_string(),
            tests: integration_tests::tests(),
        })
    }

    fn evaluate(&self, method: &mut Method) -> Result<(), Vec<ParameterError>> {
        let q_dot = method.parameters.get(&SYMBOLS.q_dot).as_float();
        let rho_a = method.parameters.get(&SYMBOLS.rho_a).as_float();
        let c_p = method.parameters.get(&SYMBOLS.c_p).as_float();
        let t_a = method.parameters.get(&SYMBOLS.t_a).as_float();
        let g = method.parameters.get(&SYMBOLS.g).as_float();
        let h_e = method.parameters.get(&SYMBOLS.h_e).as_float();

        let q_asterisk = method.parameters.get(&SYMBOLS.q_asterisk);

        let result = tr17_equation1::calculate_nondime_hrr(q_dot, rho_a, c_p, t_a, g, h_e);
        q_asterisk.update(Some(result.to_string()))?;

        return Ok(());
    }
}

fn equation_1(
    q_star: String,
    q_dot: String,
    rho_a: String,
    c_p: String,
    t_a: String,
    g: String,
    h_e: String,
) -> String {
    format!(
        "{} = \\dfrac{{{}}}{{{}}}",
        q_star,
        q_dot,
        format!(
            "{} \\cdot {}  \\cdot {} \\sqrt{{{}}} \\cdot {}^ \\frac{{{}}}{{{}}}",
            rho_a, c_p, t_a, g, h_e, 5.0, 2.0,
        )
    )
}
