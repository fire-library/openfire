pub mod integration_tests;

use framework::method::calculation::Calculation;
use framework::method::calculation::CalculationComponent;
use framework::method::form::{Form, FormStep};
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;
use framework::method::parameter::ParameterValue;
use framework::method::parameter::Parameters;
use framework::method::parameter::builder::ParamBuilder;
use framework::method::runner::MethodRunner;
use framework::method::tag::Tag;
use framework::method::test::IntegrationTests;
use framework::method::validation::ParameterError;
use framework::method::{Method, step::Step};

use crate::CIBSEGuideE;

use std::sync::{Arc, RwLock};
use std::vec;

struct Symbols {
    v_e: &'static str,
    k: &'static str,
    g: &'static str,
    q: &'static str,
    w: &'static str,
    rho: &'static str,
    c: &'static str,
    t: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    v_e: "v_e",
    k: "K",
    g: "g",
    q: "Q",
    w: "w",
    rho: "\\rho",
    c: "c",
    t: "T",
};

#[derive(Default)]
pub struct Chapter10Equation12Runner;

impl MethodRunner for Chapter10Equation12Runner {
    fn name(&self) -> String {
        "Calculates the limiting average air velocity for opposed air flow".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterTen(crate::chapter_10::Chapter10Method::Equation10_12)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Ventilation]
    }
    fn description(&self) -> Option<String> {
        Some("Liminting average air velocity where opposed air flow is used to stop smoke spread from an adjoining room into a corridor".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let v_e = params.get(SYMBOLS.v_e);

        Some(vec![v_e,])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let v_e = params.get(SYMBOLS.v_e);
        let k = params.get(SYMBOLS.k);
        let g = params.get(SYMBOLS.g);
        let q = params.get(SYMBOLS.q);
        let w = params.get(SYMBOLS.w);
        let rho = params.get(SYMBOLS.rho);
        let c = params.get(SYMBOLS.c);
        let t = params.get(SYMBOLS.t);

        let mut step = FormStep::new(
            "Input | Eq. 10.12",
            "Calculate the limiting average air velocity to prevent smoke spread into adjoining corridor.",
        );
        step_1.add_field(g.to_field());
        step_1.add_field(h.to_field());
        step_1.add_field(t_f.to_field());
        step_1.add_field(t_0.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(equation_10_10(
            v_e.symbol(),
            g.symbol(),
            h.symbol(),
            t_f.symbol(),
            t_0.symbol(),
        )));

        Form::new(vec![step])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        // equation 10.10
        let v_e = ParamBuilder::float(&SYMBOLS.v_e)
            .name("Limiting average air velocity")
            .units("m/s")
            .build();

        let g = ParamBuilder::float(SYMBOLS.g)
            .name("Gravity acceleration")
            .units("m/s^2")
            .min_exclusive(0.0)
            .default_value(Some(ParameterValue::Float(9.8)))
            .build();

        let h = ParamBuilder::float(SYMBOLS.h)
            .name("Height of the opening measured from the bottom of the opening")
            .units("m")
            .min_exclusive(0.0)
            .build();

        let t_f = ParamBuilder::float(SYMBOLS.t_f)
            .name("Temperature of the heated smoke")
            .units("K")
            .min_exclusive(0.0)
            .build();

        let t_0 = ParamBuilder::float(SYMBOLS.t_0)
            .name("Temperature of ambient air")
            .units("K")
            .min_exclusive(0.0)
            .default_value(Some(ParameterValue::Float(293.0)))
            .build();

        params.add(v_e);
        params.add(g);
        params.add(h);
        params.add(t_f);
        params.add(t_0);

        // equation 10.11
        let v_e_1011 = ParamBuilder::float(&SYMBOLS.v_e_1011)
            .name("Limiting average air velocity")
            .units("m/s")
            .build();

        let q = ParamBuilder::float(SYMBOLS.q)
            .name("Heat Release Rate")
            .units("kW")
            .min_exclusive(0.0)
            .build();

        let z = ParamBuilder::float(SYMBOLS.z)
            .name("Distance above the base of the fire to the bottom of the opening")
            .units("m")
            .min_exclusive(0.0)
            .build();

        params.add(v_e_1011);
        params.add(q);
        params.add(z);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {

        // equation 10.10
        let v_e = params.get(SYMBOLS.v_e);
        let g = params.get(SYMBOLS.g);
        let h = params.get(SYMBOLS.h);
        let t_f = params.get(SYMBOLS.t_f);
        let t_0 = params.get(SYMBOLS.t_0);


        let v_e_1011 = params.get(SYMBOLS.v_e_1011);
        let q = params.get(SYMBOLS.q);
        let z = params.get(SYMBOLS.z);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        // equation 10.10
        let step_1_deps = vec![g.clone(), h.clone(), t_f.clone(), t_0.clone()];
        let mut nomenclature_step_1 = step_1_deps.clone();
        nomenclature_step_1.push(v_e.clone());


        let step_1 = Step {
            name: "Limiting air velocity | Eq. 10.10".to_string(),
            nomenclature: nomenclature_step_1,
            input: step_1_deps.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(equation_10_10(
                v_e.symbol(),
                g.symbol(),
                h.symbol(),
                t_f.symbol(),
                t_0.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                equation_10_10(
                    v_e.symbol(),
                    g.display_value(),
                    h.display_value(),
                    t_f.display_value(),
                    t_f.display_value(),
                ),
                v_e.clone(),
            )]],
        };
        calc_sheet.write().unwrap().add_step(step_1);

        // equation 10.11
        let step_2_deps = vec![q.clone(), z.clone()];
        let mut nomenclature_step_2 = step_2_deps.clone();
        nomenclature_step_2.push(v_e_1011.clone());
        

        let step_2 = Step {
            name: "Limiting air velocity | Eq. 10.11".to_string(),
            nomenclature: nomenclature_step_2,
            input: step_2_deps.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(equation_10_11(
                v_e_1011.symbol(),
                q.symbol(),
                z.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                equation_10_11(
                    v_e_1011.symbol(),
                    q.display_value(),
                    z.display_value(),
                ),
                v_e_1011.clone(),
            )]],
        };
        calc_sheet.write().unwrap().add_step(step_2);

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
        // equation 10.10
        let v_e = method.parameters.get(SYMBOLS.v_e);
        let g = method.parameters.get(SYMBOLS.g).as_float();
        let h = method.parameters.get(SYMBOLS.h).as_float();
        let t_f= method.parameters.get(SYMBOLS.t_f).as_float();
        let t_0= method.parameters.get(SYMBOLS.t_0).as_float();

        let result = super::limiting_velocity_10_10(g, h, t_f, t_0);
        v_e.update(Some(result.to_string()))?;

        // equation 10.11
        let v_e_1011 = method.parameters.get(SYMBOLS.v_e_1011);
        let q = method.parameters.get(SYMBOLS.q).as_float();
        let z = method.parameters.get(SYMBOLS.z).as_float();

        let result_1011 = super::limiting_velocity_10_11(q, z);
        v_e_1011.update(Some(result_1011.to_string()))?;
        return Ok(());
    }
}

