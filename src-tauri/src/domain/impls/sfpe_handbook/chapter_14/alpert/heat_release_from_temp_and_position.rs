use crate::domain::impls::tag::Tag;
use crate::domain::method::builder::MethodBuilderTrait;
use crate::domain::method::calculation::{Calculation, CalculationComponent};
use crate::domain::method::equation::Equation;
use crate::domain::method::form::{Form, FormStep};
use crate::domain::method::parameter::builder::ParameterBuilder;
use crate::domain::method::parameter::ArcParameter;
use crate::domain::method::parameter::ParameterValue;
use crate::domain::method::parameter::Parameters;
use crate::domain::method::parameter::{ParameterTrait, ParametersTrait};
use crate::domain::method::{step::Step, Method};
use crate::domain::method::{MethodType, Reference};
use sfpe_handbook::chapter_14::alpert;
use std::sync::{Arc, RwLock};

pub struct AlpertHeatReleaseFromTempAndPositionBuilder;

use super::super::super::super::Document;
use super::super::super::SFPEHandbookChapter;
use super::super::Chapter14Method;

impl MethodBuilderTrait for AlpertHeatReleaseFromTempAndPositionBuilder {
    fn name() -> String {
        "HRR for heat detector response".to_string()
    }
    fn tags() -> Vec<Tag> {
        vec![Tag::SprinklerActivation, Tag::HRR, Tag::FireDynamics]
    }
    fn description() -> Option<String> {
        Some(
            "Calculates the heat release at which a heat detector submerded in the ceiling jet will activate"
                .to_string(),
        )
    }
    fn quick_calc_compatible() -> bool {
        true
    }
    fn reference() -> Reference {
        Reference(Document::SFPEHandbook(Some(SFPEHandbookChapter::Fourteen(
            Chapter14Method::HeatReleaseFromTempAndPosition,
        ))))
    }
    fn form(params: &Parameters) -> crate::domain::method::form::Form {
        let mut fields = vec![];
        for param in params.values().into_iter() {
            if param.read().unwrap().id == "\\dot{Q}" {
                continue;
            }
            fields.push(param.to_field())
        }
        let step_1 = FormStep {
            name: "Ceiling Jet Correlation Input".to_string(),
            description: "Uses Alpert's original correlation to calculate HRR for activation of a ceiling-mounted heat detector".to_string(),
            fields: fields,
            introduction: vec![],
        };

        Form {
            steps: vec![step_1],
        }
    }
    fn parameters() -> Parameters {
        let mut params = Parameters::new();

        let temp = ParameterBuilder::float("T")
            .name("Temperature at position of interest")
            .units("^{o}C")
            .default_value(Some(ParameterValue::Float(0.0)))
            .min(0.0)
            .required()
            .build();

        let temp_amb = ParameterBuilder::float("T_\\infty")
            .name("Ambient Temperature")
            .units("^{o}C")
            .default_value(Some(ParameterValue::Float(0.0)))
            .min(0.0)
            .required()
            .less_than_or_equal_to_parameter(&temp)
            .build();

        let h = ParameterBuilder::float("H")
            .name("Ceiling height")
            .units("m")
            .default_value(Some(ParameterValue::Float(0.0)))
            .min(0.0)
            .required()
            .build();

        let r = ParameterBuilder::float("r")
            .name("Radial position")
            .units("m")
            .min(0.0)
            .default_value(Some(ParameterValue::Float(0.0)))
            .required()
            .build();

        let q = ParameterBuilder::float("\\dot{Q}")
            .name("Heat release rate")
            .expression(Box::new(AlpertHeatReleaseFromTempAndPosition::new(
                temp.clone(),
                temp_amb.clone(),
                h.clone(),
                r.clone(),
            )))
            .units("kW")
            .build();

        params.add(temp);
        params.add(temp_amb);
        params.add(h);
        params.add(r);
        params.add(q);

        return params;
    }

    fn calc_sheet(params: &Parameters) -> crate::domain::method::calculation::ArcCalculation {
        let q = params.get_parameter("\\dot{Q}");
        let calc_sheet: Arc<RwLock<Calculation>> = Arc::new(RwLock::new(Calculation::new()));
        let step = Step {
            name: "Calculate Heat Release Rate from Point of Interest".to_string(),
            parameters: vec![q],
        };
        calc_sheet.write().unwrap().add_step(step);

        calc_sheet
    }

    fn method_type() -> MethodType {
        MethodType::SFPEAlpertHeatReleaseFromTemperatureAndPosition
    }
}

