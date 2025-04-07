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

use super::limiting_velocity_symbols;

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
        "Limiting average air velocity for opposed air flow | Room of fire origin to corridor"
            .to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterTen(crate::chapter_10::Chapter10Method::Equation10_12)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Ventilation]
    }
    fn description(&self) -> Option<String> {
        Some("Limiting average air velocity where opposed air flow is used to stop smoke spread from an adjoining room into a corridor".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let v_e = params.get(SYMBOLS.v_e);

        Some(vec![v_e])
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
        step.add_field(k.to_field());
        step.add_field(g.to_field());
        step.add_field(q.to_field());
        step.add_field(w.to_field());
        step.add_field(rho.to_field());
        step.add_field(c.to_field());
        step.add_field(t.to_field());

        step.add_intro();
        step.add_equation(CalculationComponent::Equation(limiting_velocity_symbols(
            v_e.symbol(),
            k.symbol(),
            g.symbol(),
            q.symbol(),
            w.symbol(),
            rho.symbol(),
            c.symbol(),
            t.symbol(),
        )));

        Form::new(vec![step])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let v_e = ParamBuilder::float(&SYMBOLS.v_e)
            .name("Limiting average air velocity")
            .units("m/s")
            .build();

        let k = ParamBuilder::float(SYMBOLS.k)
            .name("Constant")
            .min_exclusive(0.0)
            .default_value(Some(ParameterValue::Float(1.0)))
            .required()
            .build();

        let g = ParamBuilder::float(SYMBOLS.g)
            .name("Gravity acceleration")
            .units("m/s^2")
            .min_exclusive(0.0)
            .default_value(Some(ParameterValue::Float(9.8)))
            .required()
            .build();

        let q = ParamBuilder::float(SYMBOLS.q)
            .name("Heat Release Rate")
            .units("kW")
            .min_exclusive(0.0)
            .required()
            .build();

        let w = ParamBuilder::float(SYMBOLS.w)
            .name("Corridor width")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let rho = ParamBuilder::float(SYMBOLS.rho)
            .name("Density of upstream air")
            .units("kg/m^3")
            .min_exclusive(0.0)
            .default_value(Some(ParameterValue::Float(1.2)))
            .required()
            .build();

        let c = ParamBuilder::float(SYMBOLS.c)
            .name("Specific heat of downstrean gases")
            .units("kJ/kgK")
            .min_exclusive(0.0)
            .default_value(Some(ParameterValue::Float(1.0)))
            .required()
            .build();

        let t = ParamBuilder::float(SYMBOLS.t)
            .name("Temperature of the downstream mixture of air and smoke")
            .units("K")
            .min_exclusive(0.0)
            .required()
            .build();

        params.add(v_e);
        params.add(k);
        params.add(g);
        params.add(q);
        params.add(w);
        params.add(rho);
        params.add(c);
        params.add(t);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let v_e = params.get(SYMBOLS.v_e);
        let k = params.get(SYMBOLS.k);
        let g = params.get(SYMBOLS.g);
        let q = params.get(SYMBOLS.q);
        let w = params.get(SYMBOLS.w);
        let rho = params.get(SYMBOLS.rho);
        let c = params.get(SYMBOLS.c);
        let t = params.get(SYMBOLS.t);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let step = vec![
            k.clone(),
            g.clone(),
            q.clone(),
            w.clone(),
            rho.clone(),
            c.clone(),
            t.clone(),
        ];
        let mut nomenclature = step.clone();
        nomenclature.push(v_e.clone());

        let step = Step {
            name: "Limiting air velocity | Eq. 10.12".to_string(),
            nomenclature: nomenclature,
            input: step.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(
                super::limiting_velocity_symbols(
                    v_e.symbol(),
                    k.symbol(),
                    g.symbol(),
                    q.symbol(),
                    w.symbol(),
                    rho.symbol(),
                    c.symbol(),
                    t.symbol(),
                ),
            )]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::limiting_velocity_symbols(
                    v_e.symbol(),
                    k.display_value(),
                    g.display_value(),
                    q.display_value(),
                    w.display_value(),
                    rho.display_value(),
                    c.display_value(),
                    t.display_value(),
                ),
                v_e.clone(),
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
        let v_e = method.parameters.get(SYMBOLS.v_e);
        let k = method.parameters.get(SYMBOLS.k).as_float();
        let g = method.parameters.get(SYMBOLS.g).as_float();
        let q = method.parameters.get(SYMBOLS.q).as_float();
        let w = method.parameters.get(SYMBOLS.w).as_float();
        let rho = method.parameters.get(SYMBOLS.rho).as_float();
        let c = method.parameters.get(SYMBOLS.c).as_float();
        let t = method.parameters.get(SYMBOLS.t).as_float();

        let result = super::limiting_velocity(k, g, q, w, rho, c, t);
        v_e.update(Some(result.to_string()))?;

        return Ok(());
    }
}
