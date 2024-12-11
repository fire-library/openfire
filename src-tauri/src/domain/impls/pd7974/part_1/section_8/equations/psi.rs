use crate::domain::method::calculation::CalculationComponent;
use crate::domain::method::equation::Equation;
use crate::domain::method::parameter::ArcParameter;
use crate::domain::method::parameter::ParameterTrait;

#[derive(Debug)]
pub struct Psi {
    pub a_t: ArcParameter,
    pub a_v: ArcParameter,
    pub m_e: ArcParameter,
}

impl Psi {
    pub fn new_boxed(a_t: ArcParameter, a_v: ArcParameter, m_e: ArcParameter) -> Box<Self> {
        Box::new(Self { a_t, a_v, m_e })
    }

    pub fn psi_equation(m_e: String, a_t: String, a_v: String) -> String {
        format!(
            "\\dfrac{{{}}}{{\\left[{} \\cdot {}\\right]^{{0.5}}}}",
            m_e, a_v, a_t
        )
    }
}

impl Equation for Psi {
    fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\Psi = {}",
            Self::psi_equation(
                self.m_e.read().unwrap().id.clone(),
                self.a_v.read().unwrap().id.clone(),
                self.a_t.read().unwrap().id.clone(),
            )
        );

        vec![vec![CalculationComponent::Equation(eq_1)]]
    }
    fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\Psi = {}",
            Self::psi_equation(
                self.m_e.as_float().to_string(),
                self.a_v.as_float().to_string(),
                self.a_t.as_float().to_string(),
            )
        );

        vec![vec![CalculationComponent::EquationWithResult(eq_1)]]
    }

    fn dependencies(&self) -> Vec<ArcParameter> {
        vec![self.a_t.clone(), self.a_v.clone(), self.m_e.clone()]
    }
}
