pub mod builder;
pub mod object;
pub mod parameter_value;
use object::Object;
pub use parameter_value::ParameterValue;

pub mod parameter_type;
pub use parameter_type::ParameterType;

use super::form::{ArcField, FieldType};
use super::validation::ParameterError;
use crate::method::form::Field;
use core::panic;
use serde::{Deserialize, Serialize};
use specta::Type;

use std::sync::{Arc, RwLock};

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Validation {
    Required,
    MinLength(u32),
    MaxLength(u32),
    Range(f64, f64),
    MinExclusive(f64),
    Min(f64),
    Max(f64),
    Relation(Comparison),
}

#[derive(Clone, Type, Serialize, Deserialize, Debug)]
pub enum Comparison {
    GreaterThanOrEqual(ArcParameter),
    LessThanOrEqual(ArcParameter),
    Unique(Vec<ArcParameter>),
}

#[derive(Type, Serialize, Deserialize, Debug, Clone)]
pub enum DisplayOptions {
    DecimalPlaces(u32),
}

pub trait DisplayOptionsTrait {
    fn decimal_places(&self) -> Option<u32>;
}

impl DisplayOptionsTrait for Vec<DisplayOptions> {
    fn decimal_places(&self) -> Option<u32> {
        for option in self {
            let DisplayOptions::DecimalPlaces(places) = option;
            return Some(*places);
        }
        None
    }
}

#[derive(Type, Serialize, Deserialize, Debug)]
pub struct Parameter<T> {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub value: Option<T>,
    pub display_options: Vec<DisplayOptions>,
    pub units: Option<String>,
    #[specta(skip)]
    #[serde(skip)]
    pub validations: Vec<Validation>,
}

pub trait JsonSchemaTrait {
    fn is_required(&self) -> bool;
}

impl JsonSchemaTrait for Vec<Validation> {
    fn is_required(&self) -> bool {
        self.iter().any(|v| matches!(v, Validation::Required))
    }
}

