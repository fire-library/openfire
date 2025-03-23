use framework::method::calculation::CalculationComponent;
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;

#[derive(Debug)]
pub struct Omega {
    pub omega: ArcParameter,
    pub a_t: ArcParameter,
    pub a_v: ArcParameter,
    pub h_v: ArcParameter,
}

impl Omega {
    pub fn new_boxed(
        omega: ArcParameter,
        a_t: ArcParameter,
        a_v: ArcParameter,
        h_v: ArcParameter,
    ) -> Box<Self> {
        Box::new(Self {
            omega,
            a_t,
            a_v,
            h_v,
        })
    }

    pub fn omega_equation(a_t: String, a_v: String, h_v: String) -> String {
        format!("\\dfrac{{{}}}{{{} \\sqrt{{{}}}}}", a_t, a_v, h_v)
    }
}

impl Omega {
    pub fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\Omega = {}",
            Self::omega_equation(self.a_t.symbol(), self.a_v.symbol(), self.h_v.symbol(),)
        );

        vec![vec![CalculationComponent::Equation(eq_1)]]
    }
    pub fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\Omega = {}",
            Self::omega_equation(
                self.a_t.display_value(),
                self.a_v.display_value(),
                self.h_v.display_value(),
            )
        );

        vec![vec![CalculationComponent::EquationWithResult(
            eq_1,
            self.omega.clone(),
        )]]
    }
}
