use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 2".to_string(),
        description: include_str!("test_2/description.md").to_string(),
        input: SavedMethod {
            id: super::super::Chapter10Equation1Runner.id(),
            parameters: vec![
                SavedParameter {
                    name: "\\gamma".to_string(),
                    value: Some(Float(0.5)),
                },
                SavedParameter {
                    name: "d".to_string(),
                    value: Some(Float(0.5)),
                },
                SavedParameter {
                    name: "T_{s}".to_string(),
                    value: Some(Float(493.0)),
                },
                SavedParameter {
                    name: "T_{0}".to_string(),
                    value: Some(Float(293.0)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual(
            "V_{max}".to_string(),
            0.303787243675233,
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::Chapter10Equation1Runner>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