impl<T: PartialEq + Clone> PartialEq for Parameter<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Parameter<String> {
    pub fn validate(&self) -> Result<(), ParameterError> {
        for validation in &self.validations {
            match validation {
                Validation::Required => match &self.value {
                    Some(value) => {
                        if value.trim().is_empty() {
                            return Err(ParameterError::new(
                                &self.id,
                                &self.symbol,
                                "required".to_string(),
                            ));
                        }
                    }
                    None => {
                        return Err(ParameterError::new(
                            &self.id,
                            &self.symbol,
                            "required".to_string(),
                        ));
                    }
                },
                Validation::MinLength(min) => match &self.value {
                    Some(value) => {
                        if (value.len() as u32) < *min {
                            return Err(ParameterError::new(
                                &self.id,
                                &self.symbol,
                                format!("must be at least {} characters", min),
                            ));
                        }
                    }
                    None => {}
                },
                Validation::MaxLength(max) => match &self.value {
                    Some(value) => {
                        if (value.len() as u32) > *max {
                            return Err(ParameterError::new(
                                &self.id,
                                &self.symbol,
                                format!("must be at most {} characters", max),
                            ));
                        }
                    }
                    None => {}
                },
                Validation::Relation(Comparison::Unique(params)) => {
                    if let Some(value) = self.value.clone() {
                        for param in params {
                            match &*param.read().unwrap() {
                                ParameterType::String(param) => {
                                    if let Some(param_value) = param.value.clone() {
                                        if value == param_value {
                                            return Err(ParameterError::new(
                                                &self.id,
                                                &self.symbol,
                                                format!("must be unique"),
                                            ));
                                        }
                                    }
                                }
                                _ => {
                                    return Err(ParameterError::new(
                                        &self.id,
                                        &self.symbol,
                                        format!(
                                            "Parameter {} cannot be compared for uniqueness against {}",
                                            self.symbol,
                                            param.read().unwrap().symbol()
                                        ),
                                    ));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl Parameter<f64> {
    pub fn validate(&self) -> Result<(), ParameterError> {
        for validation in &self.validations {
            match validation {
                Validation::Required => match &self.value {
                    Some(_value) => {}
                    None => {
                        return Err(ParameterError::new(
                            &self.id,
                            &self.symbol,
                            "required".to_string(),
                        ));
                    }
                },
                Validation::Range(min, max) => match &self.value {
                    Some(value) => {
                        if value < min || value > max {
                            return Err(ParameterError::new(
                                &self.id,
                                &self.symbol,
                                format!("must be between {} and {}", min, max),
                            ));
                        }
                    }
                    None => {}
                },
                Validation::Min(min) => match &self.value {
                    Some(value) => {
                        if value < min {
                            return Err(ParameterError::new(
                                &self.id,
                                &self.symbol,
                                format!("min: {}", min),
                            ));
                        }
                    }
                    None => {}
                },
                Validation::Max(max) => match &self.value {
                    Some(value) => {
                        if value > max {
                            return Err(ParameterError::new(
                                &self.id,
                                &self.symbol,
                                format!("max: {}", max),
                            ));
                        }
                    }
                    None => {}
                },
                Validation::MinExclusive(min) => match &self.value {
                    Some(value) => {
                        if value <= min {
                            return Err(ParameterError::new(
                                &self.id,
                                &self.symbol,
                                format!("must be greater than {}", min),
                            ));
                        }
                    }
                    None => {}
                },
                Validation::Relation(validation) => match &validation {
                    Comparison::GreaterThanOrEqual(param) => {
                        if let Some(value) = self.value {
                            match &*param.read().unwrap() {
                                ParameterType::Float(param) => {
                                    if let Some(param_value) = param.value {
                                        if value < param_value {
                                            return Err(ParameterError::new(
                                                &self.id,
                                                &self.symbol,
                                                format!(
                                                    "must be less than or equal to: {}",
                                                    param.name
                                                ),
                                            ));
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    Comparison::Unique(params) => {
                        if let Some(value) = self.value {
                            for param in params {
                                match &*param.read().unwrap() {
                                    ParameterType::Float(param) => {
                                        if let Some(param_value) = param.value {
                                            if value == param_value {
                                                return Err(ParameterError::new(
                                                    &self.id,
                                                    &self.symbol,
                                                    format!("must be unique"),
                                                ));
                                            }
                                        }
                                    }
                                    _ => {
                                        return Err(ParameterError::new(
                                            &self.id,
                                            &self.symbol,
                                            format!(
                                                "Parameter {} cannot be compared for uniqueness against {}",
                                                self.symbol,
                                                param.read().unwrap().symbol()
                                            ),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    Comparison::LessThanOrEqual(param) => {
                        if let Some(value) = self.value {
                            match &*param.read().unwrap() {
                                ParameterType::Float(param) => {
                                    if let Some(param_value) = param.value {
                                        if value > param_value {
                                            return Err(ParameterError::new(
                                                &self.id,
                                                &self.symbol,
                                                format!(
                                                    "must be less than or equal to: {}",
                                                    param.name
                                                ),
                                            ));
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                },
                _ => {}
            }
        }
        Ok(())
    }
}

impl Parameter<bool> {
    pub fn validate(&self) -> Result<(), ParameterError> {
        for validation in &self.validations {
            match validation {
                Validation::Required => match &self.value {
                    Some(_value) => {}
                    None => {
                        return Err(ParameterError::new(
                            &self.id,
                            &self.symbol,
                            "required".to_string(),
                        ));
                    }
                },
                _ => {}
            }
        }
        Ok(())
    }
}

impl Parameter<i32> {
    pub fn validate(&self) -> Result<(), ParameterError> {
        for validation in &self.validations {
            match validation {
                Validation::Required => match &self.value {
                    Some(_value) => {}
                    None => {
                        return Err(ParameterError::new(
                            &self.id,
                            &self.symbol,
                            "required".to_string(),
                        ));
                    }
                },
                Validation::Range(min, max) => match &self.value {
                    Some(value) => {
                        let int_as_float = *value as f64;
                        if &int_as_float < min || &int_as_float > max {
                            return Err(ParameterError::new(
                                &self.id,
                                &self.symbol,
                                format!("must be between {} and {}", min, max),
                            ));
                        }
                    }
                    None => {}
                },
                Validation::Min(min) => match &self.value {
                    Some(value) => {
                        let int_as_float = *value as f64;
                        if &int_as_float < min {
                            return Err(ParameterError::new(
                                &self.id,
                                &self.symbol,
                                format!("min: {}", min),
                            ));
                        }
                    }
                    None => {}
                },
                Validation::Max(max) => match &self.value {
                    Some(value) => {
                        let int_as_float = *value as f64;
                        if &int_as_float > max {
                            return Err(ParameterError::new(
                                &self.id,
                                &self.symbol,
                                format!("max: {}", max),
                            ));
                        }
                    }
                    None => {}
                },
                Validation::MinExclusive(min) => match &self.value {
                    Some(value) => {
                        let int_as_float = *value as f64;
                        if &int_as_float <= min {
                            return Err(ParameterError::new(
                                &self.id,
                                &self.symbol,
                                format!("must be greater than {}", min),
                            ));
                        }
                    }
                    None => {}
                },
                Validation::Relation(validation) => match &validation {
                    Comparison::GreaterThanOrEqual(param) => {
                        if let Some(value) = self.value {
                            match &*param.read().unwrap() {
                                ParameterType::Integer(param) => {
                                    if let Some(param_value) = param.value {
                                        if value < param_value {
                                            return Err(ParameterError::new(
                                                &self.id,
                                                &self.symbol,
                                                format!(
                                                    "must be less than or equal to: {}",
                                                    param.name
                                                ),
                                            ));
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    Comparison::Unique(params) => {
                        if let Some(value) = self.value {
                            for param in params {
                                match &*param.read().unwrap() {
                                    ParameterType::Integer(param) => {
                                        if let Some(param_value) = param.value {
                                            if value == param_value {
                                                return Err(ParameterError::new(
                                                    &self.id,
                                                    &self.symbol,
                                                    format!("must be unique"),
                                                ));
                                            }
                                        }
                                    }
                                    _ => {
                                        return Err(ParameterError::new(
                                            &self.id,
                                            &self.symbol,
                                            format!(
                                                "Parameter {} cannot be compared for uniqueness against {}",
                                                self.symbol,
                                                param.read().unwrap().symbol()
                                            ),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    Comparison::LessThanOrEqual(param) => {
                        if let Some(value) = self.value {
                            match &*param.read().unwrap() {
                                ParameterType::Integer(param) => {
                                    if let Some(param_value) = param.value {
                                        if value > param_value {
                                            return Err(ParameterError::new(
                                                &self.id,
                                                &self.symbol,
                                                format!(
                                                    "must be less than or equal to: {}",
                                                    param.name
                                                ),
                                            ));
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                },
                _ => {}
            }
        }
        Ok(())
    }
}

pub type ArcParameter = Arc<RwLock<ParameterType>>;

impl ParameterType {
    pub fn id(&self) -> String {
        match self {
            ParameterType::Float(p) => p.id.clone(),
            ParameterType::OutputFloat(p) => p.id.clone(),
            ParameterType::Integer(p) => p.id.clone(),
            ParameterType::String(p) => p.id.clone(),
            ParameterType::Bool(p) => p.id.clone(),
            ParameterType::Object(p) => p.id.clone(),
            ParameterType::List(p) => p.id.clone(),
            ParameterType::Constant(p) => p.id.clone(),
            ParameterType::StringEnum(p, _) => p.id.clone(),
        }
    }

    pub fn symbol(&self) -> String {
        match self {
            ParameterType::Float(p) => p.symbol.clone(),
            ParameterType::OutputFloat(p) => p.symbol.clone(),
            ParameterType::Integer(p) => p.symbol.clone(),
            ParameterType::String(p) => p.symbol.clone(),
            ParameterType::Bool(p) => p.symbol.clone(),
            ParameterType::Object(p) => p.symbol.clone(),
            ParameterType::List(p) => p.symbol.clone(),
            ParameterType::Constant(p) => p.symbol.clone(),
            ParameterType::StringEnum(p, _) => p.symbol.clone(),
        }
    }
}

pub trait ParameterTrait {
    fn validate(&self) -> Result<(), Vec<ParameterError>>;
    fn to_field(&self) -> Option<ArcField>;
    fn to_field_list(&self, create_function: fn() -> ArcParameter) -> ArcField;
    fn as_float(&self) -> f64;
    fn as_integer(&self) -> i32;
    fn as_string(&self) -> String;
    fn as_list(&self) -> Vec<ArcParameter>;
    fn as_object(&self) -> Object;
    fn get_float(&self) -> Option<f64>;
    fn unique_parameters(&self, params: Vec<ArcParameter>);
    fn update(&self, value: Option<String>) -> Result<(), Vec<ParameterError>>;
    fn display_value(&self) -> String;
    fn id(&self) -> String;
    fn symbol(&self) -> String;
    fn name(&self) -> String;
    fn units(&self) -> Option<String>;
}

impl ParameterTrait for ArcParameter {
    fn validate(&self) -> Result<(), Vec<ParameterError>> {
        let p = self.read().unwrap();
        let mut errors = vec![];

        match &*p {
            ParameterType::Object(parameters) => {
                if let Some(object) = &parameters.value {
                    for parameter in object.values() {
                        if let Err(new_errors) = parameter.validate() {
                            errors.extend(new_errors);
                        }
                    }
                }
                return Ok(());
            }
            ParameterType::List(parameters) => {
                if let Some(parameters) = &parameters.value {
                    for parameter in parameters.iter() {
                        if let Err(new_errors) = parameter.validate() {
                            errors.extend(new_errors);
                        }
                    }
                }
            }
            ParameterType::Float(parameter) => parameter.validate().map_err(|e| vec![e])?,
            ParameterType::Integer(parameter) => parameter.validate().map_err(|e| vec![e])?,
            ParameterType::OutputFloat(parameter) => parameter.validate().map_err(|e| vec![e])?,
            ParameterType::String(parameter) => parameter.validate().map_err(|e| vec![e])?,
            ParameterType::Bool(parameter) => parameter.validate().map_err(|e| vec![e])?,
            ParameterType::Constant(_parameter) => {}
            ParameterType::StringEnum(parameter, _) => parameter.validate().map_err(|e| vec![e])?,
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn to_field(&self) -> Option<ArcField> {
        let p = self.read().unwrap();
        match &*p {
            ParameterType::Float(p) => {
                Some(ArcField::new(RwLock::new(FieldType::Individual(Field {
                    id: p.id.clone(),
                    name: p.name.clone(),
                    value: p.value.as_ref().map(|v| v.to_string()),
                    symbol: p.symbol.clone(),
                    units: p.units.clone(),
                    touched: false,
                    parameter: self.clone(),
                    create_type: None,
                }))))
            }
            ParameterType::OutputFloat(_) => None,
            ParameterType::String(p) => Some(Arc::new(RwLock::new(FieldType::Individual(Field {
                id: p.id.clone(),
                name: p.name.clone(),
                value: p.value.as_ref().map(|v| v.to_string()),
                symbol: p.symbol.clone(),
                units: p.units.clone(),
                touched: false,
                parameter: self.clone(),
                create_type: None,
            })))),
            ParameterType::Integer(p) => {
                Some(Arc::new(RwLock::new(FieldType::Individual(Field {
                    id: p.id.clone(),
                    name: p.name.clone(),
                    value: p.value.as_ref().map(|v| v.to_string()),
                    symbol: p.symbol.clone(),
                    units: p.units.clone(),
                    touched: false,
                    parameter: self.clone(),
                    create_type: None,
                }))))
            }
            ParameterType::StringEnum(p, values) => {
                Some(Arc::new(RwLock::new(FieldType::StringEnum(
                    Field {
                        id: p.id.clone(),
                        name: p.name.clone(),
                        value: p.value.as_ref().map(|v| v.to_string()),
                        symbol: p.symbol.clone(),
                        units: p.units.clone(),
                        touched: false,
                        parameter: self.clone(),
                        create_type: None,
                    },
                    values.clone(),
                ))))
            }
            ParameterType::Bool(p) => Some(Arc::new(RwLock::new(FieldType::Individual(Field {
                id: p.id.clone(),
                name: p.name.clone(),
                value: p.value.as_ref().map(|v| v.to_string()),
                symbol: p.symbol.clone(),
                units: p.units.clone(),
                touched: false,
                parameter: self.clone(),
                create_type: None,
            })))),
            ParameterType::Object(p) => {
                let mut fields = vec![];
                if let Some(parameter) = &p.value {
                    for inner_param in parameter.values() {
                        if let Some(field) = inner_param.to_field() {
                            fields.push(field);
                        }
                    }
                }
                Some(Arc::new(RwLock::new(FieldType::Object(Field {
                    id: p.id.clone(),
                    name: p.name.clone(),
                    value: Some(fields),
                    symbol: p.symbol.clone(),
                    units: p.units.clone(),
                    touched: false,
                    parameter: self.clone(),
                    create_type: None,
                }))))
            }
            ParameterType::List(_) => {
                panic!("Cannot create single field from a list. Use to_field_list() instead")
            }
            ParameterType::Constant(_) => None,
        }
    }

    fn unique_parameters(&self, params: Vec<ArcParameter>) {
        let non_recursive_params = params
            .iter()
            .filter(|p| p.id() != self.id())
            .cloned()
            .collect::<Vec<ArcParameter>>();

        let mut p = self.write().unwrap();

        let get_validations = |v: Vec<Validation>| {
            let mut validations = v
                .into_iter()
                .filter(|v| match v {
                    Validation::Relation(Comparison::Unique(_)) => false,
                    _ => true,
                })
                .collect::<Vec<Validation>>();

            validations.push(Validation::Relation(Comparison::Unique(
                non_recursive_params,
            )));
            return validations;
        };

        match &mut *p {
            ParameterType::Float(v) => {
                let validations = get_validations(v.validations.clone());
                v.validations = validations;
            }
            ParameterType::Integer(v) => {
                let validations = get_validations(v.validations.clone());
                v.validations = validations;
            }
            ParameterType::OutputFloat(v) => {
                let validations = get_validations(v.validations.clone());
                v.validations = validations;
            }
            ParameterType::String(v) => {
                let validations = get_validations(v.validations.clone());
                v.validations = validations;
            }
            ParameterType::Bool(v) => {
                let validations = get_validations(v.validations.clone());
                v.validations = validations;
            }
            ParameterType::Object(_v) => panic!("An object cannot be unique"),
            ParameterType::List(_) => panic!("A List cannot be unique"),
            ParameterType::Constant(_) => panic!("A Constant cannot be unique"),
            ParameterType::StringEnum(_, _) => panic!("A StringEnum cannot be unique"),
        }
    }

    fn to_field_list(&self, create_function: fn() -> ArcParameter) -> ArcField {
        let p = self.read().unwrap();
        match &*p {
            ParameterType::List(p) => {
                let mut fields = vec![];
                if let Some(params) = &p.value {
                    for param in params {
                        if let Some(field) = param.to_field() {
                            fields.push(field)
                        }
                    }
                }
                return Arc::new(RwLock::new(FieldType::List(Field {
                    id: p.id.clone(),
                    name: p.name.clone(),
                    value: Some(fields),
                    symbol: p.symbol.clone(),
                    units: p.units.clone(),
                    touched: false,
                    parameter: self.clone(),
                    create_type: Some(create_function),
                })));
            }
            _ => panic!("Not a list. Use to_field() instead"),
        }
    }

    fn update(&self, value: Option<String>) -> Result<(), Vec<ParameterError>> {
        let mut param = self.write().unwrap();

        match &mut *param {
            ParameterType::Object(_) => panic!("Cannot update Object"),
            ParameterType::List(_) => panic!("Cannot update List"),
            ParameterType::Float(p) => {
                if let Some(value) = value {
                    if let Ok(value) = value.parse::<f64>() {
                        p.value = Some(value);
                    } else if value.is_empty() {
                        p.value = None;
                    } else {
                        return Err(vec![ParameterError::new(
                            &p.id,
                            &p.symbol,
                            "Invalid Number".to_string(),
                        )]);
                    }
                } else {
                    p.value = None
                }
            }
            ParameterType::Integer(p) => {
                if let Some(value) = value {
                    if let Ok(value) = value.parse::<i32>() {
                        p.value = Some(value);
                    } else if value.is_empty() {
                        p.value = None;
                    } else {
                        return Err(vec![ParameterError::new(
                            &p.id,
                            &p.symbol,
                            "Invalid Integer".to_string(),
                        )]);
                    }
                } else {
                    p.value = None
                }
            }
            ParameterType::OutputFloat(p) => {
                if let Some(value) = value {
                    if let Ok(value) = value.parse::<f64>() {
                        p.value = Some(value);
                    } else if value.is_empty() {
                        p.value = None;
                    } else {
                        return Err(vec![ParameterError::new(
                            &p.id,
                            &p.symbol,
                            "Invalid Float".to_string(),
                        )]);
                    }
                } else {
                    p.value = None
                }
            }
            ParameterType::Bool(p) => {
                if let Some(value) = value {
                    if value.to_lowercase() == "true" {
                        p.value = Some(true);
                    } else if value.to_lowercase() == "false" {
                        p.value = Some(false);
                    } else if value.is_empty() {
                        p.value = None;
                    } else {
                        return Err(vec![ParameterError::new(
                            &p.id,
                            &p.symbol,
                            "Invalid Bool".to_string(),
                        )]);
                    }
                } else {
                    p.value = None
                }
            }
            ParameterType::String(p) => {
                if let Some(value) = value {
                    p.value = Some(value);
                } else {
                    p.value = None
                }
            }
            ParameterType::StringEnum(p, values) => {
                if let Some(value) = value {
                    if values.contains(&value) {
                        p.value = Some(value);
                    } else {
                        return Err(vec![ParameterError::new(
                            &p.id,
                            &p.symbol,
                            "Invalid String Enum".to_string(),
                        )]);
                    }
                } else {
                    p.value = None
                }
            }
            ParameterType::Constant(_) => {
                return Err(vec![ParameterError::new(
                    &param.id(),
                    &param.symbol(),
                    "Cannot update constant".to_string(),
                )]);
            }
        }

        std::mem::drop(param);

        self.validate()?;

        Ok(())
    }

    fn as_float(&self) -> f64 {
        let p = self.read().unwrap();
        match &*p {
            ParameterType::Float(float) => match float.value {
                Some(v) => v,
                _ => panic!("Value should be a float"),
            },
            ParameterType::OutputFloat(float) => match float.value {
                Some(v) => v,
                _ => panic!("Value should be a float"),
            },
            _ => panic!("Value should be a float"),
        }
    }

    fn as_integer(&self) -> i32 {
        let p = self.read().unwrap();
        match &*p {
            ParameterType::Integer(integer) => match integer.value {
                Some(v) => v,
                _ => panic!("Value should be a float"),
            },
            _ => panic!("Value should be an integer"),
        }
    }

    fn get_float(&self) -> Option<f64> {
        let p = self.read().unwrap();
        match &*p {
            ParameterType::Float(float) => match float.value {
                Some(v) => Some(v),
                _ => None,
            },
            _ => None,
        }
    }

    fn as_list(&self) -> Vec<ArcParameter> {
        let p = self.read().unwrap();
        match &*p {
            ParameterType::List(list) => match &list.value {
                Some(v) => v.clone(),
                _ => panic!("Value should be a list"),
            },
            _ => panic!("Value should be a list"),
        }
    }

    fn as_object(&self) -> Object {
        let p = self.read().unwrap();
        match &*p {
            ParameterType::Object(list) => match &list.value {
                Some(v) => v.clone(),
                _ => panic!("Value should be a list"),
            },
            _ => panic!("Value should be a list"),
        }
    }

    fn as_string(&self) -> String {
        let p = self.read().unwrap();
        match &*p {
            ParameterType::String(string) => match &string.value {
                Some(v) => v.clone(),
                _ => panic!("Value should be a string"),
            },
            ParameterType::StringEnum(v, _) => match &v.value {
                Some(v) => v.clone(),
                _ => panic!("Value should be a string"),
            },
            _ => panic!("Value should be a string"),
        }
    }

    fn display_value(&self) -> String {
        let p = self.read().unwrap();
        match &*p {
            ParameterType::String(value) => match value.value.clone() {
                Some(v) => v.clone(),
                _ => "".to_string(),
            },
            ParameterType::StringEnum(value, _) => match value.value.clone() {
                Some(v) => v.clone(),
                _ => "".to_string(),
            },
            ParameterType::Integer(param) => {
                if let Some(value) = param.value {
                    value.to_string()
                } else {
                    "".to_string()
                }
            }
            ParameterType::Float(param) => {
                let decimal_places = param.display_options.decimal_places();

                if let (Some(decimal_places), Some(value)) = (decimal_places, param.value) {
                    format!("{:.1$}", value, decimal_places as usize)
                } else {
                    "".to_string()
                }
            }
            ParameterType::OutputFloat(param) => {
                let decimal_places = param.display_options.decimal_places();

                if let (Some(decimal_places), Some(value)) = (decimal_places, param.value) {
                    format!("{:.1$}", value, decimal_places as usize)
                } else {
                    "".to_string()
                }
            }
            ParameterType::Constant(param) => param
                .value
                .clone()
                .map(|v| v.display_value())
                .unwrap_or("".to_string()),
            ParameterType::Bool(_) => panic!("Bool not supported"),
            ParameterType::Object(_) => panic!("Object not supported"),
            ParameterType::List(_) => panic!("List not supported"),
        }
    }

    fn id(&self) -> String {
        match &*self.read().unwrap() {
            ParameterType::Float(p) => p.id.clone(),
            ParameterType::Integer(p) => p.id.clone(),
            ParameterType::OutputFloat(p) => p.id.clone(),
            ParameterType::String(p) => p.id.clone(),
            ParameterType::Bool(p) => p.id.clone(),
            ParameterType::Object(p) => p.id.clone(),
            ParameterType::List(p) => p.id.clone(),
            ParameterType::Constant(p) => p.id.clone(),
            ParameterType::StringEnum(p, _) => p.id.clone(),
        }
    }
    fn name(&self) -> String {
        match &*self.read().unwrap() {
            ParameterType::Float(p) => p.name.clone(),
            ParameterType::Integer(p) => p.name.clone(),
            ParameterType::OutputFloat(p) => p.name.clone(),
            ParameterType::String(p) => p.name.clone(),
            ParameterType::Bool(p) => p.name.clone(),
            ParameterType::Object(p) => p.name.clone(),
            ParameterType::List(p) => p.name.clone(),
            ParameterType::Constant(p) => p.name.clone(),
            ParameterType::StringEnum(p, _) => p.name.clone(),
        }
    }
    fn symbol(&self) -> String {
        match &*self.read().unwrap() {
            ParameterType::Float(p) => p.symbol.clone(),
            ParameterType::Integer(p) => p.symbol.clone(),
            ParameterType::OutputFloat(p) => p.symbol.clone(),
            ParameterType::String(p) => p.symbol.clone(),
            ParameterType::Bool(p) => p.symbol.clone(),
            ParameterType::Object(p) => p.symbol.clone(),
            ParameterType::List(p) => p.symbol.clone(),
            ParameterType::Constant(p) => p.symbol.clone(),
            ParameterType::StringEnum(p, _) => p.symbol.clone(),
        }
    }
    fn units(&self) -> Option<String> {
        match &*self.read().unwrap() {
            ParameterType::Float(p) => p.units.clone(),
            ParameterType::Integer(p) => p.units.clone(),
            ParameterType::OutputFloat(p) => p.units.clone(),
            ParameterType::String(p) => p.units.clone(),
            ParameterType::Bool(p) => p.units.clone(),
            ParameterType::Object(p) => p.units.clone(),
            ParameterType::List(p) => p.units.clone(),
            ParameterType::Constant(p) => p.units.clone(),
            ParameterType::StringEnum(p, _) => p.units.clone(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::sync::{Arc, RwLock};

//     fn create_parameter(value: ParameterValue) -> Arc<RwLock<ParameterType>> {
//         Arc::new(RwLock::new(Parameter {
//             id: "test".to_string(),
//             value: Some(value),
//             validations: vec![],
//             expression: None,
//             name: "Test".to_string(),
//             units: None,
//             display_options: vec![],
//         }))
//     }

//     #[test]
//     fn test_update_float() {
//         let parameter = create_parameter(ParameterValue::Float(0.0));
//         let result = parameter.update(Some("42.0".to_string()));
//         assert!(result.is_ok());
//         assert_eq!(
//             parameter.read().unwrap().value,
//             Some(ParameterValue::Float(42.0))
//         );
//     }

//     #[test]
//     fn test_update_invalid_float() {
//         let parameter = create_parameter(ParameterValue::Float(0.0));
//         let result = parameter.update(Some("invalid".to_string()));
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_update_bool_true() {
//         let parameter = create_parameter(ParameterValue::Bool(false));
//         let result = parameter.update(Some("true".to_string()));
//         assert!(result.is_ok());
//         assert_eq!(
//             parameter.read().unwrap().value,
//             Some(ParameterValue::Bool(true))
//         );
//     }

//     #[test]
//     fn test_update_bool_false() {
//         let parameter = create_parameter(ParameterValue::Bool(true));
//         let result = parameter.update(Some("false".to_string()));
//         assert!(result.is_ok());
//         assert_eq!(
//             parameter.read().unwrap().value,
//             Some(ParameterValue::Bool(false))
//         );
//     }

//     #[test]
//     fn test_update_invalid_bool() {
//         let parameter = create_parameter(ParameterValue::Bool(false));
//         let result = parameter.update(Some("invalid".to_string()));
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_update_string() {
//         let parameter = create_parameter(ParameterValue::String("old".to_string()));
//         let result = parameter.update(Some("new".to_string()));
//         assert!(result.is_ok());
//         assert_eq!(
//             parameter.read().unwrap().value,
//             Some(ParameterValue::String("new".to_string()))
//         );
//     }
// }
