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

use crate::CIBSEGuideE;

use std::sync::{Arc, RwLock};
use std::vec;

struct Symbols {
    v: &'static str,
    m: &'static str,
    t_s: &'static str,
    rho_0: &'static str,
    t_0: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    v: "V",
    m: "m",
    t_s: "T_{s}",
    rho_0: "\\rho_{0}",
    t_0: "T_{0}",
};

#[derive(Default)]
pub struct Chapter10Equation3Runner;

impl MethodRunner for Chapter10Equation3Runner {
    fn name(&self) -> String {
        "Volumetric flow rate from mass flow rate (smoke exhaust)".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterTen(crate::chapter_10::Chapter10Method::Equation10_3)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Ventilation]
    }
    fn description(&self) -> Option<String> {
        Some("Calculates the volumetric flow rate for a smoke exhaust if the mass flow rate is known and, considering temperature dependence of density".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let v = params.get(SYMBOLS.v);

        Some(vec![v])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let v = params.get(SYMBOLS.v);
        let m = params.get(SYMBOLS.m);
        let t_s = params.get(SYMBOLS.t_s);
        let rho_0 = params.get(SYMBOLS.rho_0);
        let t_0 = params.get(SYMBOLS.t_0);

        let mut step_1 = FormStep::new(
            "Input | Eq. 10.3",
            "Input required to calculate the volumetric flow rate from the mass flow rate.",
        );
        step_1.add_field(m.to_field());
        step_1.add_field(t_s.to_field());
        step_1.add_field(rho_0.to_field());
        step_1.add_field(t_0.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(equation_1(
            v.symbol(),
            m.symbol(),
            t_s.symbol(),
            rho_0.symbol(),
            t_0.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let v = ParamBuilder::float(&SYMBOLS.v)
            .name("Volumetric rate of a smoke exhaust")
            .units("m^3/s")
            .build();

        let m = ParamBuilder::float(SYMBOLS.m)
            .name("Mass flow rate of smoke exhaust")
            .units("kg/s")
            .min_exclusive(0.0)
            .required()
            .build();

        let t_s = ParamBuilder::float(SYMBOLS.t_s)
            .name("Absolute temperature of the smoke layer")
            .units("K")
            .min_exclusive(0.0)
            .required()
            .build();

        let rho_0 = ParamBuilder::float(SYMBOLS.rho_0)
            .name("Density of air at ambient temperature")
            .units("kg/m^3")
            .min_exclusive(0.0)
            .required()
            .default_value(Some(ParameterValue::Float(1.2)))
            .build();

        let t_0 = ParamBuilder::float(SYMBOLS.t_0)
            .name("Absolute ambient temperature")
            .units("K")
            .min_exclusive(0.0)
            .default_value(Some(ParameterValue::Float(293.0)))
            .required()
            .build();

        params.add(v);
        params.add(m);
        params.add(t_s);
        params.add(rho_0);
        params.add(t_0);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let v = params.get(SYMBOLS.v);
        let m = params.get(SYMBOLS.m);
        let t_s = params.get(SYMBOLS.t_s);
        let rho_0 = params.get(SYMBOLS.rho_0);
        let t_0 = params.get(SYMBOLS.t_0);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));
        let step_1_deps = vec![m.clone(), t_s.clone(), rho_0.clone(), t_0.clone()];
        let mut nomenclature = step_1_deps.clone();
        nomenclature.push(v.clone());

        let step = Step {
            name: "Volumetric flow rate of a smoke exhaust".to_string(),
            nomenclature: nomenclature,
            input: step_1_deps.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(equation_1(
                v.symbol(),
                m.symbol(),
                t_s.symbol(),
                rho_0.symbol(),
                t_0.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                equation_1(
                    v.symbol(),
                    m.display_value(),
                    t_s.display_value(),
                    rho_0.display_value(),
                    t_0.display_value(),
                ),
                v.clone(),
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
        let v = method.parameters.get(SYMBOLS.v);
        let m = method.parameters.get(SYMBOLS.m).as_float();
        let t_s = method.parameters.get(SYMBOLS.t_s).as_float();
        let rho_0 = method.parameters.get(SYMBOLS.rho_0).as_float();
        let t_0 = method.parameters.get(SYMBOLS.t_0).as_float();

        let result = super::volumetric_flow_rate(m, t_s, rho_0, t_0);
        v.update(Some(result.to_string()))?;

        return Ok(());
    }
}

fn equation_1(v: String, m: String, t_s: String, rho_0: String, t_0: String) -> String {
    format!(
        "{} = \\frac{{{} \\cdot {}}}{{{} \\cdot {}}}",
        v, m, t_s, rho_0, t_0
    )
}
