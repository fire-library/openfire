use crate::domain::method::calculation::CalculationComponent;
use crate::domain::method::equation::Equation;
use crate::domain::method::parameter::ArcParameter;
use crate::domain::method::parameter::ParameterTrait;

#[derive(Debug)]
pub struct QMax {
    pub a_v: ArcParameter,
    pub h_v: ArcParameter,
}

impl QMax {
    pub fn new_boxed(a_v: ArcParameter, h_v: ArcParameter) -> Box<Self> {
        Box::new(Self { a_v, h_v })
    }

    pub fn q_max_equation(a_v: String, h_v: String) -> String {
        format!("1500 \\cdot {} \\cdot {}^{{1/2}}", a_v, h_v)
    }
}

impl Equation for QMax {
    fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\dot{{Q}}_{{max}} = {}",
            Self::q_max_equation(self.a_v.id(), self.h_v.id(),)
        );

        vec![vec![CalculationComponent::Equation(eq_1)]]
    }
    fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\dot{{Q}}_{{max}} = {}",
            Self::q_max_equation(self.a_v.display_value(), self.h_v.display_value(),)
        );

        vec![vec![CalculationComponent::EquationWithResult(eq_1)]]
    }

    fn dependencies(&self) -> Vec<ArcParameter> {
        vec![self.a_v.clone(), self.h_v.clone()]
    }
}
