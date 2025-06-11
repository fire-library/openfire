use crate::method::parameter::{ArcParameter, ParameterTrait};
use core::panic;
use schemars::{Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use serde_json::json;
use specta::Type;
use std::collections::BTreeMap;

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub struct Parameters {
    params: BTreeMap<String, ArcParameter>,
    order: Vec<String>,
}

impl Parameters {
    pub fn new() -> Self {
        Parameters {
            params: BTreeMap::new(),
            order: vec![],
        }
    }

    pub fn get(&self, symbol: &str) -> ArcParameter {
        if let Some(parameter) = self.params.get(symbol) {
            return parameter.clone();
        } else {
            panic!("Parameter {} not found", symbol);
        }
    }

    pub fn values(&self) -> Vec<ArcParameter> {
        self.order.iter().map(|name| self.get(name)).collect()
    }

    pub fn add(&mut self, parameter: ArcParameter) {
        let order = &self.order;
        self.params.insert(parameter.symbol(), parameter.clone());
        if !order.contains(&parameter.symbol()) {
            self.order.push(parameter.symbol());
        }
    }

    pub fn json_schema(&self, generator: &mut SchemaGenerator) -> Schema {
        let mut properties = serde_json::Map::new();
        let mut required = Vec::new();
        for (symbol, arc_param) in &self.params {
            let param_type = arc_param.read().unwrap();
            let schema = param_type.variant_json_schema(generator);
            let schema_json = serde_json::to_value(&schema).unwrap();
            properties.insert(symbol.clone(), schema_json);
            if param_type.is_required() {
                required.push(symbol.clone());
            }
        }

        let schema_json = json!({
            "type": "object",
            "properties": properties,
            "required": required
        });
        serde_json::from_value(schema_json).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::method::parameter::{Parameter, ParameterType, Validation};
    use schemars::SchemaGenerator;
    use std::sync::{Arc, RwLock};

    fn make_float_param(symbol: &str, value: f64) -> ArcParameter {
        Arc::new(RwLock::new(ParameterType::Float(Parameter {
            id: symbol.to_string(),
            symbol: symbol.to_string(),
            name: format!("{} param", symbol),
            value: Some(value),
            display_options: vec![],
            units: Some("m".to_string()),
            validations: vec![],
        })))
    }

    fn make_string_param(symbol: &str, value: &str) -> ArcParameter {
        Arc::new(RwLock::new(ParameterType::String(Parameter {
            id: symbol.to_string(),
            symbol: symbol.to_string(),
            name: format!("{} param", symbol),
            value: Some(value.to_string()),
            display_options: vec![],
            units: None,
            validations: vec![Validation::Required],
        })))
    }

    #[test]
    fn test_json_schema() {
        let mut params = Parameters::new();
        params.add(make_float_param("x", 42.0));
        params.add(make_string_param("s", "hello"));
        let mut generator = SchemaGenerator::default();
        let schema = params.json_schema(&mut generator);
        let schema_json = serde_json::to_value(&schema).unwrap();

        assert_eq!(schema_json["type"], "object");
        assert!(schema_json["properties"].get("x").is_some());
        assert!(schema_json["properties"].get("s").is_some());
        assert!(
            schema_json["required"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!("s"))
        );
        // Check property types
        assert_eq!(schema_json["properties"]["x"]["type"], "number");
        assert_eq!(schema_json["properties"]["s"]["type"], "string");
    }
}
