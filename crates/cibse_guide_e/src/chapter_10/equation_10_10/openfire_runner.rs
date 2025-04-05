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
    g: &'static str,
    h: &'static str,
    t_f: &'static str,
    t_0: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    v_e: "v_e",
    g: "g",
    h: "H",
    t_f: "T_f",
    t_0: "T_0",
};

#[derive(Default)]
pub struct Chapter10Equation10Runner;

impl MethodRunner for Chapter10Equation10Runner {
    fn name(&self) -> String {
        "Limiting average air velocity for opposed air flow | Room of fire origin to large volume".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::ChapterTen(crate::chapter_10::Chapter10Method::Equation10_10)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Ventilation]
    }
    fn description(&self) -> Option<String> {
        Some("Limiting average air velocity where opposed air flow is used to stop smoke spread from room of origin to a large volume".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let v_e = params.get(SYMBOLS.v_e);

        Some(vec![v_e])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let v_e = params.get(SYMBOLS.v_e);
        let g = params.get(SYMBOLS.g);
        let h = params.get(SYMBOLS.h);
        let t_f = params.get(SYMBOLS.t_f);
        let t_0 = params.get(SYMBOLS.t_0);


        let mut step_1 = FormStep::new(
            "Input | Eq. 10.10",
            "Calculate the limiting average air velocity to prevent smoke spread into adjoining large volume.",
        );
        step_1.add_field(g.to_field());
        step_1.add_field(h.to_field());
        step_1.add_field(t_f.to_field());
        step_1.add_field(t_0.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(super::limiting_velocity_symbols(
            v_e.symbol(),
            g.symbol(),
            h.symbol(),
            t_f.symbol(),
            t_0.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let v_e = ParamBuilder::float(&SYMBOLS.v_e)
            .name("Limiting average air velocity")
            .units("m/s")
            .build();

        let g = ParamBuilder::float(SYMBOLS.g)
            .name("Gravity acceleration")
            .units("m/s^2")
            .min_exclusive(0.0)
            .required()
            .default_value(Some(ParameterValue::Float(9.8)))
            .build();

        let h = ParamBuilder::float(SYMBOLS.h)
            .name("Height of the opening measured from the bottom of the opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let t_f = ParamBuilder::float(SYMBOLS.t_f)
            .name("Temperature of the heated smoke")
            .units("K")
            .min_exclusive(0.0)
            .required()
            .build();

        let t_0 = ParamBuilder::float(SYMBOLS.t_0)
            .name("Temperature of ambient air")
            .units("K")
            .min_exclusive(0.0)
            .default_value(Some(ParameterValue::Float(293.0)))
            .required()
            .build();

        params.add(v_e);
        params.add(g);
        params.add(h);
        params.add(t_f);
        params.add(t_0);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {

        let v_e = params.get(SYMBOLS.v_e);
        let g = params.get(SYMBOLS.g);
        let h = params.get(SYMBOLS.h);
        let t_f = params.get(SYMBOLS.t_f);
        let t_0 = params.get(SYMBOLS.t_0);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let step = vec![g.clone(), h.clone(), t_f.clone(), t_0.clone()];
        let mut nomenclature = step.clone();
        nomenclature.push(v_e.clone());


        let step = Step {
            name: "Limiting air velocity | Eq. 10.10".to_string(),
            nomenclature: nomenclature,
            input: step.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(super::limiting_velocity_symbols(
                v_e.symbol(),
                g.symbol(),
                h.symbol(),
                t_f.symbol(),
                t_0.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::limiting_velocity_symbols(
                    v_e.symbol(),
                    g.display_value(),
                    h.display_value(),
                    t_f.display_value(),
                    t_0.display_value(),
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
        let g = method.parameters.get(SYMBOLS.g).as_float();
        let h = method.parameters.get(SYMBOLS.h).as_float();
        let t_f= method.parameters.get(SYMBOLS.t_f).as_float();
        let t_0= method.parameters.get(SYMBOLS.t_0).as_float();

        let result = super::limiting_velocity(g, h, t_f, t_0);
        v_e.update(Some(result.to_string()))?;

        return Ok(());
    }
}
