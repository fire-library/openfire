use framework::method::calculation::CalculationComponent;
use framework::method::equation::Dependency;
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;

#[derive(Debug)]
pub struct QMaxFC {
    pub q_max: ArcParameter,
    pub a_f: ArcParameter,
    pub hrrpua: ArcParameter,
}

impl QMaxFC {
    pub fn new_boxed(q_max: ArcParameter, a_f: ArcParameter, hrrpua: ArcParameter) -> Box<Self> {
        Box::new(Self { a_f, hrrpua, q_max })
    }

    pub fn q_max_fc_equation(a_f: String, hrrpua: String) -> String {
        format!("{} \\cdot {}", a_f, hrrpua)
    }
}

impl QMaxFC {
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
