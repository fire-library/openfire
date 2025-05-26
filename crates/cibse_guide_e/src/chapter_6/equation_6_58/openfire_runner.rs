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
    r: &'static str,
    a_t: &'static str,
    a_o: &'static str,
    h_o: &'static str,
    w: &'static str,
    d: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    r: "R",
    a_t: "A_{t}",
    a_o: "A_{o}",
    h_o: "h_{o}",
    w: "w",
    d: "d",
};

#[derive(Default)]
pub struct Chapter6Equation7Runner;

impl MethodRunner for Chapter6Equation7Runner {
    fn name(&self) -> String {
        "Ventilation-controlled rate of burning".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::Chaptersix(crate::chapter_6::Chapter6Method::Equation6_58)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::HRR, Tag::FireDynamics]
    }
    fn description(&self) -> Option<String> {
        Some(
            "Ventilation-controlled rate of burning for cellulosic fires"
                .to_string(),
        )
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let r = params.get(SYMBOLS.r);

        Some(vec![r])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let r = params.get(SYMBOLS.r);
        let a_t = params.get(SYMBOLS.a_t);
        let a_o = params.get(SYMBOLS.a_o);
        let h_o = params.get(SYMBOLS.h_o);
        let w = params.get(SYMBOLS.w);
        let d = params.get(SYMBOLS.d);

        let mut step_1 = FormStep::new(
            "Input | Eq. 6.58",
            "Calculate the rate of burning for a ventilation-controlled fire",
        );
        step_1.add_field(a_t.to_field());
        step_1.add_field(a_o.to_field());
        step_1.add_field(h_o.to_field());
        step_1.add_field(w.to_field());
        step_1.add_field(d.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(super::equation(
            r.symbol(),
            a_t.symbol(),
            a_o.symbol(),
            h_o.symbol(),
            w.symbol(),
            d.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let r = ParamBuilder::float(SYMBOLS.r)
            .name("Rate of fuel combustion")
            .units("kg/s")
            .build();

        let a_t = ParamBuilder::float(SYMBOLS.a_t)
            .name("Area of enclosing walls")
            .units("m^2")
            .min_exclusive(0.0)
            .required()
            .build();

        let a_o = ParamBuilder::float(SYMBOLS.a_o)
            .name("Area of the opening")
            .units("m^2")
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
            .name("Width of the wall containing the opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let d = ParamBuilder::float(SYMBOLS.d)
            .name("Depth of the room behind the opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        params.add(r);
        params.add(a_t);
        params.add(a_o);
        params.add(h_o);
        params.add(w);
        params.add(d);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let r = params.get(SYMBOLS.r);
        let a_t = params.get(SYMBOLS.a_t);
        let a_o = params.get(SYMBOLS.a_o);
        let h_o = params.get(SYMBOLS.h_o);
        let w = params.get(SYMBOLS.w);
        let d = params.get(SYMBOLS.d);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let step = vec![a_t.clone(), a_o.clone(), h_o.clone(), w.clone(), d.clone()];
        let mut nomenclature = step.clone();
        nomenclature.push(r.clone());

        let step = Step {
            name: "Ventilation-controlled rate of burning | Eq. 6.58".to_string(),
            nomenclature: nomenclature,
            input: step.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(super::equation(
                r.symbol(),
                a_t.symbol(),
                a_o.symbol(),
                h_o.symbol(),
                w.symbol(),
                d.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::equation(r.symbol(), a_t.display_value(), a_o.display_value(), h_o.display_value(), w.display_value(), d.display_value()),
                r.clone(),
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
        let r = method.parameters.get(SYMBOLS.r);
        let a_t = method.parameters.get(SYMBOLS.a_t).as_float();
        let a_o = method.parameters.get(SYMBOLS.a_o).as_float();
        let h_o = method.parameters.get(SYMBOLS.h_o).as_float();
        let w = method.parameters.get(SYMBOLS.w).as_float();
        let d = method.parameters.get(SYMBOLS.d).as_float();

        let result = super::ventcontrolled_rate_of_burning(a_t, a_o, h_o, w, d);
        r.update(Some(result.to_string()))?;

        Ok(())
    }
}
