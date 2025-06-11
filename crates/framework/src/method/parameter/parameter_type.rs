use core::panic;

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::method::parameter::{self, JsonSchemaTrait};

use super::{ArcParameter, Parameter, ParameterValue, object::Object};
use schemars::{JsonSchema, Schema, SchemaGenerator};

#[derive(Type, Serialize, Deserialize, Debug)]
pub enum ParameterType {
    String(Parameter<String>),
    Float(Parameter<f64>),
    Integer(Parameter<i32>),
    OutputFloat(Parameter<f64>),
    Bool(Parameter<bool>),
    Object(Parameter<Object>),
    StringEnum(Parameter<String>, Vec<String>),
    List(Parameter<Vec<ArcParameter>>),
    Constant(Parameter<ParameterValue>),
}

impl ParameterType {
    pub fn variant_json_schema(&self, generator: &mut SchemaGenerator) -> Schema {
        match self {
            ParameterType::String(p) => {
                let mut a = <String as JsonSchema>::json_schema(generator);
                self.extend_schema(p, &mut a);
                a
            }
            ParameterType::Float(p) => {
                let mut a = <f64 as JsonSchema>::json_schema(generator);
                self.extend_schema(p, &mut a);
                a
            }
            ParameterType::Integer(p) => {
                let mut a = <i32 as JsonSchema>::json_schema(generator);
                self.extend_schema(p, &mut a);
                a
            }
            ParameterType::OutputFloat(_) => {
                panic!("Output floats should not be used in JSON schema generation")
            }
            ParameterType::Bool(p) => {
                let mut a = <bool as JsonSchema>::json_schema(generator);
                a.insert(
                    "name".to_string(),
                    serde_json::Value::String(p.name.clone()),
                );
                a
            }
            ParameterType::Object(_) => {
                panic!("Constant parameters should not be used in JSON schema generation")
            }
            ParameterType::StringEnum(_, _) => {
                panic!("Constant parameters should not be used in JSON schema generation")
            }
            ParameterType::Constant(_) => {
                panic!("Constant parameters should not be used in JSON schema generation")
            }
            ParameterType::List(_) => {
                panic!("Constant parameters should not be used in JSON schema generation")
            }
        }
    }

    fn extend_schema<T>(&self, parameter: &Parameter<T>, schema: &mut Schema) {
        let description = if parameter.units.is_some() {
            format!("Units: {}", parameter.units.clone().unwrap())
        } else {
            "Units: dimensionless".to_string()
        };

        schema.insert(
            "name".to_string(),
            serde_json::Value::String(parameter.name.clone()),
        );
        schema.insert(
            "description".to_string(),
            serde_json::Value::String(description),
        );
    }

    pub fn is_required(&self) -> bool {
        match self {
            ParameterType::String(param) => param.validations.is_required(),
            ParameterType::Float(param) => param.validations.is_required(),
            ParameterType::Integer(param) => param.validations.is_required(),
            ParameterType::OutputFloat(param) => param.validations.is_required(),
            ParameterType::Bool(param) => param.validations.is_required(),
            ParameterType::Object(_) => {
                panic!("Object parameters not implemented for is_required check")
            }
            ParameterType::StringEnum(_, _) => {
                panic!("StringEnum parameters no implemented for is_required check")
            }
            ParameterType::List(_) => {
                panic!("List parameters no implemented for is_required check")
            }
            ParameterType::Constant(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use schemars::SchemaGenerator;

    #[test]
    fn test_parameter_type_variant_schema_float() {
        let param = ParameterType::Float(crate::method::parameter::Parameter {
            id: "id".to_string(),
            symbol: "x".to_string(),
            name: "Test Float".to_string(),
            value: Some(42.0),
            display_options: vec![],
            units: Some("m".to_string()),
            validations: vec![],
        });
        let mut generator = SchemaGenerator::default();
        let schema = param.variant_json_schema(&mut generator);
        let schema_json = serde_json::to_string(&schema).unwrap();
        assert_eq!(
            schema_json.to_string(),
            "{\"description\":\"Units: m\",\"type\":\"number\",\"format\":\"double\",\"name\":\"Test Float\"}".to_string()
        );
    }

    #[test]
    fn test_parameter_type_variant_schema_string() {
        let param = ParameterType::String(crate::method::parameter::Parameter {
            id: "id".to_string(),
            symbol: "s".to_string(),
            name: "Test String".to_string(),
            value: Some("hello".to_string()),
            display_options: vec![],
            units: None,
            validations: vec![],
        });
        let mut generator = SchemaGenerator::default();
        let schema = param.variant_json_schema(&mut generator);
        let schema_json = serde_json::to_string(&schema).unwrap();
        assert_eq!(schema_json.to_string(), "{\"description\":\"Units: dimensionless\",\"type\":\"string\",\"name\":\"Test String\"}".to_string());
    }

    #[test]
    fn test_parameter_type_variant_schema_integer() {
        let param = ParameterType::Integer(crate::method::parameter::Parameter {
            id: "id".to_string(),
            symbol: "i".to_string(),
            name: "Test Integer".to_string(),
            value: Some(7),
            display_options: vec![],
            units: None,
            validations: vec![],
        });
        let mut generator = SchemaGenerator::default();
        let schema = param.variant_json_schema(&mut generator);
        let schema_json = serde_json::to_string(&schema).unwrap();
        assert_eq!(
            schema_json.to_string(),
            "{\"description\":\"Units: dimensionless\",\"type\":\"integer\",\"format\":\"int32\",\"name\":\"Test Integer\"}".to_string()
        );
    }

    #[test]
    fn test_parameter_type_variant_schema_bool() {
        let param = ParameterType::Bool(crate::method::parameter::Parameter {
            id: "id".to_string(),
            symbol: "b".to_string(),
            name: "Test Bool".to_string(),
            value: Some(true),
            display_options: vec![],
            units: None,
            validations: vec![],
        });
        let mut generator = SchemaGenerator::default();
        let schema = param.variant_json_schema(&mut generator);
        let schema_json = serde_json::to_string(&schema).unwrap();
        assert_eq!(
            schema_json.to_string(),
            "{\"type\":\"boolean\",\"name\":\"Test Bool\"}".to_string()
        );
    }
}
