use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 1".to_string(),
        description: include_str!("test_1/description.md").to_string(),
        input: SavedMethod {
            id: super::super::Chapter6Equation58Runner.id(),
            parameters: vec![
                SavedParameter {
                    name: "A_{t}".to_string(),
                    value: Some(Float(45.0)),
                },
                SavedParameter {
                    name: "A_{o}".to_string(),
                    value: Some(Float(2.1)),
                },
                SavedParameter {
                    name: "h_{o}".to_string(),
                    value: Some(Float(2.1)),
                },
                SavedParameter {
                    name: "w".to_string(),
                    value: Some(Float(3.0)),
                },
                SavedParameter {
                    name: "d".to_string(),
                    value: Some(Float(4.0)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual("R".to_string(), 0.1979036228367894)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::Chapter6Equation58Runner>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
