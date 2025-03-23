use framework::method::calculation::CalculationComponent;
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;

#[derive(Debug)]
pub struct QFoMcCaffrey {
    pub q_fo_mccaffrey: ArcParameter,
    pub a_t: ArcParameter,
    pub a_v: ArcParameter,
    pub h_v: ArcParameter,
    pub h_k: ArcParameter,
}

impl QFoMcCaffrey {
    pub fn new_boxed(
        q_fo_mccaffrey: ArcParameter,
        a_t: ArcParameter,
        a_v: ArcParameter,
        h_v: ArcParameter,
        h_k: ArcParameter,
    ) -> Box<Self> {
        Box::new(Self {
            a_t,
            a_v,
            h_v,
            h_k,
            q_fo_mccaffrey,
        })
    }

    pub fn q_fo_equation(a_t: String, a_v: String, h_v: String, h_k: String) -> String {
        format!(
            "610 \\left({} \\cdot {} \\cdot {} \\cdot {}^{{1/2}} \\right)^{{1/2}}",
            h_k, a_t, a_v, h_v
        )
    }
}

impl QFoMcCaffrey {
    pub fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\dot{{Q}}_{{fo, \\space McCaffrey}} = {}",
            Self::q_fo_equation(
                self.a_t.symbol(),
                self.a_v.symbol(),
                self.h_v.symbol(),
                self.h_k.symbol(),
            )
        );

        vec![vec![CalculationComponent::Equation(eq_1)]]
    }

    pub fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "\\dot{{Q}}_{{fo, \\space McCaffrey}} = {}",
            Self::q_fo_equation(
                self.a_t.display_value(),
                self.a_v.display_value(),
                self.h_v.display_value(),
                self.h_k.display_value(),
            )
        );

        vec![vec![CalculationComponent::EquationWithResult(
            eq_1,
            self.q_fo_mccaffrey.clone(),
        )]]
    }

    pub fn input(&self) -> Vec<ArcParameter> {
        vec![
            self.a_t.clone(),
            self.a_v.clone(),
            self.h_v.clone(),
            self.h_k.clone(),
        ]
    }

    pub fn dependencies(&self) -> Vec<ArcParameter> {
        let mut input = self.input();
        input.push(self.q_fo_mccaffrey.clone());
        input
    }
}
