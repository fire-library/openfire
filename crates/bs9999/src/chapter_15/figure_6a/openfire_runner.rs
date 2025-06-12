pub mod integration_tests;

use framework::method::calculation::Calculation;
use framework::method::calculation::CalculationComponent;
use framework::method::equation::Dependency;
use framework::method::form::{Form, FormStep};
use framework::method::parameter::ParameterValue;
use framework::method::parameter::builder::ParamBuilder;
use framework::method::parameter::{ArcParameter, ParameterTrait};
use framework::method::parameters::Parameters;
use framework::method::runner::MethodRunner;
use framework::method::tag::Tag;
use framework::method::test::IntegrationTests;
use framework::method::validation::ParameterError;

use framework::method::{Method, step::Step};
use std::sync::{Arc, RwLock};
use std::vec;

use crate::BS9999;
use crate::chapter_15::Chapter15Method;
use crate::chapter_15::figure_6a as bs9999_figure6a;

struct Symbols {
    w_fe: &'static str,
    s_up: &'static str,
    w_se: &'static str,
    n: &'static str,
    d: &'static str,
    x: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    w_fe: "W_{FE}",
    s_up: "S_{up}",
    w_se: "W_{SE}",
    n: "N",
    d: "D",
    x: "X",
};

#[derive(Default)]
pub struct BS9999Chapter15Figure6aBuilder;

impl MethodRunner for BS9999Chapter15Figure6aBuilder {
    fn name(&self) -> String {
        "Merging flow from stair with storey exit at final exit level".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &BS9999::ChapterFifteen(Chapter15Method::Figure6a)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Evacuation]
    }
    fn description(&self) -> Option<String> {
        Some("Calculates the merging flow for stairs where the stair and storey exit share a common final exit".to_string())
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let w_fe = params.get(SYMBOLS.w_fe);

        Some(vec![w_fe])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let s_up = params.get(SYMBOLS.s_up);
        let w_se = params.get(SYMBOLS.w_se);
        let n = params.get(SYMBOLS.n);
        let d = params.get(SYMBOLS.d);
        let x = params.get(SYMBOLS.x);
        let w_fe = params.get(SYMBOLS.w_fe);

        let equation = BS9999Chapter15Figure6a::new(
            w_fe.clone(),
            s_up.clone(),
            w_se.clone(),
            n.clone(),
            d.clone(),
            x.clone(),
        );

        let mut step_1 = FormStep::new(
            "Input | Fig. 6a",
            "Input required to calculate the width of the final exit.",
        );
        for param in params.values().into_iter() {
            if param.symbol() == SYMBOLS.w_fe {
                continue;
            }
            step_1.add_field(param.to_field())
        }

        step_1.add_intro();
        step_1.add_equation(equation.generate_with_symbols()[0][0].clone());
        step_1.add_text("or");
        step_1.add_equation(equation.generate_with_symbols()[1][0].clone());

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let s_up = ParamBuilder::float(SYMBOLS.s_up)
            .name("Stair width for upward portion of the stair")
            .units("mm")
            .min(0.0)
            .default_value(Some(ParameterValue::Float(1000.0)))
            .required()
            .build();

        let w_se = ParamBuilder::float(SYMBOLS.w_se)
            .name("Width of the final exit level storey exit")
            .units("mm")
            .min(0.0)
            .default_value(Some(ParameterValue::Float(850.0)))
            .required()
            .build();

        let n = ParamBuilder::float(SYMBOLS.n)
            .name("Number of people served by the final exit level storey exit")
            .min(0.0)
            .required()
            .build();

        let d = ParamBuilder::float(SYMBOLS.d)
            .name("Lesser distance from final exit level storey exit or lowest riser from stair")
            .units("m")
            .min(0.0)
            .required()
            .build();

        let x = ParamBuilder::float(SYMBOLS.x)
            .name("Minimum door width per person")
            .units("mm/person")
            .min(0.0)
            .required()
            .default_value(Some(ParameterValue::Float(3.6)))
            .build();

        let w_fe = ParamBuilder::float(SYMBOLS.w_fe)
            .name("Width of the final exit")
            .units("mm")
            .build();

        params.add(s_up);
        params.add(w_se);
        params.add(n);
        params.add(d);
        params.add(x);
        params.add(w_fe);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let s_up = params.get(SYMBOLS.s_up);
        let w_se = params.get(SYMBOLS.w_se);
        let n = params.get(SYMBOLS.n);
        let d = params.get(SYMBOLS.d);
        let x = params.get(SYMBOLS.x);
        let w_fe = params.get(SYMBOLS.w_fe);
        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));

        let equation = BS9999Chapter15Figure6a::new(
            w_fe.clone(),
            s_up.clone(),
            w_se.clone(),
            n.clone(),
            d.clone(),
            x.clone(),
        );
        let step = Step {
            name: "Width of the final exit".to_string(),
            nomenclature: equation.dependencies(),
            input: equation.input(),
            process: equation.generate_with_symbols(),
            calculation: equation.generate_with_values(),
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
        let s_up = method.parameters.get(SYMBOLS.s_up).as_float();
        let w_se = method.parameters.get(SYMBOLS.w_se).as_float();
        let n = method.parameters.get(SYMBOLS.n).as_float();
        let d = method.parameters.get(SYMBOLS.d).as_float();
        let x = method.parameters.get(SYMBOLS.x).as_float();

        let w_fe = method.parameters.get(SYMBOLS.w_fe);

        let result = bs9999_figure6a::calculate_exit_width(s_up, w_se, n, d, x);
        w_fe.update(Some(result.to_string()))?;

        return Ok(());
    }
}

