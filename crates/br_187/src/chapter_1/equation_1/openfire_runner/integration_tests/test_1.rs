use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 1".to_string(),
        description: include_str!("test_1/description.md").to_string(),
        input: SavedMethod {
            id: super::super::BR187Chapter1Equation1Builder.id(),
            parameters: vec![
                SavedParameter {
                    name: "A_s".to_string(),
                    value: Some(Float(37.215)),
                },
                SavedParameter {
                    name: "A".to_string(),
                    value: Some(Float(1.785)),
                },
                SavedParameter {
                    name: "H".to_string(),
                    value: Some(Float(2.1)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual("O".to_string(), 14.386997081751101)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::BR187Chapter1Equation1Builder>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
