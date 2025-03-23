use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 3".to_string(),
        description: include_str!("test_3/description.md").to_string(),
        input: SavedMethod {
            id: super::super::BurningRegimeBuilder.id(),
            parameters: vec![
                SavedParameter {
                    name: "\\rho".to_string(),
                    value: Some(Float(1.2)),
                },
                SavedParameter {
                    name: "g".to_string(),
                    value: Some(Float(9.8)),
                },
                SavedParameter {
                    name: "A_w".to_string(),
                    value: Some(Float(1.785)),
                },
                SavedParameter {
                    name: "H".to_string(),
                    value: Some(Float(2.1)),
                },
                SavedParameter {
                    name: "A_f".to_string(),
                    value: Some(Float(37.215)),
                },
            ],
        },
        assertions: vec![
            Assertion::FloatEqual("F".to_string(), 0.261110374934643),
            Assertion::StringEqual("Regime".to_string(), "Undefined / Crossover".to_string()),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::BurningRegimeBuilder>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
