use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 1".to_string(),
        description: include_str!("test_1/description.md").to_string(),
        input: SavedMethod {
            id: super::super::Chapter10Equation11Runner.id(),
            parameters: vec![
                SavedParameter {
                    name: "q".to_string(),
                    value: Some(Float(1000.0)),
                },
                SavedParameter {
                    name: "z".to_string(),
                    value: Some(Float(1.5)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual("v_e".to_string(), 0.49794086489969)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::Chapter10Equation11Runner>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
