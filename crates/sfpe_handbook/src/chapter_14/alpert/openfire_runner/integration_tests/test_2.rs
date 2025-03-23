use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 2".to_string(),
        description: include_str!("test_2/description.md").to_string(),
        input: SavedMethod {
            id: super::super::AlpertHeatReleaseFromTempAndPositionBuilder.id(),
            parameters: vec![
                SavedParameter {
                    name: "T".to_string(),
                    value: Some(Float(333.0)),
                },
                SavedParameter {
                    name: "T_\\infty".to_string(),
                    value: Some(Float(293.0)),
                },
                SavedParameter {
                    name: "H".to_string(),
                    value: Some(Float(5.0)),
                },
                SavedParameter {
                    name: "r".to_string(),
                    value: Some(Float(2.5)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual(
            "\\dot{Q}".to_string(),
            566.645688522019,
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<
            super::super::super::AlpertHeatReleaseFromTempAndPositionBuilder,
        >();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
