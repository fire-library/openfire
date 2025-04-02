use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 1".to_string(),
        description: include_str!("test_1/description.md").to_string(),
        input: SavedMethod {
            id: super::super::Chapter10Equation10Runner.id(),
            parameters: vec![
                SavedParameter {
                    name: "g".to_string(),
                    value: Some(Float(9.8)),
                },
                SavedParameter {
                    name: "H".to_string(),
                    value: Some(Float(2.0)),
                },
                SavedParameter {
                    name: "T_f".to_string(),
                    value: Some(Float(773.0)),
                },
                SavedParameter {
                    name: "T_0".to_string(),
                    value: Some(Float(293.0)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual("v_e".to_string(), 2.26763036612251)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::Chapter10Equation10Runner>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
