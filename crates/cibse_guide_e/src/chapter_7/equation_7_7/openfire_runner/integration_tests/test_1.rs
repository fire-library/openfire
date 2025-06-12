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
            id: super::super::Chapter7Equation7Runner.id(),
            parameters: vec![SavedParameter {
                name: "p".to_string(),
                value: Some(Float(3.5)),
            },
            SavedParameter {
                name: "A".to_string(),
                value: Some(Float(8.0)),
            },
            SavedParameter {
                name: "S".to_string(),
                value: Some(Integer(6)),
            }],
],
],
        },
        assertions: vec![Assertion::IntegerEqual("N_c".to_string(), 168)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::Chapter7Equation7Runner>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
