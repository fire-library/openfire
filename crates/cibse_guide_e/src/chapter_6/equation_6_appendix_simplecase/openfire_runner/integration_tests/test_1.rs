use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 1".to_string(),
        description: include_str!("test_1/description.md").to_string(),
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
            ],
        },
        assertions: vec![Assertion::FloatEqual("A_f".to_string(), 17.5)],
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
