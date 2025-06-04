use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::parameter::ParameterValue::Integer;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 1".to_string(),
        description: include_str!("test_1/description.md").to_string(),
        input: SavedMethod {
            id: super::super::Chapter7Equation3Runner.id(),
            parameters: vec![
                SavedParameter {
                    name: "P".to_string(),
                    value: Some(Integer(550)),
                },
                SavedParameter {
                    name: "n".to_string(),
                    value: Some(Integer(6)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual("w".to_string(), 1.38888888888889)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::Chapter7Equation3Runner>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
