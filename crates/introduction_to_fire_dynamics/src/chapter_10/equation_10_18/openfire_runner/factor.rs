use framework::method::calculation::CalculationComponent;
use framework::method::parameter::ArcParameter;
use framework::method::parameter::ParameterTrait;

#[derive(Debug)]
pub struct Factor {
    pub f: ArcParameter,
    pub rho: ArcParameter,
    pub g: ArcParameter,
    pub a_w: ArcParameter,
    pub h: ArcParameter,
    pub a_f: ArcParameter,
}

impl Factor {
    pub fn new_boxed(
        f: ArcParameter,
        rho: ArcParameter,
        g: ArcParameter,
        a_w: ArcParameter,
        h: ArcParameter,
        a_f: ArcParameter,
    ) -> Box<Self> {
        Box::new(Self {
            f,
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

impl Factor {
    pub fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>> {
        let eq_1 = format!(
            "F = {}",
            Self::regime_equation(
                self.rho.symbol(),
                self.g.symbol(),
                self.a_w.symbol(),
                self.h.symbol(),
                self.a_f.symbol(),
            )
        );

        vec![vec![CalculationComponent::Equation(eq_1.clone())]]
    }
    pub fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>> {
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

        vec![vec![CalculationComponent::EquationWithResult(
            eq_1,
            self.f.clone(),
        )]]
    }
}