pub fn evaluate(method: &mut Method) -> Result<(), String> {
    let temp = method.parameters.get_parameter("T").as_float();
    let temp_amb = method.parameters.get_parameter("T_\\infty").as_float();
    let h = method.parameters.get_parameter("H").as_float();
    let r = method.parameters.get_parameter("r").as_float();

    let q = method.parameters.get_parameter("\\dot{Q}");

    let result = alpert::heat_release::from_temperature_and_position(temp, temp_amb, h, r);
    q.write().unwrap().value = Some(ParameterValue::Float(result));

    return Ok(());
}

#[derive(Debug)]
pub struct AlpertHeatReleaseFromTempAndPosition {
    pub temp: ArcParameter,
    pub temp_amb: ArcParameter,
    pub height: ArcParameter,
    pub radial_position: ArcParameter,
}

impl AlpertHeatReleaseFromTempAndPosition {
    pub fn new(
        temp: ArcParameter,
        temp_amb: ArcParameter,
        height: ArcParameter,
        radial_position: ArcParameter,
    ) -> Self {
        Self {
            temp,
            temp_amb,
            height,
            radial_position,
        }
    }
}

impl Equation for AlpertHeatReleaseFromTempAndPosition {
    fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\dot{{Q}} = {}",
            equation_1(
                self.temp.read().unwrap().id.clone(),
                self.temp_amb.read().unwrap().id.clone(),
                self.height.read().unwrap().id.clone(),
            ),
        );
        let cond_1 = "\\text{if: } \\dfrac{r}{H} \\le 0.18".to_string();

        let eq_2 = format!(
            "\\dot{{Q}} = {}",
            equation_2(
                self.temp.read().unwrap().id.clone(),
                self.temp_amb.read().unwrap().id.clone(),
                self.height.read().unwrap().id.clone(),
                self.radial_position.read().unwrap().id.clone(),
            ),
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
    fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let r = self.radial_position.as_float();
        let h = self.height.as_float();
        let result = if r / h <= 0.18 {
            let cond = format!(
                "\\dfrac{{r}}{{H}} = \\dfrac{{{}}}{{{}}} = {} \\le 0.18",
                r,
                h,
                r / h
            );
            let eq = format!(
                "\\dot{{Q}} = \\left(({}-{}) \\cdot \\dfrac{{{}^{{\\frac{{5}}{{3}}}}}}{{16.9}}\\right)^{{\\frac{{3}}{{2}}}}",
                self.temp.as_float(),
                self.temp_amb.as_float(),
                self.height.as_float(),
            );
            vec![
                vec![CalculationComponent::Equation(cond)],
                vec![(CalculationComponent::Equation("Therefore: ".to_string()))],
                vec![CalculationComponent::EquationWithResult(eq)],
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
                    self.temp.read().unwrap().id.clone(),
                    self.temp_amb.read().unwrap().id.clone(),
                    self.height.read().unwrap().id.clone(),
                    self.radial_position.read().unwrap().id.clone(),
                ),
                equation_2(
                    self.temp.display_value(),
                    self.temp_amb.display_value(),
                    self.height.display_value(),
                    self.radial_position.display_value(),
                ),
            );
            vec![
                vec![CalculationComponent::Equation(cond)],
                vec![CalculationComponent::Text("Therefore: ".to_string())],
                vec![CalculationComponent::EquationWithResult(eq)],
            ]
        };
        return result;
    }

    fn dependencies(&self) -> Vec<ArcParameter> {
        vec![
            self.temp.clone(),
            self.temp_amb.clone(),
            self.height.clone(),
            self.radial_position.clone(),
        ]
    }
}

pub fn equation_1(t: String, t_inf: String, h: String) -> String {
    format!(
        "\\left(({}-{}) \\cdot \\dfrac{{{}^{{\\frac{{5}}{{3}}}}}}{{16.9}}\\right)^{{\\frac{{3}}{{2}}}}",
        t,
        t_inf,
        h
    )
}

pub fn equation_2(t: String, t_inf: String, h: String, r: String) -> String {
    format!(
        "\\left(\\dfrac{{({}-{}) \\cdot \\left(\\dfrac{{{}}}{{{}}}\\right)^{{\\frac{{2}}{{3}}}} \\cdot {}^{{\\frac{{5}}{{3}}}}}}{{5.38}}\\right)^{{\\frac{{3}}{{2}}}}",
        t,
        t_inf,
        r,
        h,
        h
    )
}
