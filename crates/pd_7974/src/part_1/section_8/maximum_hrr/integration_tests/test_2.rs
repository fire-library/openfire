use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 2".to_string(),
        description: include_str!("test_2/description.md").to_string(),
        input: SavedMethod {
            id: super::super::MaximumHRRBuilder.id(),
            parameters: vec![
                SavedParameter {
                    name: "A_f".to_string(),
                    value: Some(Float(10.0)),
                },
                SavedParameter {
                    name: "HRRPUA".to_string(),
                    value: Some(Float(500.0)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual(
            "\\dot{Q}_{fo, \\space FC}".to_string(),
            5000.0,
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::MaximumHRRBuilder>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
