use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 1".to_string(),
        description: include_str!("test_1/description.md").to_string(),
        input: SavedMethod {
            id: super::super::MaximumHRRBuilder.id(),
            parameters: vec![
                SavedParameter {
                    name: "A_v".to_string(),
                    value: Some(Float(2.0)),
                },
                SavedParameter {
                    name: "H_v".to_string(),
                    value: Some(Float(2.1)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual(
            "\\dot{Q}_{max, \\space VC}".to_string(),
            4347.41302385683,
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
