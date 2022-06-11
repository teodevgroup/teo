use std::str::FromStr;
use serde_json::{Value as JsonValue};
use chrono::prelude::{Date, NaiveDate, Utc, DateTime};
use rust_decimal::Decimal;
use crate::core::field::Field;
use crate::core::graph::Graph;
use crate::core::value::Value;
use crate::error::ActionError;

#[derive(Debug, Clone)]
pub(crate) enum FieldType {
    Undefined,
    ObjectId,
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Decimal,
    String,
    Date,
    DateTime,
    Enum(&'static str),
    Vec(Box<Field>),
    Map(Box<Field>),
    Object(&'static str)
}

impl FieldType {

    pub(crate) fn decode_value(&self, json_value: &JsonValue, graph: &'static Graph) -> Result<Value, ActionError> {
        if *json_value == JsonValue::Null {
            return Ok(Value::Null);
        }
        return match self {
            FieldType::Undefined => { Ok(Value::Null) }
            FieldType::ObjectId => {
                if json_value.is_string() {
                    Ok(Value::ObjectId(json_value.as_str().unwrap().to_string()))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::Bool => {
                if json_value.is_boolean() {
                    Ok(Value::Bool(json_value.as_bool().unwrap()))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::I8 => {
                if json_value.is_number() {
                    Ok(Value::I8(json_value.as_i64().unwrap() as i8))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::I16 => {
                if json_value.is_number() {
                    Ok(Value::I16(json_value.as_i64().unwrap() as i16))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::I32 => {
                if json_value.is_number() {
                    Ok(Value::I32(json_value.as_i64().unwrap() as i32))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::I64 => {
                if json_value.is_number() {
                    Ok(Value::I64(json_value.as_i64().unwrap()))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::I128 => {
                if json_value.is_number() {
                    Ok(Value::I128(json_value.as_i64().unwrap() as i128))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::U8 => {
                if json_value.is_number() {
                    Ok(Value::U8(json_value.as_i64().unwrap() as u8))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::U16 => {
                if json_value.is_number() {
                    Ok(Value::U16(json_value.as_i64().unwrap() as u16))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::U32 => {
                if json_value.is_number() {
                    Ok(Value::U32(json_value.as_i64().unwrap() as u32))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::U64 => {
                if json_value.is_number() {
                    Ok(Value::U64(json_value.as_i64().unwrap() as u64))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::U128 => {
                if json_value.is_number() {
                    Ok(Value::U128(json_value.as_i64().unwrap() as u128))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::F32 => {
                if json_value.is_number() {
                    Ok(Value::F32(json_value.as_f64().unwrap() as f32))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::F64 => {
                if json_value.is_number() {
                    Ok(Value::F64(json_value.as_f64().unwrap() as f64))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::Decimal => {
                if json_value.is_string() {
                    Ok(Value::Decimal(Decimal::from_str(json_value.as_str().unwrap()).unwrap()))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::String => {
                if json_value.is_string() {
                    Ok(Value::String(String::from(json_value.as_str().unwrap())))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::Date => {
                if json_value.is_string() {
                    match NaiveDate::parse_from_str(&json_value.as_str().unwrap(), "%Y-%m-%d") {
                        Ok(naive_date) => {
                            let date: Date<Utc> = Date::from_utc(naive_date, Utc);
                            Ok(Value::Date(date))
                        }
                        Err(_) => {
                            Err(ActionError::wrong_date_format())
                        }
                    }
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::DateTime => {
                if json_value.is_string() {
                    match DateTime::parse_from_rfc3339(&json_value.as_str().unwrap()) {
                        Ok(fixed_offset_datetime) => {
                            let datetime: DateTime<Utc> = fixed_offset_datetime.with_timezone(&Utc);
                            Ok(Value::DateTime(datetime))
                        }
                        Err(_) => {
                            Err(ActionError::wrong_datetime_format())
                        }
                    }
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::Enum(enum_name) => {
                if json_value.is_string() {
                    let string = String::from(json_value.as_str().unwrap());
                    let enums = graph.enums();
                    let vals = enums.get(enum_name).unwrap();
                    if vals.contains(&&*string) {
                        Ok(Value::String(string))
                    } else {
                        Err(ActionError::wrong_enum_choice())
                    }
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::Vec(field) => {
                if !json_value.is_array() {
                    return Err(ActionError::wrong_input_type());
                }
                let arr = json_value.as_array().unwrap();
                Ok(Value::Vec(arr.iter().map(|v| {
                    field.field_type.decode_value(v, graph).unwrap()
                }).collect()))
            }
            _ => {
                panic!()
            }
        }

    }
}