#[derive(Debug)]
pub struct BS9999Chapter15Figure6a {
    w_fe: ArcParameter,
    s_up: ArcParameter,
    w_se: ArcParameter,
    n: ArcParameter,
    d: ArcParameter,
    x: ArcParameter,
}

impl BS9999Chapter15Figure6a {
    pub fn new(
        w_fe: ArcParameter,
        s_up: ArcParameter,
        w_se: ArcParameter,
        n: ArcParameter,
        d: ArcParameter,
        x: ArcParameter,
    ) -> Self {
        BS9999Chapter15Figure6a {
            w_fe,
            s_up,
            w_se,
            n,
            d,
            x,
        }
    }
}

impl BS9999Chapter15Figure6a {
    pub fn dependencies(&self) -> Vec<ArcParameter> {
        vec![
            self.s_up.clone(),
            self.w_se.clone(),
            self.n.clone(),
            self.d.clone(),
            self.w_fe.clone(),
            self.x.clone(),
        ]
    }

    pub fn input(&self) -> Vec<Dependency> {
        vec![
            self.s_up.clone().into(),
            self.w_se.clone().into(),
            self.n.clone().into(),
            self.d.clone().into(),
            self.x.clone().into(),
        ]
    }

    pub fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "W_{{FE}} = {}",
            equation_1(self.s_up.symbol(), self.w_se.symbol()),
        );
        let cond_1 = "\\text{if: N > 60 and D < 2 m}".to_string();

        let eq_2 = format!(
            "W_{{FE}} = {}",
            equation_2(self.n.symbol(), self.x.symbol(), self.s_up.symbol(),),
        );
        let cond_2 = "N < 60 \\space and/or  \\space D > 2 m".to_string();

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
        let n = self.n.as_float();
        let d = self.d.as_float();
        let result = if n > 60.0 && d < 2.0 {
            let cond = format!("N = {} \\quad and \\quad D = {}  \\quad", n, d);
            let eq = format!(
                "W_{{FE}} = {} + {}",
                self.s_up.as_float(),
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
            let cond = format!("N = {} \\quad and \\quad D = {}", n, d);

            let eq = format!(
                "W_{{FE}} = {} \\cdot {} + 0.75 \\cdot {}",
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

fn equation_1(s_up: String, w_se: String) -> String {
    format!("{} + {}", s_up, w_se)
}

fn equation_2(n: String, x: String, s_up: String) -> String {
    format!("{} \\cdot {} + 0.75 \\cdot {}", n, x, s_up)
}
