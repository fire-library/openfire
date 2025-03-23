pub mod integration_tests;

use framework::method::calculation::Calculation;
use framework::method::calculation::CalculationComponent;
use framework::method::equation::Dependency;
use framework::method::form::{Form, FormStep};
use framework::method::parameter::ParameterValue;
use framework::method::parameter::Parameters;
use framework::method::parameter::builder::ParamBuilder;
use framework::method::parameter::{ArcParameter, ParameterTrait};
use framework::method::runner::MethodRunner;
use framework::method::tag::Tag;
use framework::method::test::IntegrationTests;
use framework::method::validation::ParameterError;

use framework::method::{Method, step::Step};
use std::sync::{Arc, RwLock};
use std::vec;

use crate::BS9999;
use crate::chapter_15::Chapter15Method;
use crate::chapter_15::figure_6c as bs9999_figure6c;

#[derive(Default)]
pub struct BS9999Chapter15Figure6cBuilder;

impl MethodRunner for BS9999Chapter15Figure6cBuilder {
    fn name(&self) -> String {
        "Merging flow from stairs from above and below, combined with storey exit".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &BS9999::ChapterFifteen(Chapter15Method::Figure6c)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Evacuation]
    }
    fn description(&self) -> Option<String> {
        Some(
            "Calcualtes the merging flow from stairs above and below the final exit level, combined with with storey exit"
                .to_string(),
        )
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let w_fe = params.get("W_{FE}");

        Some(vec![w_fe])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let b = params.get("B");
        let n = params.get("N");
        let d = params.get("D");
        let s_up = params.get("S_{up}");
        let s_dn = params.get("S_{dn}");
        let w_se = params.get("W_{SE}");
        let x = params.get("X");
        let w_fe = params.get("W_{FE}");

        let w_fe_equation = BS9999Chapter15Figure6c::new(
            w_fe.clone(),
            b.clone(),
            n.clone(),
            d.clone(),
            s_up.clone(),
            s_dn.clone(),
            w_se.clone(),
            x.clone(),
        );

        let mut step_1 = FormStep::new(
            "Input | Fig. 6c",
            "Input required to calculate the width of the final exit.",
        );
        for param in params.values().into_iter() {
            if param.symbol() == "W_{FE}" {
                continue;
            }
            step_1.add_field(param.to_field())
        }
        step_1.add_intro();
        step_1.add_equation(w_fe_equation.generate_with_symbols()[0][0].clone());
        step_1.add_text("or");
        step_1.add_equation(w_fe_equation.generate_with_symbols()[1][0].clone());

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let b = ParamBuilder::float("B")
            .name("Number of people served by the stair from below the final exit level")
            .min(0.0)
            .required()
            .build();

        let n = ParamBuilder::float("N")
            .name("Number of people served by the final exit level storey exit")
            .min(0.0)
            .required()
            .build();

        let d = ParamBuilder::float("D")
            .name("Lesser distance from the final exit level storey exit or the lowest riser from the upward portion of the stairs")
            .units("m")
            .min(0.0)
            .required()
            .build();

        let s_up = ParamBuilder::float("S_{up}")
            .name("Stair width for upward portion of the stair")
            .units("mm")
            .min(0.0)
            .default_value(Some(ParameterValue::Float(1000.0)))
            .required()
            .build();

        let s_dn = ParamBuilder::float("S_{dn}")
            .name("Stair width for downward portion of the stair")
            .units("mm")
            .min(0.0)
            .default_value(Some(ParameterValue::Float(1000.0)))
            .required()
            .build();

        let w_se = ParamBuilder::float("W_{SE}")
            .name("Width of the final exit level storey exit")
            .units("mm")
            .min(0.0)
            .required()
            .default_value(Some(ParameterValue::Float(850.0)))
            .build();

        let x = ParamBuilder::float("X")
            .name("Minimum door width per person")
            .units("mm/person")
            .min(0.0)
            .required()
            .default_value(Some(ParameterValue::Float(3.6)))
            .build();

        let w_fe = ParamBuilder::float("W_{FE}")
            .name("Width of the final exit")
            .units("mm")
            .build();

        params.add(b);
        params.add(n);
        params.add(d);
        params.add(s_up);
        params.add(s_dn);
        params.add(w_se);
        params.add(x);
        params.add(w_fe);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let b = params.get("B");
        let n = params.get("N");
        let d = params.get("D");
        let s_up = params.get("S_{up}");
        let s_dn = params.get("S_{dn}");
        let w_se = params.get("W_{SE}");
        let x = params.get("X");
        let w_fe = params.get("W_{FE}");

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));
        let w_fe_equation = BS9999Chapter15Figure6c::new(
            w_fe.clone(),
            b.clone(),
            n.clone(),
            d.clone(),
            s_up.clone(),
            s_dn.clone(),
            w_se.clone(),
            x.clone(),
        );
        let step = Step {
            name: "Width of the final exit".to_string(),
            nomenclature: w_fe_equation.dependencies(),
            input: w_fe_equation.input(),
            process: w_fe_equation.generate_with_symbols(),
            calculation: w_fe_equation.generate_with_values(),
            render: true,
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
        let b = method.parameters.get("B").as_float();
        let n = method.parameters.get("N").as_float();
        let d = method.parameters.get("D").as_float();
        let s_up = method.parameters.get("S_{up}").as_float();
        let s_dn = method.parameters.get("S_{dn}").as_float();
        let w_se = method.parameters.get("W_{SE}").as_float();
        let x = method.parameters.get("X").as_float();

        let w_fe = method.parameters.get("W_{FE}");

        let result = bs9999_figure6c::calculate_exit_width(b, n, d, s_up, s_dn, w_se, x);
        w_fe.update(Some(result.to_string()))?;

        return Ok(());
    }
}

