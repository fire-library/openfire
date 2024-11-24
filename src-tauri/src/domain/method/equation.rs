use std::fmt::Debug;

use super::{calculation::CalculationComponent, parameter::ArcParameter};

pub trait Equation: Debug + Send + Sync {
    fn generate_with_symbols(&self) -> Vec<Vec<CalculationComponent>>;
    fn generate_with_values(&self) -> Vec<Vec<CalculationComponent>>;
    fn dependencies(&self) -> Vec<ArcParameter>;
}
