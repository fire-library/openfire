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
            "\\dot{{Q}}_{{max, \\space FC}} = {}",
            Self::q_max_fc_equation(self.a_f.symbol(), self.hrrpua.symbol(),)
        );

        vec![vec![CalculationComponent::Equation(eq_1)]]
    }
    pub fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\dot{{Q}}_{{max, \\space FC}} = {}",
            Self::q_max_fc_equation(self.a_f.display_value(), self.hrrpua.display_value(),)
        );

        vec![vec![CalculationComponent::EquationWithResult(
            eq_1,
            self.q_max.clone(),
        )]]
    }

    pub fn input(&self) -> Vec<Dependency> {
        vec![self.a_f.clone().into(), self.hrrpua.clone().into()]
    }

    pub fn dependencies(&self) -> Vec<ArcParameter> {
        vec![self.a_f.clone(), self.hrrpua.clone(), self.q_max.clone()]
    }
}
