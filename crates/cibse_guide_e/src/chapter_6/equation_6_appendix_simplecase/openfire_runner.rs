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
    a_f: &'static str,
    a_o: &'static str,
    a_net: &'static str,
    h: &'static str,
    h_o: &'static str,
    w_o: &'static str,
    w_1: &'static str,
    w_2: &'static str,
    d_over_w: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    a_f: "A_f",
    a_o: "A_o",
    a_net: "A_{net}",
    h: "h",
    h_o: "h_o",
    w_o: "w_o",
    w_1: "w_1",
    w_2: "w_2",
    d_over_w: "d/w",
};

#[derive(Default)]
pub struct Chapter6EquationAppendixSimpleCaseRunner;

impl MethodRunner for Chapter6EquationAppendixSimpleCaseRunner {
    fn name(&self) -> String {
        "Relevant dimensions of a room or compartment. Single opening".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &CIBSEGuideE::Chaptersix(crate::chapter_6::Chapter6Method::Equation6_Appendix_SimpleCase)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::HRR, Tag::FireDynamics]
    }
    fn description(&self) -> Option<String> {
        Some(
            "Effective or equivalent dimensions of a room or compartment, needed for the use of HRR correlations"
                .to_string(),
        )
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let a_f = params.get(SYMBOLS.a_f);
        let a_o = params.get(SYMBOLS.a_o);
        let a_net = params.get(SYMBOLS.a_net);
        let d_over_w = params.get(SYMBOLS.d_over_w);

        Some(vec![a_f, a_o, a_net, d_over_w])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let a_f = params.get(SYMBOLS.a_f);
        let a_o = params.get(SYMBOLS.a_o);
        let a_net = params.get(SYMBOLS.a_net);
        let d_over_w = params.get(SYMBOLS.d_over_w);
        let h = params.get(SYMBOLS.h);
        let h_o = params.get(SYMBOLS.h_o);
        let w_o = params.get(SYMBOLS.w_o);
        let w_1 = params.get(SYMBOLS.w_1);
        let w_2 = params.get(SYMBOLS.w_2);

        let mut step_1 = FormStep::new(
            "Input | Simple case",
            "Dimensions for room with single opening",
        );
        step_1.add_field(h.to_field());
        step_1.add_field(h_o.to_field());
        step_1.add_field(w_o.to_field());
        step_1.add_field(w_1.to_field());
        step_1.add_field(w_2.to_field());

        step_1.add_intro();
        step_1.add_equation(CalculationComponent::Equation(super::equation_af(
            a_f.symbol(),
            w_1.symbol(),
            w_2.symbol(),
        )));
        step_1.add_equation(CalculationComponent::Equation(super::equation_ao(
            a_o.symbol(),
            w_o.symbol(),
            h_o.symbol(),
        )));
        step_1.add_equation(CalculationComponent::Equation(super::equation_anet(
            a_net.symbol(),
            a_f.symbol(),
            h.symbol(),
            w_1.symbol(),
            w_2.symbol(),
            a_o.symbol(),
        )));
        step_1.add_equation(CalculationComponent::Equation(super::equation_doverw(
            d_over_w.symbol(),
            w_1.symbol(),
            w_2.symbol(),
        )));

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let a_f = ParamBuilder::float(&SYMBOLS.a_f)
            .name("Floor area")
            .units("m^2")
            .build();

        let a_o = ParamBuilder::float(SYMBOLS.a_o)
            .name("Area of opening of room")
            .units("m^2")
            .build();

        let a_net = ParamBuilder::float(SYMBOLS.a_net)
            .name("Internal surface area of the room minus area of the openings")
            .units("m^2")
            .build();

        let d_over_w = ParamBuilder::float(SYMBOLS.d_over_w)
            .name("Depth of the opening divided by the width of the opening")
            .build();

        let h = ParamBuilder::float(SYMBOLS.h)
            .name("Floor to ceiling height of room or, height above base of fire")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let h_o = ParamBuilder::float(SYMBOLS.h_o)
            .name("Height of the opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let w_o = ParamBuilder::float(SYMBOLS.w_o)
            .name("Width of the opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let w_1 = ParamBuilder::float(SYMBOLS.w_1)
            .name("Width of wall containing the opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        let w_2 = ParamBuilder::float(SYMBOLS.w_2)
            .name("Width of wall without an opening")
            .units("m")
            .min_exclusive(0.0)
            .required()
            .build();

        params.add(a_f);
        params.add(a_o);
        params.add(a_net);
        params.add(d_over_w);
        params.add(h);
        params.add(h_o);
        params.add(w_o);
        params.add(w_1);
        params.add(w_2);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let a_f = params.get(SYMBOLS.a_f);
        let a_o = params.get(SYMBOLS.a_o);
        let a_net = params.get(SYMBOLS.a_net);
        let d_over_w = params.get(SYMBOLS.d_over_w);
        let h = params.get(SYMBOLS.h);
        let h_o = params.get(SYMBOLS.h_o);
        let w_o = params.get(SYMBOLS.w_o);
        let w_1 = params.get(SYMBOLS.w_1);
        let w_2 = params.get(SYMBOLS.w_2);

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let step_af = vec![w_1.clone(), w_2.clone()];
        let mut nomenclature_af = step_af.clone();
        nomenclature_af.push(a_f.clone());

        let step_af = Step {
            name: "Floor area".to_string(),
            nomenclature: nomenclature_af,
            input: step_af.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(super::equation_af(
                a_f.symbol(),
                w_1.symbol(),
                w_2.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::equation_af(a_f.symbol(), w_1.display_value(), w_2.display_value()),
                a_f.clone(),
            )]],
        };

        calc_sheet.write().unwrap().add_step(step_af);

        let step_ao = vec![w_o.clone(), h_o.clone()];
        let mut nomenclature_ao = step_ao.clone();
        nomenclature_ao.push(a_o.clone());

        let step_ao = Step {
            name: "Area of the opening".to_string(),
            nomenclature: nomenclature_ao,
            input: step_ao.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(super::equation_ao(
                a_o.symbol(),
                w_o.symbol(),
                h_o.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::equation_ao(a_o.symbol(), w_o.display_value(), h_o.display_value()),
                a_o.clone(),
            )]],
        };

        calc_sheet.write().unwrap().add_step(step_ao);

        let step_anet = vec![
            a_f.clone(),
            h.clone(),
            w_1.clone(),
            w_2.clone(),
            a_o.clone(),
        ];
        let mut nomenclature_anet = step_anet.clone();
        nomenclature_anet.push(a_net.clone());

        let step_anet = Step {
            name: "Internal surface area of the room minus area of the openings".to_string(),
            nomenclature: nomenclature_anet,
            input: step_anet.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(super::equation_anet(
                a_net.symbol(),
                a_f.symbol(),
                h.symbol(),
                w_1.symbol(),
                w_2.symbol(),
                a_o.symbol(),
            ))]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::equation_anet(
                    a_net.symbol(),
                    a_f.display_value(),
                    h.display_value(),
                    w_1.display_value(),
                    w_2.display_value(),
                    a_o.display_value(),
                ),
                a_net.clone(),
            )]],
        };

        calc_sheet.write().unwrap().add_step(step_anet);

        let step_doverw = vec![w_1.clone(), w_2.clone()];
        let mut nomenclature_doverw = step_doverw.clone();
        nomenclature_doverw.push(d_over_w.clone());

        let step_doverw = Step {
            name: "Depth of the opening divided by the width of the opening".to_string(),
            nomenclature: nomenclature_doverw,
            input: step_doverw.clone().into_iter().map(|p| p.into()).collect(),
            render: true,
            process: vec![vec![CalculationComponent::Equation(
                super::equation_doverw(d_over_w.symbol(), w_1.symbol(), w_2.symbol()),
            )]],
            calculation: vec![vec![CalculationComponent::EquationWithResult(
                super::equation_doverw(d_over_w.symbol(), w_1.display_value(), w_2.display_value()),
                d_over_w.clone(),
            )]],
        };

        calc_sheet.write().unwrap().add_step(step_doverw);

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
        let a_f = method.parameters.get(SYMBOLS.a_f);
        let w_1 = method.parameters.get(SYMBOLS.w_1).as_float();
        let w_2 = method.parameters.get(SYMBOLS.w_2).as_float();

        let result_af = super::af(w_1, w_2);
        a_f.update(Some(result_af.to_string()))?;

        let a_o = method.parameters.get(SYMBOLS.a_o);
        let w_o = method.parameters.get(SYMBOLS.w_o).as_float();
        let h_o = method.parameters.get(SYMBOLS.h_o).as_float();

        let result_ao = super::ao(w_o, h_o);
        a_o.update(Some(result_ao.to_string()));

        let a_net = method.parameters.get(SYMBOLS.a_net);
        let h = method.parameters.get(SYMBOLS.h).as_float();

        let result_anet = super::anet(result_af, h, w_1, w_2, result_ao);
        a_net.update(Some(result_anet.to_string()));

        let d_over_w = method.parameters.get(SYMBOLS.d_over_w);
        let result_doverw = super::d_over_w(w_1, w_2);
        d_over_w.update(Some(result_doverw.to_string()));

        return Ok(());
    }
}
