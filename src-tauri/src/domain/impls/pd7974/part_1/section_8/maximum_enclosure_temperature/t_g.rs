use crate::domain::method::calculation::CalculationComponent;
use crate::domain::method::equation::Equation;
use crate::domain::method::parameter::ArcParameter;
use crate::domain::method::parameter::ParameterTrait;

#[derive(Debug)]
pub struct TG {
    pub psi: ArcParameter,
    pub t_g_max: ArcParameter,
}

impl TG {
    pub fn new_boxed(psi: ArcParameter, t_g_max: ArcParameter) -> Box<Self> {
        Box::new(Self { psi, t_g_max })
    }

    pub fn t_g_equation(t_g_max: String, psi: String) -> String {
        format!("{} \\left(1 - e^{{-0.05 \\cdot {}}}\\right)", t_g_max, psi)
    }
}
impl Equation for TG {
    fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_2 = format!(
            "T_{{g}} = {}",
            Self::t_g_equation(
                self.t_g_max.read().unwrap().id.clone(),
                self.psi.read().unwrap().id.clone()
            )
        );

        vec![vec![CalculationComponent::Equation(eq_2)]]
    }
    fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_2 = format!(
            "T_{{g}} = {}",
            Self::t_g_equation(self.t_g_max.display_value(), self.psi.display_value())
        );

        vec![vec![CalculationComponent::EquationWithResult(eq_2)]]
    }

    fn dependencies(&self) -> Vec<ArcParameter> {
        vec![self.t_g_max.clone()]
    }
}
