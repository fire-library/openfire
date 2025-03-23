use framework::method::calculation::CalculationComponent;
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;

#[derive(Debug)]
pub struct Psi {
    pub psi: ArcParameter,
    pub a_t: ArcParameter,
    pub a_v: ArcParameter,
    pub m_e: ArcParameter,
}

impl Psi {
    pub fn new_boxed(
        psi: ArcParameter,
        a_t: ArcParameter,
        a_v: ArcParameter,
        m_e: ArcParameter,
    ) -> Box<Self> {
        Box::new(Self { psi, a_t, a_v, m_e })
    }

    pub fn psi_equation(m_e: String, a_t: String, a_v: String) -> String {
        format!(
            "\\dfrac{{{}}}{{\\left[{} \\cdot {}\\right]^{{0.5}}}}",
            m_e, a_v, a_t
        )
    }
}

impl Psi {
    pub fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\Psi = {}",
            Self::psi_equation(self.m_e.symbol(), self.a_v.symbol(), self.a_t.symbol(),)
        );

        vec![vec![CalculationComponent::Equation(eq_1)]]
    }
    pub fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\Psi = {}",
            Self::psi_equation(
                self.m_e.display_value(),
                self.a_v.display_value(),
                self.a_t.display_value(),
            )
        );

        vec![vec![CalculationComponent::EquationWithResult(
            eq_1,
            self.psi.clone(),
        )]]
    }
}
