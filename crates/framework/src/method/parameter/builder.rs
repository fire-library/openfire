use std::sync::Arc;
use std::sync::RwLock;

use uuid::Uuid;

use super::ArcParameter;
use super::Comparison;
use super::DisplayOptions;
use super::Parameter;
use super::ParameterType;
use super::ParameterValue;
use super::Parameters;
use super::Validation;
use super::object::Object;

pub enum ParamBuilder {
    Float(ParameterBuilder<f64>),
    OutputFloat(ParameterBuilder<f64>),
    String(ParameterBuilder<String>),
    StringEnum(ParameterBuilder<String>, Vec<String>),
    Object(ParameterBuilder<Parameters>),
    List(ParameterBuilder<Vec<ArcParameter>>),
}

pub struct ParameterBuilder<T> {
    symbol: String,
    name: Option<String>,
    units: Option<String>,
    validations: Vec<Validation>,
    value: Option<T>,
    display_options: Vec<DisplayOptions>,
}

impl ParamBuilder {
    pub fn float(symbol: &str) -> Self {
        ParamBuilder::Float(ParameterBuilder::<f64> {
            symbol: symbol.to_string(),
            name: None,
            units: None,
            validations: vec![],
            value: None,
            display_options: vec![DisplayOptions::DecimalPlaces(2)],
        })
    }

    pub fn output_float(symbol: &str) -> Self {
        ParamBuilder::OutputFloat(ParameterBuilder::<f64> {
            symbol: symbol.to_string(),
            name: None,
            units: None,
            validations: vec![],
            value: None,
            display_options: vec![DisplayOptions::DecimalPlaces(2)],
        })
    }

    pub fn string(symbol: &str) -> Self {
        ParamBuilder::String(ParameterBuilder::<String> {
            symbol: symbol.to_string(),
            name: None,
            units: None,
            validations: vec![],
            value: None,
            display_options: vec![],
        })
    }

    pub fn string_enum(symbol: &str, values: Vec<&str>) -> Self {
        ParamBuilder::StringEnum(
            ParameterBuilder::<String> {
                symbol: symbol.to_string(),
                name: None,
                units: None,
                validations: vec![],
                value: None,
                display_options: vec![],
            },
            values.iter().map(|s| s.to_string()).collect(),
        )
    }

    pub fn object(symbol: &str, params: Parameters) -> Self {
        ParamBuilder::Object(ParameterBuilder::<Parameters> {
            symbol: symbol.to_string(),
            name: None,
            units: None,
            validations: vec![],
            value: Some(params),
            display_options: vec![],
        })
    }

    pub fn list(symbol: &str) -> Self {
        ParamBuilder::List(ParameterBuilder::<Vec<ArcParameter>> {
            symbol: symbol.to_string(),
            name: None,
            units: None,
            validations: vec![],
            value: Some(vec![]),
            display_options: vec![],
        })
    }

