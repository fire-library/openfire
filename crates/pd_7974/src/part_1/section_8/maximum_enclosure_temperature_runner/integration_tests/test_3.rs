use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 3".to_string(),
        description: include_str!("test_3/description.md").to_string(),
        input: SavedMethod {
            id: super::super::MaximumEnclosureTemperatureBuilder.id(),
            parameters: vec![
                SavedParameter {
                    name: "A_t".to_string(),
                    value: Some(Float(45.0)),
                },
                SavedParameter {
                    name: "A_v".to_string(),
                    value: Some(Float(2.1)),
                },
                SavedParameter {
                    name: "H_v".to_string(),
                    value: Some(Float(2.1)),
                },
                SavedParameter {
                    name: "m_e".to_string(),
                    value: Some(Float(250.0)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual("\\Psi".to_string(), 25.717224993682)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::MaximumEnclosureTemperatureBuilder>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
