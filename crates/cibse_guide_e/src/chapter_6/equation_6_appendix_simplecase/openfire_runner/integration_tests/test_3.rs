use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 3".to_string(),
        description: include_str!("test_3/description.md").to_string(),
        input: SavedMethod {
            id: super::super::Chapter6EquationAppendixSimpleCaseRunner.id(),
            parameters: vec![
                SavedParameter {
                    name: "w_1".to_string(),
                    value: Some(Float(3.5)),
                },
                SavedParameter {
                    name: "w_2".to_string(),
                    value: Some(Float(5.0)),
                },
                SavedParameter {
                    name: "w_o".to_string(),
                    value: Some(Float(1.5)),
                },
                SavedParameter {
                    name: "h_o".to_string(),
                    value: Some(Float(0.5)),
                },
                SavedParameter {
                    name: "h".to_string(),
                    value: Some(Float(3.0)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual("A_{net}".to_string(), 34.25)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::Chapter6EquationAppendixSimpleCaseRunner>(
        );
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
