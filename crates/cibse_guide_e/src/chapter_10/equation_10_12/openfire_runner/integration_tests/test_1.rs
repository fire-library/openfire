use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 1".to_string(),
        description: include_str!("test_1/description.md").to_string(),
        input: SavedMethod {
            id: super::super::Chapter10Equation12Runner.id(),
            parameters: vec![
                SavedParameter {
                    name: "K".to_string(),
                    value: Some(Float(1.0)),
                },
                SavedParameter {
                    name: "g".to_string(),
                    value: Some(Float(9.8)),
                },
                SavedParameter {
                    name: "Q".to_string(),
                    value: Some(Float(1000.0)),
                },
                SavedParameter {
                    name: "w".to_string(),
                    value: Some(Float(2.5)),
                },
                SavedParameter {
                    name: "\\rho".to_string(),
                    value: Some(Float(1.2)),
                },
                SavedParameter {
                    name: "c".to_string(),
                    value: Some(Float(0.9)),
                },
                SavedParameter {
                    name: "T".to_string(),
                    value: Some(Float(523.0)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual("v_e".to_string(), 1.90745166141768)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::Chapter10Equation12Runner>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
