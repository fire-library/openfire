use super::{calculation::CalculationComponent, equation::Dependency, parameter::ArcParameter};
use crate::method::parameter::ParameterTrait;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct Step {
    pub name: String,
    pub nomenclature: Vec<ArcParameter>,
    pub input: Vec<Dependency>,
    pub process: Vec<Vec<CalculationComponent>>,
    pub calculation: Vec<Vec<CalculationComponent>>,
    pub render: bool,
}

impl Step {
    pub fn nomenclature(&self) -> Vec<ArcParameter> {
        let mut deps = self.nomenclature.clone();
        deps.sort_by_key(|d| d.symbol());
        deps.dedup_by(|first, second| first.symbol() == second.symbol());

        deps.sort_by_key(|f| f.id());

        deps
    }

    pub fn get_input(&self) -> Vec<Dependency> {
        self.input.clone()
    }
}
