pub mod integration_tests;

use crate::chapter_14::alpert;
use framework::method::calculation::{Calculation, CalculationComponent};
use framework::method::form::{Form, FormStep};
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;
use framework::method::parameter::builder::ParamBuilder;
use framework::method::parameters::Parameters;
use framework::method::runner::MethodRunner;
use framework::method::tag::Tag;
use framework::method::test::IntegrationTests;
use framework::method::validation::ParameterError;
use framework::method::{Method, step::Step};
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct AlpertHeatReleaseFromTempAndPositionBuilder;

use crate::SFPEHandbook;
use crate::chapter_14::Chapter14Method;

struct Symbols {
    temp: &'static str,
    temp_amb: &'static str,
    h: &'static str,
    r: &'static str,
    q: &'static str,
}

const SYMBOLS: Symbols = Symbols {
    temp: "T",
    temp_amb: "T_\\infty",
    h: "H",
    r: "r",
    q: "\\dot{Q}",
};

impl MethodRunner for AlpertHeatReleaseFromTempAndPositionBuilder {
    fn name(&self) -> String {
        "HRR for heat detector response".to_string()
    }
    fn reference(&self) -> &dyn framework::method::runner::Reference {
        &SFPEHandbook::ChapterFourteen(Chapter14Method::HeatReleaseFromTempAndPosition)
    }
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::SprinklerActivation, Tag::HRR, Tag::FireDynamics]
    }
    fn description(&self) -> Option<String> {
        Some(
            "Calculates the heat release at which a heat detector submerded in the ceiling jet will activate"
                .to_string(),
        )
    }
    fn quick_calc(&self, params: &Parameters) -> Option<Vec<ArcParameter>> {
        let q = params.get("\\dot{Q}");

        Some(vec![q])
    }

    fn form(&self, params: &Parameters) -> framework::method::form::Form {
        let temp = params.get(SYMBOLS.temp);
        let temp_amb = params.get(SYMBOLS.temp_amb);
        let h = params.get(SYMBOLS.h);
        let r = params.get(SYMBOLS.r);

        let mut step_1: FormStep = FormStep::new(
            "Input | Eq. 14.2 & 14.3",
            "Input required to calculate steady HRR for activation of a ceiling-mounted heat detector, using Alpert's original correlation.",
        );
        for param in params.values().into_iter() {
            if param.symbol() == "\\dot{Q}" {
                continue;
            }
            step_1.add_field(param.to_field())
        }

        step_1.add_intro();
        step_1.add_equation(
            process(temp.clone(), temp_amb.clone(), h.clone(), r.clone())[0][0].clone(),
        );
        step_1.add_text("or");
        step_1.add_equation(process(temp, temp_amb, h, r)[1][0].clone());

        Form::new(vec![step_1])
    }
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();

        let temp = ParamBuilder::float(SYMBOLS.temp)
            .name("Temperature at position of interest")
            .units("^{o}C")
            .min(0.0)
            .required()
            .build();

        let temp_amb = ParamBuilder::float(SYMBOLS.temp_amb)
            .name("Ambient Temperature")
            .units("^{o}C")
            .min(0.0)
            .required()
            .less_than_or_equal_to_parameter(&temp)
            .build();

        let h = ParamBuilder::float(SYMBOLS.h)
            .name("Ceiling height")
            .units("m")
            .min(0.0)
            .required()
            .build();

        let r = ParamBuilder::float(SYMBOLS.r)
            .name("Radial position")
            .units("m")
            .min(0.0)
            .required()
            .build();

        let q = ParamBuilder::float(SYMBOLS.q)
            .name("Heat release rate")
            .units("kW")
            .build();

        params.add(temp);
        params.add(temp_amb);
        params.add(h);
        params.add(r);
        params.add(q);

        return params;
    }

    fn calc_sheet(
        &self,
        params: &Parameters,
        stale: Option<bool>,
    ) -> framework::method::calculation::ArcCalculation {
        let q = params.get(SYMBOLS.q);
        let temp = params.get("T");
        let temp_amb = params.get("T_\\infty");
        let h = params.get("H");
        let r = params.get("r");

        let stale = stale.unwrap_or(false);
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new(stale)));
        let deps = vec![
            temp.clone(),
            temp_amb.clone(),
            h.clone(),
            r.clone(),
            q.clone(),
        ];
        let mut nomenclature = deps.clone();
        nomenclature.push(q.clone());

        let step = Step {
            name: "Calculate Heat Release Rate from Point of Interest".to_string(),
            nomenclature: nomenclature,
            input: deps.into_iter().map(|p| p.into()).collect(),
            process: process(temp.clone(), temp_amb.clone(), h.clone(), r.clone()),
            calculation: calculation(temp, temp_amb, h, r, q),
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
        let temp = method.parameters.get("T").as_float();
        let temp_amb = method.parameters.get("T_\\infty").as_float();
        let h = method.parameters.get("H").as_float();
        let r = method.parameters.get("r").as_float();

        let q = method.parameters.get("\\dot{Q}");

        let result = alpert::heat_release::from_temperature_and_position(temp, temp_amb, h, r);
        q.update(Some(result.to_string()))?;

        return Ok(());
    }
}

