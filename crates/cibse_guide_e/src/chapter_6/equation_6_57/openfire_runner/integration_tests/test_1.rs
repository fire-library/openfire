use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 1".to_string(),
        description: include_str!("test_1/description.md").to_string(),
        input: SavedMethod {
            id: super::super::Chapter6Equation57Runner.id(),
            parameters: vec![
                SavedParameter {
                    name: "R".to_string(),
                    value: Some(Float(0.2)),
                },
                SavedParameter {
                    name: "w".to_string(),
                    value: Some(Float(1.0)),
                },
                SavedParameter {
                    name: "h_{o}".to_string(),
                    value: Some(Float(2.1)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual(
            "z_{fo}".to_string(),
            2.2775384234923455,
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::Chapter6Equation57Runner>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
