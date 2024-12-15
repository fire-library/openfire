use std::sync::Arc;
use std::sync::RwLock;

use crate::domain::method::equation::Equation;

use super::ArcParameter;
use super::Comparison;
use super::DisplayOptions;
use super::Parameter;
use super::ParameterType;
use super::ParameterValue;
use super::Validation;

pub struct ParameterBuilder {
    id: String,
    name: Option<String>,
    units: Option<String>,
    validations: Vec<Validation>,
    parameter_type: ParameterType,
    value: Option<ParameterValue>,
    expression: Option<Box<dyn Equation>>,
    display_options: Vec<DisplayOptions>,
}

impl ParameterBuilder {
    pub fn float(id: &str) -> Self {
        ParameterBuilder {
            id: id.to_string(),
            name: None,
            units: None,
            validations: vec![],
            parameter_type: ParameterType::Float,
            value: None,
            expression: None,
            display_options: vec![DisplayOptions::DecimalPlaces(2)],
        }
    }

    pub fn string(id: &str) -> Self {
        ParameterBuilder {
            id: id.to_string(),
            name: None,
            units: None,
            validations: vec![],
            parameter_type: ParameterType::String,
            value: None,
            expression: None,
            display_options: vec![],
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn units(mut self, units: &str) -> Self {
        self.units = Some(units.to_string());
        self
    }

    pub fn expression(mut self, exp: Box<dyn Equation>) -> Self {
        self.expression = Some(exp);
        self
    }

    pub fn default_value(mut self, value: Option<ParameterValue>) -> Self {
        self.value = value;
        self
    }

    pub fn decimal_places(mut self, decimal_places: u32) -> Self {
        self.display_options
            .push(DisplayOptions::DecimalPlaces(decimal_places));
        self
    }

    pub fn required(mut self) -> Self {
        self.validations.push(Validation::Required);
        self
    }

    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.validations.push(Validation::Range(min, max));
        self
    }

    pub fn min(mut self, min: f64) -> Self {
        self.validations.push(Validation::Min(min));
        self
    }

    pub fn less_than_or_equal_to_parameter(mut self, parameter: &ArcParameter) -> Self {
        self.validations
            .push(Validation::Relation(Comparison::LessThanOrEqual(
                parameter.clone(),
            )));

        self
    }

    pub fn greater_than_or_equal_to_parameter(mut self, parameter: &ArcParameter) -> Self {
        self.validations
            .push(Validation::Relation(Comparison::GreaterThanOrEqual(
                parameter.clone(),
            )));
        self
    }

    pub fn min_exclusive(mut self, min: f64) -> Self {
        self.validations.push(Validation::MinExclusive(min));
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.validations.push(Validation::Max(max));
        self
    }

    pub fn build(self) -> ArcParameter {
        let p = Parameter {
            name: self.name.unwrap(),
            id: self.id,
            units: self.units,
            validations: self.validations,
            parameter_type: self.parameter_type,
            value: self.value,
            expression: self.expression,
            display_options: self.display_options,
        };

        Arc::new(RwLock::new(p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_field_builder() {
        let field = ParameterBuilder::float("test_symbol")
            .name("test_name")
            .units("test_units")
            .default_value(Some(ParameterValue::Float(5.0)))
            .required()
            .range(1.0, 10.0)
            .build();

        let field = field.read().unwrap();
        assert_eq!(field.name, "test_name");
        assert_eq!(field.id, "test_symbol");
        assert_eq!(field.units.as_ref().unwrap(), "test_units");
        assert_eq!(field.value, Some(ParameterValue::Float(5.0)));
        // assert!(field.validations.contains(&Validation::Required));
        // assert!(field.validations.contains(&Validation::Range(1.0, 10.0)));
    }
}
