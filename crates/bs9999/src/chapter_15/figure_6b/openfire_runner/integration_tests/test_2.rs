use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    framework::register_runner::<super::super::BS9999Chapter15Figure6bBuilder>();
    Test {
        name: "Test 2".to_string(),
        description: include_str!("test_2/description.md").to_string(),
        input: SavedMethod {
            id: super::super::BS9999Chapter15Figure6bBuilder.id(),
            parameters: vec![
                SavedParameter {
                    name: "S_{up}".to_string(),
                    value: Some(Float(1000.0)),
                },
                SavedParameter {
                    name: "S_{dn}".to_string(),
                    value: Some(Float(1000.0)),
                },
                SavedParameter {
                    name: "B".to_string(),
                    value: Some(Float(61.0)),
                },
                SavedParameter {
                    name: "D".to_string(),
                    value: Some(Float(3.0)),
                },
                SavedParameter {
                    name: "X".to_string(),
                    value: Some(Float(3.6)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual("W_{FE}".to_string(), 969.6)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