#[derive(Debug)]
pub struct BS9999Chapter15Figure6c {
    w_fe: ArcParameter,
    b: ArcParameter,
    n: ArcParameter,
    d: ArcParameter,
    s_up: ArcParameter,
    s_dn: ArcParameter,
    w_se: ArcParameter,
    x: ArcParameter,
}

impl BS9999Chapter15Figure6c {
    pub fn new(
        w_fe: ArcParameter,
        b: ArcParameter,
        n: ArcParameter,
        d: ArcParameter,
        s_up: ArcParameter,
        s_dn: ArcParameter,
        w_se: ArcParameter,
        x: ArcParameter,
    ) -> Self {
        BS9999Chapter15Figure6c {
            w_fe,
            b,
            n,
            d,
            s_up,
            s_dn,
            w_se,
            x,
        }
    }
}

impl BS9999Chapter15Figure6c {
    pub fn dependencies(&self) -> Vec<ArcParameter> {
        vec![
            self.b.clone(),
            self.n.clone(),
            self.d.clone(),
            self.s_up.clone(),
            self.s_dn.clone(),
            self.w_se.clone(),
            self.w_fe.clone(),
            self.x.clone(),
        ]
    }

    pub fn input(&self) -> Vec<Dependency> {
        vec![
            self.b.clone().into(),
            self.n.clone().into(),
            self.d.clone().into(),
            self.s_up.clone().into(),
            self.s_dn.clone().into(),
            self.w_se.clone().into(),
            self.x.clone().into(),
        ]
    }

    pub fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "W_{{FE}} = {}",
            equation_1(self.s_up.symbol(), self.s_dn.symbol(), self.w_se.symbol()),
        );
        let cond_1 = "\\text{if: } B + N > 60 and D < 2 m".to_string();

        let eq_2 = format!(
            "W_{{FE}} = {}",
            equation_2(
                self.b.symbol(),
                self.n.symbol(),
                self.x.symbol(),
                self.s_up.symbol(),
            ),
        );
        let cond_2 = "\\text{if: }B + N < 60 \\space and/or  \\space D > 2 m".to_string();

        vec![
            vec![
                CalculationComponent::Equation(eq_1),
                CalculationComponent::Equation(cond_1),
            ],
            vec![
                CalculationComponent::Equation(eq_2),
                CalculationComponent::Equation(cond_2),
            ],
        ]
    }

    pub fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let b = self.b.as_float();
        let n = self.n.as_float();
        let d = self.d.as_float();

        let result = if b + n > 60.0 && d < 2.0 {
            let cond = format!("B + N = {} \\quad and \\quad D = {}  \\quad", b + n, d);
            let eq = format!(
                "W_{{FE}} = {} + {} + {}",
                self.s_up.as_float(),
                self.s_dn.as_float(),
                self.w_se.as_float(),
            );
            vec![
                vec![CalculationComponent::Equation(cond)],
                vec![(CalculationComponent::Equation("Therefore: ".to_string()))],
                vec![CalculationComponent::EquationWithResult(
                    eq,
                    self.w_fe.clone(),
                )],
            ]
        } else {
            let cond = format!("B + N = {} \\quad and \\quad D = {}", b + n, d);

            let eq = format!(
                "W_{{FE}} = {} \\cdot {} + {} \\cdot {} + 0.75 \\cdot {}",
                self.b.as_float(),
                self.x.as_float(),
                self.n.as_float(),
                self.x.as_float(),
                self.s_up.as_float(),
            );
            vec![
                vec![CalculationComponent::Equation(cond)],
                vec![CalculationComponent::Text("Therefore: ".to_string())],
                vec![CalculationComponent::EquationWithResult(
                    eq,
                    self.w_fe.clone(),
                )],
            ]
        };
        return result;
    }
}

fn equation_1(s_up: String, s_dn: String, w_se: String) -> String {
    format!("{} + {} + {}", s_up, s_dn, w_se)
}

fn equation_2(b: String, n: String, x: String, s_up: String) -> String {
    format!(
        "{} \\cdot {} + {} \\cdot {} + 0.75 \\cdot {}",
        b, x, n, x, s_up
    )
}
