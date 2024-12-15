use crate::domain::method::calculation::CalculationComponent;
use crate::domain::method::equation::Equation;
use crate::domain::method::parameter::ArcParameter;
use crate::domain::method::parameter::ParameterTrait;
use introduction_to_fire_dynamics::chapter_10::equation_10_18::BurningRegime;

#[derive(Debug)]
pub struct Regime {
    pub f: ArcParameter,
}

impl Regime {
    pub fn new_boxed(f: ArcParameter) -> Box<Self> {
        Box::new(Self { f })
    }
}

impl Equation for Regime {
    fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        vec![
            vec![
                CalculationComponent::Text("If: ".to_string()),
                CalculationComponent::Equation(format!("{} < 0.235", self.f.id())),
                CalculationComponent::Text(format!(
                    "Burning Regime = {}",
                    BurningRegime::VentilationControlled.to_string()
                )),
            ],
            vec![
                CalculationComponent::Text("If: ".to_string()),
                CalculationComponent::Equation(format!("{} > 0.290", self.f.id())),
                CalculationComponent::Text(format!(
                    "Burning Regime = {}",
                    BurningRegime::FuelControlled.to_string()
                )),
            ],
            vec![
                CalculationComponent::Text("Else: ".to_string()),
                CalculationComponent::Text(format!(
                    "Burning Regime = undefined / {}",
                    BurningRegime::Crossover.to_string()
                )),
            ],
        ]
    }
    fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        if self.f.as_float() < 0.235 {
            return vec![vec![
                CalculationComponent::Equation(format!("{} < 0.235", self.f.display_value())),
                CalculationComponent::Text(format!(
                    "Therefore: Burning Regime = {}",
                    BurningRegime::VentilationControlled.to_string()
                )),
            ]];
        } else if self.f.as_float() > 0.290 {
            return vec![vec![
                CalculationComponent::Equation(format!("{} > 0.290", self.f.display_value())),
                CalculationComponent::Text(format!(
                    "Therefore: Burning Regime = {}",
                    BurningRegime::FuelControlled.to_string()
                )),
            ]];
        } else {
            return vec![vec![
                CalculationComponent::Equation(format!(
                    "0.235 \\le {} \\le 0.290",
                    self.f.display_value()
                )),
                CalculationComponent::Text(format!(
                    "Therefore: Burning Regime = undefined / {}",
                    BurningRegime::Crossover.to_string()
                )),
            ]];
        }
    }

    fn dependencies(&self) -> Vec<ArcParameter> {
        vec![self.f.clone()]
    }
}
