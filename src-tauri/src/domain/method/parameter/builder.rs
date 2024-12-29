use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use crate::domain::method::equation::Equation;

use super::ArcParameter;
use super::Comparison;
use super::DisplayOptions;
use super::Parameter;
use super::ParameterType;
use super::ParameterValue;
use super::Parameters;
use super::Validation;

pub enum ParamBuilder {
    Float(ParameterBuilder<f64>),
    String(ParameterBuilder<String>),
    Object(ParameterBuilder<Vec<ArcParameter>>),
}

pub struct ParameterBuilder<T> {
    id: String,
    name: Option<String>,
    units: Option<String>,
    validations: Vec<Validation>,
    value: Option<T>,
    expression: Option<Box<dyn Equation>>,
    display_options: Vec<DisplayOptions>,
}

impl ParamBuilder {
    pub fn float(id: &str) -> Self {
        ParamBuilder::Float(ParameterBuilder::<f64> {
            id: id.to_string(),
            name: None,
            units: None,
            validations: vec![],
            value: None,
            expression: None,
            display_options: vec![DisplayOptions::DecimalPlaces(2)],
        })
    }

    pub fn string(id: &str) -> Self {
        ParamBuilder::String(ParameterBuilder::<String> {
            id: id.to_string(),
            name: None,
            units: None,
            validations: vec![],
            value: None,
            expression: None,
            display_options: vec![],
        })
    }

    pub fn object(id: &str, params: Vec<ArcParameter>) -> Self {
        ParamBuilder::Object(ParameterBuilder::<Vec<ArcParameter>> {
            id: id.to_string(),
            name: None,
            units: None,
            validations: vec![],
            value: None,
            expression: None,
            display_options: vec![],
        })
    }

