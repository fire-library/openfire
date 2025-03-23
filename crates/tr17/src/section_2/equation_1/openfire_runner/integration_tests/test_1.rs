use framework::filesystem::saved_method::{SavedMethod, SavedParameter};
use framework::method::parameter::ParameterValue::Float;
use framework::method::runner::MethodRunner;
use framework::method::test::{Assertion, Test};

pub fn test() -> Test {
    framework::register_runner::<super::super::TR17Section2Equation1Builder>();

    Test {
        name: "Test 1".to_string(),
        description: include_str!("test_1/description.md").to_string(),
        input: SavedMethod {
            id: super::super::TR17Section2Equation1Builder.id(),
            parameters: vec![
                SavedParameter {
                    name: "\\dot{Q}".to_string(),
                    value: Some(Float(1000.0)),
                },
                SavedParameter {
                    name: "\\rho_a".to_string(),
                    value: Some(Float(1.2)),
                },
                SavedParameter {
                    name: "c_p".to_string(),
                    value: Some(Float(1.0)),
                },
                SavedParameter {
                    name: "T_a".to_string(),
                    value: Some(Float(293.0)),
                },
                SavedParameter {
                    name: "g".to_string(),
                    value: Some(Float(9.8)),
                },
                SavedParameter {
                    name: "H_e".to_string(),
                    value: Some(Float(3.0)),
                },
            ],
        },
        assertions: vec![Assertion::FloatEqual(
            "Q^{*}".to_string(),
            0.058282068762112685,
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test = test();
        let results = framework::method::test::run_test(test).unwrap();

        for r in results {
            assert!(r.has_passed());
        }
    }
}
