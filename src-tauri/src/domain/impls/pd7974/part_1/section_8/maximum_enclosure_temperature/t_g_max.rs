use crate::domain::method::calculation::CalculationComponent;
use crate::domain::method::equation::Equation;
use crate::domain::method::parameter::ArcParameter;
use crate::domain::method::parameter::ParameterTrait;

#[derive(Debug)]
pub struct TGMax {
    pub omega: ArcParameter,
}

impl TGMax {
    pub fn new_boxed(omega: ArcParameter) -> Box<Self> {
        Box::new(Self { omega })
    }

    pub fn t_max_equation(omega: String) -> String {
        format!(
            "6000 \\dfrac{{\\left(1 - e^{{-0.1 \\cdot {}}}  \\right)}}{{\\sqrt{{{}}}}}",
            omega, omega
        )
    }
}

impl Equation for TGMax {
    fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_2 = format!(
            "T_{{g(max)}} = {}",
            Self::t_max_equation(self.omega.read().unwrap().id.clone(),)
        );

        vec![vec![CalculationComponent::Equation(eq_2)]]
    }
    fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_2 = format!(
            "T_{{g(max)}} = {}",
            Self::t_max_equation(self.omega.as_float().to_string())
        );

        vec![vec![CalculationComponent::EquationWithResult(eq_2)]]
    }

    fn dependencies(&self) -> Vec<ArcParameter> {
        vec![]
    }
}
