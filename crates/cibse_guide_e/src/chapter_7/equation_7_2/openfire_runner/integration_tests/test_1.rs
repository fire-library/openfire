use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 1".to_string(),
        description: include_str!("test_1/description.md").to_string(),
        input: SavedMethod {
            id: super::super::Chapter7Equation2Runner.id(),
            parameters: vec![
                SavedParameter {
                    name: "w".to_string(),
                    value: Some(Float(1.2)),
                },
                SavedParameter {
                    name: "n".to_string(),
                    value: Some(Float(6.0)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual("P".to_string(), 465.0)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::Chapter7Equation2Runner>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
