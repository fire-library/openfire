use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 1".to_string(),
        description: include_str!("test_1/description.md").to_string(),
        input: SavedMethod {
            id: super::super::Chapter10Equation8Runner.id(),
            parameters: vec![
                SavedParameter {
                    name: "m_{f}".to_string(),
                    value: Some(Float(10.0)),
                },
                SavedParameter {
                    name: "t".to_string(),
                    value: Some(Float(2.0)),
                },
                SavedParameter {
                    name: "LC_{50}".to_string(),
                    value: Some(Float(1250.0)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual("FED".to_string(), 0.016)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::Chapter10Equation8Runner>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
