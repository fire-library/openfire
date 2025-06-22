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
            id: super::super::Chapter7Equation8Runner.id(),
            parameters: vec![
                SavedParameter {
                    name: "W_s".to_string(),
                    value: Some(Float(1.2)),
                },
                SavedParameter {
                    name: "t".to_string(),
                    value: Some(Float(150.0)),
                },
                SavedParameter {
                    name: "A".to_string(),
                    value: Some(Float(10.0)),
                },
                SavedParameter {
                    name: "S".to_string(),
                    value: Some(Integer(5)),
                },
            ],
        },
        assertions: vec![Assertion::IntegerEqual("N_{in_(max)}".to_string(), 379)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::Chapter7Equation8Runner>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
