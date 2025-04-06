use framework::method::calculation::CalculationComponent;
use framework::method::equation::Dependency;
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;

#[derive(Debug)]
pub struct QMaxVC {
    pub q_max: ArcParameter,
    pub a_v: ArcParameter,
    pub h_v: ArcParameter,
}

impl QMaxVC {
    pub fn new_boxed(q_max: ArcParameter, a_v: ArcParameter, h_v: ArcParameter) -> Box<Self> {
        Box::new(Self { a_v, h_v, q_max })
    }

    pub fn q_max_equation(a_v: String, h_v: String) -> String {
        format!("1500 \\cdot {} \\cdot {}^{{1/2}}", a_v, h_v)
    }
}

impl QMaxVC {
    pub fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\dot{{Q}}_{{max, \\space Kawagoe}} = {}",
            Self::q_max_equation(self.a_v.symbol(), self.h_v.symbol(),)
        );

        vec![vec![CalculationComponent::Equation(eq_1)]]
    }
    pub fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\dot{{Q}}_{{max, \\space Kawagoe}} = {}",
            Self::q_max_equation(self.a_v.display_value(), self.h_v.display_value(),)
        );

        vec![vec![CalculationComponent::EquationWithResult(
            eq_1,
            self.q_max.clone(),
        )]]
    }

    pub fn input(&self) -> Vec<Dependency> {
        vec![self.a_v.clone().into(), self.h_v.clone().into()]
    }

    pub fn dependencies(&self) -> Vec<ArcParameter> {
        vec![self.a_v.clone(), self.h_v.clone(), self.q_max.clone()]
    }
}