fn process(
    temp: ArcParameter,
    temp_amb: ArcParameter,
    h: ArcParameter,
    r: ArcParameter,
) -> Vec<Vec<CalculationComponent>> {
    let eq_1 = format!(
        "\\dot{{Q}} = {}",
        equation_1(temp.symbol(), temp_amb.symbol(), h.symbol(),),
    );
    let cond_1 = "\\text{if: } \\dfrac{r}{H} \\le 0.18".to_string();

    let eq_2 = format!(
        "\\dot{{Q}} = {}",
        equation_2(temp.symbol(), temp_amb.symbol(), h.symbol(), r.symbol(),),
    );
    let cond_2 = "\\text{if: } \\dfrac{r}{H} \\gt 0.18".to_string();

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
fn calculation(
    temp: ArcParameter,
    temp_amb: ArcParameter,
    height: ArcParameter,
    radial_position: ArcParameter,
    q: ArcParameter,
) -> Vec<Vec<CalculationComponent>> {
    let r = radial_position.as_float();
    let h = height.as_float();
    let result = if r / h <= 0.18 {
        let cond = format!(
            "\\dfrac{{r}}{{H}} = \\dfrac{{{}}}{{{}}} = {} \\le 0.18",
            r,
            h,
            r / h
        );
        let eq = format!(
            "\\dot{{Q}} = \\left(({}-{}) \\cdot \\dfrac{{{}^{{\\frac{{5}}{{3}}}}}}{{16.9}}\\right)^{{\\frac{{3}}{{2}}}}",
            temp.as_float(),
            temp_amb.as_float(),
            height.as_float(),
        );
        vec![
            vec![CalculationComponent::Equation(cond)],
            vec![(CalculationComponent::Equation("Therefore: ".to_string()))],
            vec![CalculationComponent::EquationWithResult(eq, q.clone())],
        ]
    } else {
        let cond = format!(
            "\\dfrac{{r}}{{H}} = \\dfrac{{{}}}{{{}}} = {} \\gt 0.18",
            r,
            h,
            r / h
        );

        let eq = format!(
            "\\dot{{Q}} = {} = {}",
            equation_2(
                temp.symbol().clone(),
                temp_amb.symbol().clone(),
                height.symbol().clone(),
                radial_position.symbol().clone(),
            ),
            equation_2(
                temp.display_value(),
                temp_amb.display_value(),
                height.display_value(),
                radial_position.display_value(),
            ),
        );
        vec![
            vec![CalculationComponent::Equation(cond)],
            vec![CalculationComponent::Text("Therefore: ".to_string())],
            vec![CalculationComponent::EquationWithResult(eq, q.clone())],
        ]
    };
    return result;
}

pub fn equation_1(t: String, t_inf: String, h: String) -> String {
    format!(
        "\\left(({}-{}) \\cdot \\dfrac{{{}^{{\\frac{{5}}{{3}}}}}}{{16.9}}\\right)^{{\\frac{{3}}{{2}}}}",
        t, t_inf, h
    )
}

pub fn equation_2(t: String, t_inf: String, h: String, r: String) -> String {
    format!(
        "\\left(\\dfrac{{({}-{}) \\cdot \\left(\\dfrac{{{}}}{{{}}}\\right)^{{\\frac{{2}}{{3}}}} \\cdot {}^{{\\frac{{5}}{{3}}}}}}{{5.38}}\\right)^{{\\frac{{3}}{{2}}}}",
        t, t_inf, r, h, h
    )
}
