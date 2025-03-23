use framework::method::calculation::CalculationComponent;
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;

#[derive(Debug)]
pub struct TGMax {
    pub t_max: ArcParameter,
    pub omega: ArcParameter,
}

impl TGMax {
    pub fn new_boxed(t_max: ArcParameter, omega: ArcParameter) -> Box<Self> {
        Box::new(Self { t_max, omega })
    }

    pub fn t_max_equation(omega: String) -> String {
        format!(
            "6000 \\dfrac{{\\left(1 - e^{{-0.1 \\cdot {}}}  \\right)}}{{\\sqrt{{{}}}}}",
            omega, omega
        )
    }
}

impl TGMax {
    pub fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_2 = format!(
            "T_{{g(max)}} = {}",
            Self::t_max_equation(self.omega.symbol(),)
        );

        vec![vec![CalculationComponent::Equation(eq_2)]]
    }

    pub fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_2 = format!(
            "T_{{g(max)}} = {}",
            Self::t_max_equation(self.omega.display_value())
        );

        vec![vec![CalculationComponent::EquationWithResult(
            eq_2,
            self.t_max.clone(),
        )]]
    }
}
