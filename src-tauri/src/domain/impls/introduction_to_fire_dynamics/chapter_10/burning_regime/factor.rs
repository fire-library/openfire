use crate::domain::method::calculation::CalculationComponent;
use crate::domain::method::equation::Equation;
use crate::domain::method::parameter::ArcParameter;
use crate::domain::method::parameter::ParameterTrait;

#[derive(Debug)]
pub struct Factor {
    pub rho: ArcParameter,
    pub g: ArcParameter,
    pub a_w: ArcParameter,
    pub h: ArcParameter,
    pub a_f: ArcParameter,
}

impl Factor {
    pub fn new_boxed(
        rho: ArcParameter,
        g: ArcParameter,
        a_w: ArcParameter,
        h: ArcParameter,
        a_f: ArcParameter,
    ) -> Box<Self> {
        Box::new(Self {
            rho,
            g,
            a_w,
            h,
            a_f,
        })
    }

    pub fn regime_equation(rho: String, g: String, a_w: String, h: String, a_f: String) -> String {
        format!(
            "\\dfrac{{{} \\cdot {}^{{1/2}} \\cdot {} \\cdot {}^{{1/2}}}}{{{}}}",
            rho, g, a_w, h, a_f
        )
    }
}

impl Equation for Factor {
    fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "F = {}",
            Self::regime_equation(
                self.rho.id(),
                self.g.id(),
                self.a_w.id(),
                self.h.id(),
                self.a_f.id(),
            )
        );

        vec![vec![CalculationComponent::Equation(eq_1.clone())]]
    }
    fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "F = {}",
            Self::regime_equation(
                self.rho.display_value(),
                self.g.display_value(),
                self.a_w.display_value(),
                self.h.display_value(),
                self.a_f.display_value(),
            )
        );

        vec![vec![CalculationComponent::EquationWithResult(eq_1)]]
    }

    fn dependencies(&self) -> Vec<ArcParameter> {
        vec![
            self.rho.clone(),
            self.g.clone(),
            self.a_w.clone(),
            self.h.clone(),
            self.a_f.clone(),
        ]
    }
}
