use crate::domain::method::calculation::CalculationComponent;
use crate::domain::method::equation::Equation;
use crate::domain::method::parameter::ArcParameter;
use crate::domain::method::parameter::ParameterTrait;

#[derive(Debug)]
pub struct QFoThomas {
    pub a_t: ArcParameter,
    pub a_v: ArcParameter,
    pub h_v: ArcParameter,
}

impl QFoThomas {
    pub fn new_boxed(a_t: ArcParameter, a_v: ArcParameter, h_v: ArcParameter) -> Box<Self> {
        Box::new(Self { a_t, a_v, h_v })
    }

    pub fn q_fo_equation(a_t: String, a_v: String, h_v: String) -> String {
        format!(
            "7.8 \\cdot {} + 378 \\cdot {} \\cdot {}^{{1/2}}",
            a_t, a_v, h_v
        )
    }
}

impl Equation for QFoThomas {
    fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\dot{{Q}}_{{fo, \\space Thomas}} = {}",
            Self::q_fo_equation(
                self.a_t.read().unwrap().id.clone(),
                self.a_v.read().unwrap().id.clone(),
                self.h_v.read().unwrap().id.clone(),
            )
        );

        vec![vec![CalculationComponent::Equation(eq_1)]]
    }
    fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\dot{{Q}}_{{fo, \\space Thomas}} = {}",
            Self::q_fo_equation(
                self.a_t.display_value(),
                self.a_v.display_value(),
                self.h_v.display_value(),
            )
        );

        vec![vec![CalculationComponent::EquationWithResult(eq_1)]]
    }

    fn dependencies(&self) -> Vec<ArcParameter> {
        vec![self.a_t.clone(), self.a_v.clone(), self.h_v.clone()]
    }
}
