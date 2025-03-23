use crate::filesystem::saved_method::SavedMethod;
use crate::method::Method;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::{
    parameter::{ArcParameter, ParameterTrait},
    validation::ParameterError,
};

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub enum Assertion {
    FloatEqual(String, f64),
    StringEqual(String, String),
}

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub enum AssertionResult {
    Equal(ArcParameter, bool),
}

impl AssertionResult {
    #[allow(dead_code)]
    pub fn has_passed(&self) -> bool {
        match self {
            AssertionResult::Equal(_, passed) => *passed,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct IntegrationTests {
    pub description: String,
    pub tests: Vec<Test>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct Test {
    pub name: String,
    pub description: String,
    pub input: SavedMethod,
    pub assertions: Vec<Assertion>,
}

pub fn run_test(test: Test) -> Result<Vec<AssertionResult>, Vec<ParameterError>> {
    let mut method = Method::try_from(test.input.clone())?;
    method.evaluate()?;

    let mut results = Vec::new();
    for assertion in test.assertions {
        match assertion {
            Assertion::FloatEqual(symbol, value) => {
                let parameter = method.parameters.get(&symbol);
                results.push(AssertionResult::Equal(
                    parameter.clone(),
                    ((parameter.as_float() - value).abs()) < 1e-10,
                ));
            }
            Assertion::StringEqual(symbol, value) => {
                let parameter = method.parameters.get(&symbol);
                results.push(AssertionResult::Equal(
                    parameter.clone(),
                    parameter.as_string() == value,
                ));
            }
        }
    }

    Ok(results)
}