    pub fn name(self, name: &str) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.name = Some(name.to_string());
                ParamBuilder::Float(builder)
            }
            ParamBuilder::OutputFloat(mut builder) => {
                builder.name = Some(name.to_string());
                ParamBuilder::OutputFloat(builder)
            }
            ParamBuilder::String(mut builder) => {
                builder.name = Some(name.to_string());
                ParamBuilder::String(builder)
            }
            ParamBuilder::StringEnum(mut builder, values) => {
                builder.name = Some(name.to_string());
                ParamBuilder::StringEnum(builder, values)
            }
            ParamBuilder::Object(mut builder) => {
                builder.name = Some(name.to_string());
                ParamBuilder::Object(builder)
            }
            ParamBuilder::List(mut builder) => {
                builder.name = Some(name.to_string());
                ParamBuilder::List(builder)
            }
        }
    }

    pub fn units(self, units: &str) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.units = Some(units.to_string());
                ParamBuilder::Float(builder)
            }
            ParamBuilder::OutputFloat(mut builder) => {
                builder.units = Some(units.to_string());
                ParamBuilder::OutputFloat(builder)
            }
            ParamBuilder::StringEnum(mut builder, values) => {
                builder.units = Some(units.to_string());
                ParamBuilder::StringEnum(builder, values)
            }
            ParamBuilder::String(mut builder) => {
                builder.units = Some(units.to_string());
                ParamBuilder::String(builder)
            }
            ParamBuilder::Object(_) => panic!("An object cannot have units"),
            ParamBuilder::List(_) => panic!("A List cannot have units"),
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
            ParamBuilder::OutputFloat(mut builder) => {
                builder.value = value.map(|v| match v {
                    ParameterValue::Float(f) => f,
                    _ => panic!("Invalid value type"),
                });
                ParamBuilder::OutputFloat(builder)
            }
            ParamBuilder::StringEnum(mut builder, values) => {
                builder.value = value.map(|v| match v {
                    ParameterValue::String(s) => s,
                    _ => panic!("Invalid value type"),
                });
                ParamBuilder::StringEnum(builder, values)
            }
            ParamBuilder::String(mut builder) => {
                builder.value = value.map(|v| match v {
                    ParameterValue::String(s) => s,
                    _ => panic!("Invalid value type"),
                });
                ParamBuilder::String(builder)
            }
            ParamBuilder::Object(_) => panic!("An object cannot have a default value"),
            ParamBuilder::List(mut builder) => {
                builder.value = value.map(|v| match v {
                    ParameterValue::List(l) => l,
                    _ => panic!("Invalid value type"),
                });
                ParamBuilder::List(builder)
            }
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
            ParamBuilder::OutputFloat(mut builder) => {
                builder
                    .display_options
                    .push(DisplayOptions::DecimalPlaces(decimal_places));
                ParamBuilder::OutputFloat(builder)
            }
            ParamBuilder::StringEnum(_, _) => panic!("Invalid display option for a string enum"),
            ParamBuilder::String(_) => panic!("Invalid display option for a string"),
            ParamBuilder::Object(_) => panic!("An object cannot have display options"),
            ParamBuilder::List(_) => panic!("A list cannot have display options"),
        }
    }

    pub fn required(self) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.validations.push(Validation::Required);
                ParamBuilder::Float(builder)
            }
            ParamBuilder::OutputFloat(mut builder) => {
                builder.validations.push(Validation::Required);
                ParamBuilder::OutputFloat(builder)
            }
            ParamBuilder::StringEnum(mut builder, v) => {
                builder.validations.push(Validation::Required);
                ParamBuilder::StringEnum(builder, v)
            }
            ParamBuilder::String(mut builder) => {
                builder.validations.push(Validation::Required);
                ParamBuilder::String(builder)
            }
            ParamBuilder::Object(mut builder) => {
                builder.validations.push(Validation::Required);
                ParamBuilder::Object(builder)
            }
            ParamBuilder::List(_) => panic!("A list cannot be required"),
        }
    }

    // pub fn range(self, min: f64, max: f64) -> Self {
    //     match self {
    //         ParamBuilder::Float(mut builder) => {
    //             builder.validations.push(Validation::Range(min, max));
    //             ParamBuilder::Float(builder)
    //         }
    //         ParamBuilder::OutputFloat(mut builder) => {
    //             builder.validations.push(Validation::Range(min, max));
    //             ParamBuilder::OutputFloat(builder)
    //         }
    //         ParamBuilder::String(_) => panic!("Invalid validation for a string"),
    //         ParamBuilder::Object(_) => panic!("Invalid validation for an object"),
    //         ParamBuilder::List(_) => panic!("Invalid validation for a list"),
    //         ParamBuilder::StringEnum(_, _) => panic!("Invalid validation for a string enum"),
    //     }
    // }

    pub fn min(self, min: f64) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.validations.push(Validation::Min(min));
                ParamBuilder::Float(builder)
            }
            ParamBuilder::OutputFloat(mut builder) => {
                builder.validations.push(Validation::Min(min));
                ParamBuilder::OutputFloat(builder)
            }
            ParamBuilder::String(_) => panic!("Invalid validation for a string"),
            ParamBuilder::Object(_) => panic!("Invalid validation for an object"),
            ParamBuilder::List(_) => panic!("Invalid validation for a list"),
            ParamBuilder::StringEnum(_, _) => panic!("Invalid validation for a string enum"),
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
            ParamBuilder::OutputFloat(mut builder) => {
                builder
                    .validations
                    .push(Validation::Relation(Comparison::LessThanOrEqual(
                        parameter.clone(),
                    )));
                ParamBuilder::OutputFloat(builder)
            }
            ParamBuilder::String(_) => panic!("Invalid validation for a string"),
            ParamBuilder::Object(_) => panic!("Invalid validation for an object"),
            ParamBuilder::List(_) => panic!("Invalid validation for a list"),
            ParamBuilder::StringEnum(_, _) => panic!("Invalid validation for a string enum"),
        }
    }

    pub fn min_exclusive(self, min: f64) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.validations.push(Validation::MinExclusive(min));
                ParamBuilder::Float(builder)
            }
            ParamBuilder::OutputFloat(mut builder) => {
                builder.validations.push(Validation::MinExclusive(min));
                ParamBuilder::OutputFloat(builder)
            }
            ParamBuilder::String(_) => panic!("Invalid validation for a string"),
            ParamBuilder::Object(_) => panic!("Invalid validation for an object"),
            ParamBuilder::List(_) => panic!("Invalid validation for a list"),
            ParamBuilder::StringEnum(_, _) => panic!("Invalid validation for a string enum"),
        }
    }

    pub fn max(self, max: f64) -> Self {
        match self {
            ParamBuilder::Float(mut builder) => {
                builder.validations.push(Validation::Max(max));
                ParamBuilder::Float(builder)
            }
            ParamBuilder::OutputFloat(mut builder) => {
                builder.validations.push(Validation::Max(max));
                ParamBuilder::OutputFloat(builder)
            }
            ParamBuilder::String(_) => panic!("Invalid validation for a string"),
            ParamBuilder::Object(_) => panic!("Invalid validation for an object"),
            ParamBuilder::List(_) => panic!("Invalid validation for a list"),
            ParamBuilder::StringEnum(_, _) => panic!("Invalid validation for a string enum"),
        }
    }

    pub fn build(self) -> ArcParameter {
        Arc::new(RwLock::new(self.build_raw()))
    }

    pub fn build_raw(self) -> ParameterType {
        match self {
            ParamBuilder::Float(p) => ParameterType::Float(Parameter::<f64> {
                name: p.name.unwrap(),
                id: Uuid::new_v4().to_string(),
                symbol: p.symbol,
                units: p.units,
                validations: p.validations,
                value: p.value,
                display_options: p.display_options,
            }),
            ParamBuilder::OutputFloat(p) => ParameterType::OutputFloat(Parameter::<f64> {
                name: p.name.unwrap(),
                id: Uuid::new_v4().to_string(),
                symbol: p.symbol,
                units: p.units,
                validations: p.validations,
                value: p.value,
                display_options: p.display_options,
            }),
            ParamBuilder::String(p) => ParameterType::String(Parameter::<String> {
                name: p.name.unwrap(),
                id: Uuid::new_v4().to_string(),
                symbol: p.symbol,
                units: p.units,
                validations: p.validations,
                value: p.value,
                display_options: p.display_options,
            }),
            ParamBuilder::Object(p) => ParameterType::Object(Parameter::<Object> {
                name: p.name.unwrap(),
                id: Uuid::new_v4().to_string(),
                symbol: p.symbol,
                units: p.units,
                validations: p.validations,
                value: Some(Object(p.value.unwrap_or(Parameters::new()))),
                display_options: p.display_options,
            }),
            ParamBuilder::List(p) => ParameterType::List(Parameter::<Vec<ArcParameter>> {
                name: p.name.unwrap(),
                id: Uuid::new_v4().to_string(),
                symbol: p.symbol,
                units: p.units,
                validations: p.validations,
                value: Some(p.value.unwrap_or(vec![])),
                display_options: p.display_options,
            }),
            ParamBuilder::StringEnum(p, values) => ParameterType::StringEnum(
                Parameter::<String> {
                    name: p.name.unwrap(),
                    id: Uuid::new_v4().to_string(),
                    symbol: p.symbol,
                    units: p.units,
                    validations: p.validations,
                    value: p.value,
                    display_options: p.display_options,
                },
                values,
            ),
        }
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
