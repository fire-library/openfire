pub mod integration_tests;

use framework::method::calculation::Calculation;
use framework::method::calculation::CalculationComponent;
use framework::method::form::{Form, FormStep};
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;
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
    z_fo: &'static str,
    r: &'static str,
    w: &'static str,
    h_o: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    z_fo: "z_{fo}",
    r: "R",
    w: "w",
    h_o: "h_{o}",
};


#[derive(Default)]
pub struct Chapter6Equation57Runner;

impl MethodRunner for Chapter6Equation57Runner {
    fn name(&self) -> String {
        "Height of the flame above the top of the opening".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::Chaptersix(crate::chapter_6::Chapter6Method::Equation6_57)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::HRR, Tag::FireDynamics]
    }
    fn description(&self) -> Option<String> {
        Some("Height of the flame projection above the top of the opening".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let z_fo = params.get(SYMBOLS.r);
        Some(vec![z_fo])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let z_fo = params.get(SYMBOLS.z_fo);
        let r = params.get(SYMBOLS.r);
        let w = params.get(SYMBOLS.w);
        let h_o = params.get(SYMBOLS.h_o);

        let mut step_1 = FormStep::new(
            "Input | Eq. 6.57",
            "Calculate the height of the flame projection above the top of the opening",
        );
        step_1.add_field(r.to_field());
        step_1.add_field(w.to_field());
        step_1.add_field(h_o.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(super::equation(
            z_fo.symbol(),
            r.symbol(),
            w.symbol(),
            h_o.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let z_fo = ParamBuilder::float(SYMBOLS.z_fo)
            .name("Flame height above the top of the opening")
            .units("m")
            .build();

        let r = ParamBuilder::float(SYMBOLS.r)
            .name("Rate of fuel combustion")
            .units("kg/s")
            .min_exclusive(0.0)
            .required()
            .build();

        let h_o = ParamBuilder::float(SYMBOLS.h_o)
            .name("Height of the opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let w = ParamBuilder::float(SYMBOLS.w)
            .name("Width of the compartment opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();


        params.add(z_fo);
        params.add(r);
        params.add(w);
        params.add(h_o);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let z_fo = params.get(SYMBOLS.z_fo);
        let r = params.get(SYMBOLS.r);
        let w = params.get(SYMBOLS.w);
        let h_o = params.get(SYMBOLS.h_o);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let step = vec![r.clone(), w.clone(), h_o.clone()];
        let nomenclature = step.clone();

        let step = Step {
            name: "Height of flame above opening | Eq. 6.57".to_string(),
            nomenclature: nomenclature,
            input: step.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(super::equation(z_fo.symbol(),
                r.symbol(),
                w.symbol(),
                h_o.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::equation(
                    z_fo.symbol(),
                    r.display_value(),
                    w.display_value(),
                    h_o.display_value(),
                ),
                z_fo.clone(),
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
        let r = method.parameters.get(SYMBOLS.r).as_float();
        let w = method.parameters.get(SYMBOLS.w).as_float();
        let h_o = method.parameters.get(SYMBOLS.h_o).as_float();
        let z_fo = method.parameters.get(SYMBOLS.z_fo);

        let result = super::heightofflame_aboveopening(r, w, h_o);
        z_fo.update(Some(result.to_string()))?;

        Ok(())
    }
}
