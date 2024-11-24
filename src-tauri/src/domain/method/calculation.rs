use super::step::Step;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::{Arc, RwLock};

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct Calculation {
    pub steps: Vec<Step>,
    pub stale: bool,
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum CalculationComponent {
    Equation(String),
    EquationWithResult(String),
    Text(String),
}

pub type ArcCalculation = Arc<RwLock<Calculation>>;

impl Calculation {
    pub fn new() -> Self {
        Self {
            steps: vec![],
            stale: false,
        }
    }

    pub fn add_step(&mut self, step: Step) {
        self.steps.push(step);
    }
}
