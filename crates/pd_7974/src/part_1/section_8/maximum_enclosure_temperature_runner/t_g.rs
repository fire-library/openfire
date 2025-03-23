use framework::method::calculation::CalculationComponent;
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;

#[derive(Debug)]
pub struct TG {
    pub t_g: ArcParameter,
    pub psi: ArcParameter,
    pub t_g_max: ArcParameter,
}

impl TG {
    pub fn new_boxed(t_g: ArcParameter, psi: ArcParameter, t_g_max: ArcParameter) -> Box<Self> {
        Box::new(Self { psi, t_g_max, t_g })
    }

    pub fn t_g_equation(t_g_max: String, psi: String) -> String {
        format!("{} \\left(1 - e^{{-0.05 \\cdot {}}}\\right)", t_g_max, psi)
    }
}
impl TG {
    pub fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_2 = format!(
            "T_{{g}} = {}",
            Self::t_g_equation(self.t_g_max.symbol(), self.psi.symbol())
        );

        vec![vec![CalculationComponent::Equation(eq_2)]]
    }
    pub fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_2 = format!(
            "T_{{g}} = {}",
            Self::t_g_equation(self.t_g_max.display_value(), self.psi.display_value())
        );

        vec![vec![CalculationComponent::EquationWithResult(
            eq_2,
            self.t_g.clone(),
        )]]
    }
}
