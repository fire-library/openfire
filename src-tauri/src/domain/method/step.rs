use super::parameter::ArcParameter;
use crate::domain::method::parameter::ParameterTrait;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct Step {
    pub name: String,
    pub parameters: Vec<ArcParameter>,
}

impl Step {
    pub fn get_dependencies(&self) -> Vec<ArcParameter> {
        let mut dependencies = vec![];
        for parameter in &self.parameters {
            dependencies.push(parameter.clone());
            dependencies.append(&mut &mut parameter.read().unwrap().get_dependencies());
        }
        dependencies.dedup_by(|first, second| first.id() == second.id());

        dependencies.sort_by_key(|f| f.id());

        dependencies
    }

    pub fn get_input(&self) -> Vec<ArcParameter> {
        let mut input = vec![];
        let mut top_level_params = vec![];
        for parameter in &self.parameters {
            input.push(parameter.clone());
            top_level_params.push(parameter.clone());
            input.append(&mut parameter.read().unwrap().get_dependencies());
        }

        let top_level_params: Vec<String> = top_level_params.into_iter().map(|p| p.id()).collect();

        input
            .into_iter()
            .filter(|p| {
                let id = p.id();
                !top_level_params.contains(&id)
            })
            .collect()
    }
}
