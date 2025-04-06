use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    Test {
        name: "Test 2".to_string(),
        description: include_str!("test_2/description.md").to_string(),
        input: SavedMethod {
            id: super::super::HRRAtFlashoverBuilder.id(),
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
                    name: "h_k".to_string(),
                    value: Some(Float(0.03)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual(
            "\\dot{Q}_{fo, \\space McCaffrey}".to_string(),
            1236.4062459149,
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        framework::register_runner::<super::super::super::HRRAtFlashoverBuilder>();
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
