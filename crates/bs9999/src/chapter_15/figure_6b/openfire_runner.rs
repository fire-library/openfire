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
use crate::chapter_15::figure_6b as bs9999_figure6b;

struct Symbols {
    w_fe: &'static str,
    s_up: &'static str,
    s_dn: &'static str,
    b: &'static str,
    d: &'static str,
    x: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    w_fe: "W_{FE}",
    s_up: "S_{up}",
    s_dn: "S_{dn}",
    b: "B",
    d: "D",
    x: "X",
};

#[derive(Default)]
pub struct BS9999Chapter15Figure6bBuilder;

impl MethodRunner for BS9999Chapter15Figure6bBuilder {
    fn name(&self) -> String {
        "Merging flow from stair above and from stair below final exit level".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &BS9999::ChapterFifteen(Chapter15Method::Figure6b)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::Evacuation]
    }
    fn description(&self) -> Option<String> {
        Some(
            "Calcualtes the merging flow from stairs above and below the final exit level"
                .to_string(),
        )
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let w_fe = params.get(SYMBOLS.w_fe);

        Some(vec![w_fe])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let s_up = params.get(SYMBOLS.s_up);
        let s_dn = params.get(SYMBOLS.s_dn);
        let b = params.get(SYMBOLS.b);
        let d = params.get(SYMBOLS.d);
        let x = params.get(SYMBOLS.x);
        let w_fe = params.get(SYMBOLS.w_fe);

        let equation = BS9999Chapter15Figure6b::new(
            w_fe.clone(),
            s_up.clone(),
            s_dn.clone(),
            b.clone(),
            d.clone(),
            x.clone(),
        );

        let mut step_1 = FormStep::new(
            "Input | Fig. 6b",
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

        let s_dn = ParamBuilder::float(SYMBOLS.s_dn)
            .name("Stair width for downward portion of the stair")
            .units("mm")
            .min(0.0)
            .default_value(Some(ParameterValue::Float(1000.0)))
            .required()
            .build();

        let b = ParamBuilder::float(SYMBOLS.b)
            .name("Number of people served by the stair from below the final exit level")
            .min(0.0)
            .required()
            .build();

        let d = ParamBuilder::float(SYMBOLS.d)
            .name("Distance from the nose of the top going of the downward portion of the stair")
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
        params.add(s_dn);
        params.add(b);
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
        let s_dn = params.get(SYMBOLS.s_dn);
        let b = params.get(SYMBOLS.b);
        let d = params.get(SYMBOLS.d);
        let x = params.get(SYMBOLS.x);
        let w_fe = params.get(SYMBOLS.w_fe);

        let equation = BS9999Chapter15Figure6b::new(
            w_fe.clone(),
            s_up.clone(),
            s_dn.clone(),
            b.clone(),
            d.clone(),
            x.clone(),
        );

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));
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
        let s_dn = method.parameters.get(SYMBOLS.s_dn).as_float();
        let b = method.parameters.get(SYMBOLS.b).as_float();
        let d = method.parameters.get(SYMBOLS.d).as_float();
        let x = method.parameters.get(SYMBOLS.x).as_float();

        let w_fe = method.parameters.get(SYMBOLS.w_fe);

        let result = bs9999_figure6b::calculate_exit_width(b, d, s_up, s_dn, x);
        w_fe.update(Some(result.to_string()))?;

        return Ok(());
    }
}

#[derive(Debug)]
pub struct BS9999Chapter15Figure6b {
    w_fe: ArcParameter,
    s_up: ArcParameter,
    s_dn: ArcParameter,
    b: ArcParameter,
    d: ArcParameter,
    x: ArcParameter,
}

impl BS9999Chapter15Figure6b {
    pub fn new(
        w_fe: ArcParameter,
        s_up: ArcParameter,
        s_dn: ArcParameter,
        b: ArcParameter,
        d: ArcParameter,
        x: ArcParameter,
    ) -> Self {
        BS9999Chapter15Figure6b {
            w_fe,
            s_up,
            s_dn,
            b,
            d,
            x,
        }
    }
}

impl BS9999Chapter15Figure6b {
    pub fn dependencies(&self) -> Vec<ArcParameter> {
        vec![
            self.s_up.clone(),
            self.s_dn.clone(),
            self.b.clone(),
            self.d.clone(),
            self.w_fe.clone(),
            self.x.clone(),
        ]
    }

    pub fn input(&self) -> Vec<Dependency> {
        vec![
            self.s_up.clone().into(),
            self.s_dn.clone().into(),
            self.b.clone().into(),
            self.d.clone().into(),
            self.x.clone().into(),
        ]
    }

    pub fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "W_{{FE}} = {}",
            equation_1(self.s_up.symbol(), self.s_dn.symbol()),
        );
        let cond_1 = "\\text{if: B > 60 and D < 2 m}".to_string();

        let eq_2 = format!(
            "W_{{FE}} = {}",
            equation_2(self.b.symbol(), self.x.symbol(), self.s_up.symbol(),),
        );
        let cond_2 = "B < 60 \\space and/or  \\space D > 2 m".to_string();

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
        let d = self.d.as_float();
        let result = if b > 60.0 && d < 2.0 {
            let cond = format!("B = {} \\quad and \\quad D = {}  \\quad", b, d);
            let eq = format!(
                "W_{{FE}} = {} + {}",
                self.s_up.as_float(),
                self.s_dn.as_float(),
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
            let cond = format!("B = {} \\quad and \\quad D = {}", b, d);

            let eq = format!(
                "W_{{FE}} = {} \\cdot {} + 0.75 \\cdot {}",
                self.b.as_float(),
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

fn equation_1(s_up: String, s_dn: String) -> String {
    format!("{} + {}", s_up, s_dn)
}

fn equation_2(b: String, x: String, s_up: String) -> String {
    format!("{} \\cdot {} + 0.75 \\cdot {}", b, x, s_up)
}
