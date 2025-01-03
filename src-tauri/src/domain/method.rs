pub mod builder;
pub mod calculation;
pub mod equation;
pub mod form;
pub mod parameter;
pub mod step;
pub mod validation;

use builder::MethodBuilderTrait;
use calculation::ArcCalculation;
use parameter::ParameterType;
use parameter::ParameterValue;
use parameter::Parameters;
use serde::{Deserialize, Serialize};
use specta::Type;
use validation::ParameterError;

use super::filesystem::saved_method::SavedMethod;
use super::impls::br187;
use super::impls::introduction_to_fire_dynamics;
use super::impls::pd7974;
use super::impls::sfpe_handbook;
use super::impls::Document;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct Method {
    pub name: String,
    pub description: Option<String>,
    pub reference: Reference,
    pub method_type: MethodType,
    pub parameters: Parameters,
    pub quick_calc_compatible: bool,
    pub calc_sheet: ArcCalculation,
    pub form: form::Form,
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct Reference(pub Document);

impl Method {
    pub fn evaluate(&mut self) -> Result<(), ParameterError> {
        match &self.method_type {
            MethodType::BR187Chapter1Equation1 => br187::chapter_1::equation_1::evaluate(self)?,
            MethodType::SFPEAlpertHeatReleaseFromTemperatureAndPosition => {
                sfpe_handbook::chapter_14::alpert::heat_release_from_temp_and_position::evaluate(
                    self,
                )?
            }
            MethodType::PD7974Part1Section8MaximumEnclosureTemperature => {
                pd7974::part_1::section_8::maximum_enclosure_temperature::evaluate(self)?
            }
            MethodType::PD7974Part1Section8HRRAtFlashover => {
                pd7974::part_1::section_8::hrr_at_flashover::evaluate(self)?
            }
            MethodType::IntroductionToFireDynamcicsChapter10BurningRegime => {
                introduction_to_fire_dynamics::chapter_10::burning_regime::evaluate(self)?
            }
        };

        self.calc_sheet.write().unwrap().stale = false;

        Ok(())
    }
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum MethodType {
    PD7974Part1Section8MaximumEnclosureTemperature,
    PD7974Part1Section8HRRAtFlashover,
    BR187Chapter1Equation1,
    SFPEAlpertHeatReleaseFromTemperatureAndPosition,
    IntroductionToFireDynamcicsChapter10BurningRegime,
}

impl MethodType {
    pub fn method(&self) -> Method {
        match &self {
            &MethodType::BR187Chapter1Equation1 => br187::chapter_1::equation_1::BR187Chapter1Equation1Builder::build_method(),
            &MethodType::SFPEAlpertHeatReleaseFromTemperatureAndPosition => {
                sfpe_handbook::chapter_14::alpert::heat_release_from_temp_and_position::AlpertHeatReleaseFromTempAndPositionBuilder::build_method()
            },
            &MethodType::PD7974Part1Section8MaximumEnclosureTemperature => {
                pd7974::part_1::section_8::maximum_enclosure_temperature::MaximumEnclosureTemperatureBuilder::build_method()
            }
            &MethodType::PD7974Part1Section8HRRAtFlashover => {
                pd7974::part_1::section_8::hrr_at_flashover::HRRAtFlashoverBuilder::build_method()
            }
            &MethodType::IntroductionToFireDynamcicsChapter10BurningRegime => {
                introduction_to_fire_dynamics::chapter_10::burning_regime::BurningRegimeBuilder::build_method()
            }
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            &MethodType::BR187Chapter1Equation1 => {
                return "BR187 | Chapter 1 | Equation 1".to_string()
            }
            &MethodType::SFPEAlpertHeatReleaseFromTemperatureAndPosition => {
                return "SFPE Handbook | Heat Release From Temperature & Position".to_string()
            }
            &MethodType::PD7974Part1Section8MaximumEnclosureTemperature => {
                return "PD7974 | Part 1 | Section 8 | Maximum Enclosure Temperature".to_string()
            }
            &MethodType::PD7974Part1Section8HRRAtFlashover => {
                return "PD7974 | Part 1 | Section 8 | HRR at Flashover".to_string()
            }
            &MethodType::IntroductionToFireDynamcicsChapter10BurningRegime => {
                return "Introduction to Fire Dynamics | Chapter 10 | Burning Regime".to_string()
            }
        }
    }
}

impl From<SavedMethod> for Method {
    fn from(saved: SavedMethod) -> Self {
        let method = match saved.method_type {
            MethodType::BR187Chapter1Equation1 => br187::chapter_1::equation_1::BR187Chapter1Equation1Builder::build_method(),
            MethodType::SFPEAlpertHeatReleaseFromTemperatureAndPosition => {
                sfpe_handbook::chapter_14::alpert::heat_release_from_temp_and_position::AlpertHeatReleaseFromTempAndPositionBuilder::build_method()
            },
            MethodType::PD7974Part1Section8MaximumEnclosureTemperature => {
                pd7974::part_1::section_8::maximum_enclosure_temperature::MaximumEnclosureTemperatureBuilder::build_method()
            }
            MethodType::PD7974Part1Section8HRRAtFlashover => {
                pd7974::part_1::section_8::hrr_at_flashover::HRRAtFlashoverBuilder::build_method()
            }
            MethodType::IntroductionToFireDynamcicsChapter10BurningRegime => {
                introduction_to_fire_dynamics::chapter_10::burning_regime::BurningRegimeBuilder::build_method()
            }
        };

        for param in saved.parameters {
            let mut p = method.parameters.get(&param.name).unwrap().write().unwrap();

            match (&mut *p, param.value) {
                (ParameterType::Float(float), Some(ParameterValue::Float(saved))) => {
                    float.value = Some(saved);
                }
                (ParameterType::String(string), Some(ParameterValue::String(saved))) => {
                    string.value = Some(saved);
                }
                _ => panic!("Parameter type mismatch"),
            }
        }

        method
    }
}