    pub fn name(self, name: &str) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.name = Some(name.to_string());
                ParamBuilder::Float(builder)
            }
            ParamBuilder::String(mut builder) => {
                builder.name = Some(name.to_string());
                ParamBuilder::String(builder)
            }
            ParamBuilder::Object(mut builder) => {
                builder.name = Some(name.to_string());
                ParamBuilder::Object(builder)
            }
        }
    }

    pub fn units(self, units: &str) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.units = Some(units.to_string());
                ParamBuilder::Float(builder)
            }
            ParamBuilder::String(mut builder) => {
                builder.units = Some(units.to_string());
                ParamBuilder::String(builder)
            }
            ParamBuilder::Object(_) => panic!("An object cannot have units"),
        }
    }

    pub fn expression(self, exp: Box<dyn Equation>) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.expression = Some(exp);
                ParamBuilder::Float(builder)
            }
            ParamBuilder::String(mut builder) => {
                builder.expression = Some(exp);
                ParamBuilder::String(builder)
            }
            ParamBuilder::Object(_) => panic!("An object cannot have an expression"),
        }
    }

    pub fn default_value(self, value: Option<ParameterValue>) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.value = value.map(|v| match v {
                    ParameterValue::Float(f) => f,
                    _ => panic!("Invalid value type"),
                });
                ParamBuilder::Float(builder)
            }
            ParamBuilder::String(mut builder) => {
                builder.value = value.map(|v| match v {
                    ParameterValue::String(s) => s,
                    _ => panic!("Invalid value type"),
                });
                ParamBuilder::String(builder)
            }
            ParamBuilder::Object(_) => panic!("An object cannot have a default value"),
        }
    }

    pub fn decimal_places(self, decimal_places: u32) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder
                    .display_options
                    .push(DisplayOptions::DecimalPlaces(decimal_places));
                ParamBuilder::Float(builder)
            }
            ParamBuilder::String(_) => panic!("Invalid display option for a string"),
            ParamBuilder::Object(_) => panic!("An object cannot have display options"),
        }
    }

    pub fn required(self) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.validations.push(Validation::Required);
                ParamBuilder::Float(builder)
            }
            ParamBuilder::String(mut builder) => {
                builder.validations.push(Validation::Required);
                ParamBuilder::String(builder)
            }
            ParamBuilder::Object(mut builder) => {
                builder.validations.push(Validation::Required);
                ParamBuilder::Object(builder)
            }
        }
    }

    pub fn range(self, min: f64, max: f64) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.validations.push(Validation::Range(min, max));
                ParamBuilder::Float(builder)
            }
            ParamBuilder::String(_) => panic!("Invalid validation for a string"),
            ParamBuilder::Object(_) => panic!("Invalid validation for an object"),
        }
    }

    pub fn min(self, min: f64) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.validations.push(Validation::Min(min));
                ParamBuilder::Float(builder)
            }
            ParamBuilder::String(_) => panic!("Invalid validation for a string"),
            ParamBuilder::Object(_) => panic!("Invalid validation for an object"),
        }
    }

    pub fn less_than_or_equal_to_parameter(self, parameter: &ArcParameter) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder
                    .validations
                    .push(Validation::Relation(Comparison::LessThanOrEqual(
                        parameter.clone(),
                    )));
                ParamBuilder::Float(builder)
            }
            ParamBuilder::String(_) => panic!("Invalid validation for a string"),
            ParamBuilder::Object(_) => panic!("Invalid validation for an object"),
        }
    }

    pub fn greater_than_or_equal_to_parameter(self, parameter: &ArcParameter) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder
                    .validations
                    .push(Validation::Relation(Comparison::GreaterThanOrEqual(
                        parameter.clone(),
                    )));
                ParamBuilder::Float(builder)
            }
            ParamBuilder::String(_) => panic!("Invalid validation for a string"),
            ParamBuilder::Object(_) => panic!("Invalid validation for an object"),
        }
    }

    pub fn min_exclusive(self, min: f64) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.validations.push(Validation::MinExclusive(min));
                ParamBuilder::Float(builder)
            }
            ParamBuilder::String(_) => panic!("Invalid validation for a string"),
            ParamBuilder::Object(_) => panic!("Invalid validation for an object"),
        }
    }

    pub fn max(self, max: f64) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.validations.push(Validation::Max(max));
                ParamBuilder::Float(builder)
            }
            ParamBuilder::String(_) => panic!("Invalid validation for a string"),
            ParamBuilder::Object(_) => panic!("Invalid validation for an object"),
        }
    }

    pub fn build(self) -> ArcParameter {
        let p = match self {
            ParamBuilder::Float(p) => ParameterType::Float(Parameter::<f64> {
                name: p.name.unwrap(),
                id: p.id,
                units: p.units,
                validations: p.validations,
                value: p.value,
                expression: p.expression,
                display_options: p.display_options,
            }),
            ParamBuilder::String(p) => ParameterType::String(Parameter::<String> {
                name: p.name.unwrap(),
                id: p.id,
                units: p.units,
                validations: p.validations,
                value: p.value,
                expression: p.expression,
                display_options: p.display_options,
            }),
            ParamBuilder::Object(p) => ParameterType::Object(Parameter::<Parameters> {
                name: p.name.unwrap(),
                id: p.id,
                units: p.units,
                validations: p.validations,
                value: Some(HashMap::new()),
                expression: p.expression,
                display_options: p.display_options,
            }),
        };

        Arc::new(RwLock::new(p))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_float_field_builder() {
//         let field = ParameterBuilder::float("test_symbol")
//             .name("test_name")
//             .units("test_units")
//             .default_value(Some(ParameterValue::Float(5.0)))
//             .required()
//             .range(1.0, 10.0)
//             .build();

//         let field = field.read().unwrap();
//         assert_eq!(field.name, "test_name");
//         assert_eq!(field.id, "test_symbol");
//         assert_eq!(field.units.as_ref().unwrap(), "test_units");
//         assert_eq!(field.value, Some(ParameterValue::Float(5.0)));
//         // assert!(field.validations.contains(&Validation::Required));
//         // assert!(field.validations.contains(&Validation::Range(1.0, 10.0)));
//     }
// }
