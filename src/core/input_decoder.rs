use serde_json::{Value as JsonValue};
use chrono::{Date, DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromStr;
use crate::core::field::{Field, Optionality};
use crate::core::field::r#type::FieldType;
use crate::core::graph::Graph;
use crate::core::input::AtomicUpdateType::{Decrement, Divide, Increment, Multiply, Push};
use crate::core::input::{Input, RelationInputType};
use crate::core::input::Input::{AtomicUpdate, SetValue};
use crate::core::object::Object;
use crate::core::relation::Relation;
use crate::core::value::Value;
use crate::core::error::ActionError;

enum NumberInputType {
    Int,
    UInt,
    Float,
    Decimal,
}

pub(crate) fn input_to_vec(json_value: &JsonValue) -> Result<Vec<&JsonValue>, ActionError> {
    if json_value.is_object() {
        Ok(vec![json_value])
    } else if json_value.is_array() {
        let array = json_value.as_array().unwrap();
        let mapped: Vec<&JsonValue> = array.iter().map(|i| i).collect();
        Ok(mapped)
    } else {
        Err(ActionError::wrong_input_type())
    }
}

pub(crate) fn one_length_json_obj<'a>(json_value: &'a JsonValue, path: &str) -> Result<(&'a str, &'a JsonValue), ActionError> {
    let json_obj = json_value.as_object().unwrap();
    if json_obj.keys().len() != 1 {
        Err(ActionError::wrong_input_updator())
    } else {
        for (key, value) in json_obj {
            return Ok((key.as_str(), value));
        }
        Err(ActionError::wrong_input_updator())
    }
}

fn decode_null(field_type: &FieldType, optionality: Optionality, path: &str) -> Result<Input, ActionError> {
    if optionality == Optionality::Optional {
        Ok(SetValue(Value::Null))
    } else {
        Err(ActionError::unexpected_null(path))
    }
}

fn number_to_target_type(json_value: &JsonValue, target: &FieldType, number_type: NumberInputType) -> Result<Value, ActionError> {
    match number_type {
        NumberInputType::Int => {
            match json_value.as_i64() {
                None => Err(ActionError::wrong_input_type()),
                Some(n) => match target {
                    FieldType::I8 => Ok(Value::I8(n as i8)),
                    FieldType::I16 => Ok(Value::I16(n as i16)),
                    FieldType::I32 => Ok(Value::I32(n as i32)),
                    FieldType::I64 => Ok(Value::I64(n as i64)),
                    FieldType::I128 => Ok(Value::I128(n as i128)),
                    _ => panic!()
                }
            }
        }
        NumberInputType::UInt => {
            match json_value.as_u64() {
                None => Err(ActionError::wrong_input_type()),
                Some(n) => match target {
                    FieldType::U8 => Ok(Value::U8(n as u8)),
                    FieldType::U16 => Ok(Value::U16(n as u16)),
                    FieldType::U32 => Ok(Value::U32(n as u32)),
                    FieldType::U64 => Ok(Value::U64(n as u64)),
                    FieldType::U128 => Ok(Value::U128(n as u128)),
                    _ => panic!()
                }
            }
        }
        NumberInputType::Float => {
            match json_value.as_f64() {
                None => Err(ActionError::wrong_input_type()),
                Some(n) => match target {
                    FieldType::F32 => Ok(Value::F32(n as f32)),
                    FieldType::F64 => Ok(Value::F64(n as f64)),
                    _ => panic!()
                }
            }
        }
        NumberInputType::Decimal => {
            match json_value.as_str() {
                None => Err(ActionError::wrong_input_type()),
                Some(str) => {
                    match Decimal::from_str(str) {
                        Ok(decimal) => Ok(Value::Decimal(decimal)),
                        Err(_err) => Err(ActionError::wrong_input_type())
                    }
                }
            }
        }
    }
}

pub(crate) fn str_to_target_type(json_str: &str, target: &FieldType, graph: &Graph) -> Result<Value, ActionError> {
    match target {
        #[cfg(feature = "data-source-mongodb")]
        FieldType::ObjectId => Ok(Value::ObjectId(json_str.to_string())),
        FieldType::String => Ok(Value::String(json_str.to_string())),
        FieldType::Date => match NaiveDate::parse_from_str(json_str, "%Y-%m-%d") {
            Ok(naive_date) => {
                let date: Date<Utc> = Date::from_utc(naive_date, Utc);
                Ok(Value::Date(date))
            }
            Err(_) => {
                Err(ActionError::wrong_date_format())
            }
        }
        FieldType::DateTime => match DateTime::parse_from_rfc3339(json_str) {
            Ok(fixed_offset_datetime) => {
                let datetime: DateTime<Utc> = fixed_offset_datetime.with_timezone(&Utc);
                Ok(Value::DateTime(datetime))
            }
            Err(_) => {
                Err(ActionError::wrong_datetime_format())
            }
        }
        FieldType::Enum(enum_name) => {
            let enums = graph.enums();
            let vals = enums.get(&enum_name.to_string()).unwrap();
            if vals.values.contains(&json_str.to_string()) {
                Ok(Value::String(json_str.into()))
            } else {
                Err(ActionError::wrong_enum_choice())
            }
        }
        _ => panic!("Unknown json string type.")
    }
}

fn decode_string_input(graph: &Graph, json_value: &JsonValue, field_type: &FieldType, optionality: Optionality, path: &str) -> Result<Input, ActionError> {
    if json_value.is_string() {
        Ok(SetValue(str_to_target_type(json_value.as_str().unwrap(), field_type, graph)?))
    } else if json_value.is_object() {
        let (key, value) = one_length_json_obj(json_value, path)?;
        match key {
            "set" => {
                match value {
                    JsonValue::Null => {
                        decode_null(field_type, optionality, path)
                    }
                    JsonValue::String(string_value) => {
                        Ok(SetValue(str_to_target_type(string_value.as_str(), field_type, graph)?))
                    }
                    _ => {
                        Err(ActionError::wrong_input_type())
                    }
                }
            }
            _ => {
                Err(ActionError::wrong_input_updator())
            }
        }
    } else {
        Err(ActionError::wrong_input_type())
    }
}

fn decode_bool_input(json_value: &JsonValue, field_type: &FieldType, optionality: Optionality, path: &str) -> Result<Input, ActionError> {
    if json_value.is_boolean() {
        Ok(SetValue(Value::Bool(json_value.as_bool().unwrap())))
    } else if json_value.is_object() {
        let (key, value) = one_length_json_obj(json_value, path)?;
        match key {
            "set" => {
                match value {
                    JsonValue::Null => {
                        decode_null(field_type, optionality, path)
                    }
                    JsonValue::Bool(bool_value) => {
                        Ok(SetValue(Value::Bool(*bool_value)))
                    }
                    _ => {
                        Err(ActionError::wrong_input_type())
                    }
                }
            }
            _ => {
                Err(ActionError::wrong_input_updator())
            }
        }
    } else {
        Err(ActionError::wrong_input_type())
    }
}

fn decode_vec_input(graph: &Graph, json_value: &JsonValue, field_type: &FieldType, optionality: Optionality, path: &str, inner_field: &Box<Field>) -> Result<Input, ActionError> {
    if json_value.is_array() {
        let arr = json_value.as_array().unwrap();
        Ok(SetValue(Value::Vec(arr.iter().enumerate().map(|(i, v)| {
            let new_path_name = path.to_string() + "." + &format!("{}", i);
            match decode_field_input(graph, v, &inner_field.field_type, inner_field.optionality, &new_path_name) {
                Ok(v) => {
                    match v {
                        SetValue(v) => v,
                        _ => panic!()
                    }
                }
                Err(_e) => {
                    Value::Null
                }
            }
        }).collect())))
    } else if json_value.is_object() {
        let (key, value) = one_length_json_obj(json_value, path)?;
        match key {
            "set" => {
                match value {
                    JsonValue::Null => {
                        decode_null(&field_type, optionality, path)
                    }
                    JsonValue::Array(arr) => {
                        Ok(SetValue(Value::Vec(arr.iter().enumerate().map(|(i, v)| {
                            let new_path_name = path.to_string() + "." + &format!("{}", i);
                            match decode_field_input(graph, v, field_type, optionality, &new_path_name) {
                                Ok(v) => {
                                    match v {
                                        SetValue(v) => v,
                                        _ => panic!()
                                    }
                                }
                                Err(_err) => {
                                    Value::Null
                                }
                            }
                        }).collect())))
                    }
                    _ => {
                        Err(ActionError::wrong_input_type())
                    }
                }
            }
            "push" => {
                let inner_val = match decode_field_input(graph, value, &inner_field.field_type, inner_field.optionality, path)? {
                    SetValue(val) => val,
                    _ => return Err(ActionError::wrong_input_type())
                };
                Ok(AtomicUpdate(Push(inner_val)))
            }
            _ => {
                Err(ActionError::wrong_input_type())
            }
        }
    } else {
        Err(ActionError::wrong_input_type())
    }
}

fn decode_number_input(json_value: &JsonValue, field_type: &FieldType, optionality: Optionality, path: &str) -> Result<Input, ActionError> {
    let number_type = match field_type {
        FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 => NumberInputType::Int,
        FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 => NumberInputType::UInt,
        FieldType::F32 | FieldType::F64 => NumberInputType::Float,
        FieldType::Decimal => NumberInputType::Decimal,
        _ => panic!("Wrong number input type.")
    };
    if json_value.is_string() {
        match number_type {
            NumberInputType::Decimal => {
                Ok(SetValue(number_to_target_type(json_value, field_type, number_type)?))
            }
            _ => Err(ActionError::wrong_input_type())
        }
    } else if json_value.is_number() {
        Ok(SetValue(number_to_target_type(json_value, field_type, number_type)?))
    } else if json_value.is_object() {
        let (key, value) = one_length_json_obj(json_value, path)?;
        let arg = match value {
            JsonValue::Null => {
                return if key == "set" {
                    decode_null(field_type, optionality, path)
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            JsonValue::Number(_num) => {
                number_to_target_type(value, field_type, number_type)
            }
            JsonValue::String(_str) => {
                match number_type {
                    NumberInputType::Decimal => {
                        number_to_target_type(value, field_type, number_type)
                    }
                    _ => Err(ActionError::wrong_input_type())
                }
            }
            _ => {
                Err(ActionError::wrong_input_type())
            }
        }?;
        match key {
            "set" => Ok(SetValue(arg)),
            "increment" => Ok(AtomicUpdate(Increment(arg))),
            "decrement" => Ok(AtomicUpdate(Decrement(arg))),
            "multiply" => Ok(AtomicUpdate(Multiply(arg))),
            "divide" => Ok(AtomicUpdate(Divide(arg))),
            _ => {
                Err(ActionError::wrong_input_updator())
            }
        }
    } else {
        Err(ActionError::wrong_input_type())
    }
}

pub(crate) fn decode_field_value(graph: &Graph, json_value: &JsonValue, field: &Field, path: &str) -> Result<Value, ActionError> {
    match decode_field_input(graph, json_value, &field.field_type, field.optionality, path) {
        Ok(input) => {
            match input {
                Input::SetValue(value) => {
                    Ok(value)
                }
                _ => {
                    Err(ActionError::wrong_input_type())
                }
            }
        }
        Err(err) => {
            Err(err)
        }
    }
}

pub(crate) fn decode_field_input(graph: &Graph, json_value: &JsonValue, field_type: &FieldType, optionality: Optionality, path: &str) -> Result<Input, ActionError> {
    // value is JSON null
    if json_value == &JsonValue::Null {
        return if optionality == Optionality::Optional {
            Ok(SetValue(Value::Null))
        } else {
            Err(ActionError::unexpected_null(path))
        }
    }
    // value is present
    match field_type {
        FieldType::Undefined => { panic!("Field type should not be undefined!") }
        #[cfg(feature = "data-source-mongodb")]
        FieldType::ObjectId => {
            decode_string_input(graph, json_value, field_type, optionality, path)
        }
        FieldType::String | FieldType::Date | FieldType::DateTime | FieldType::Enum(_) => {
            decode_string_input(graph, json_value, field_type, optionality, path)
        }
        FieldType::Bool => {
            decode_bool_input(json_value, field_type, optionality, path)
        }
        FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 |
        FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 |
        FieldType::F32 | FieldType::F64 | FieldType::Decimal => {
            decode_number_input(json_value, field_type, optionality, path)
        }
        FieldType::Vec(inner_field) => {
            decode_vec_input(graph, json_value, field_type, optionality, path, inner_field)
        }
        _ => panic!()
    }
}

pub(crate) fn decode_relation_input( _object: &Object, json_value: &JsonValue, relation: &Relation, path: &str) -> Result<Input, ActionError> {
    if !json_value.is_object() {
        return Err(ActionError::wrong_input_type());
    }
    let (key, value) = one_length_json_obj(json_value, path)?;
    let input = match key {
        "create" => RelationInputType::Create(value.clone()),
        "createMany" => if relation.is_vec {
            RelationInputType::Create(value.clone())
        } else {
            return Err(ActionError::invalid_input(path, "Single relationship cannot create many."))
        },
        "set" => RelationInputType::Set(value.clone()),
        "connect" => RelationInputType::Connect(value.clone()),
        "connectOrCreate" => RelationInputType::ConnectOrCreate(value.clone(), value.clone()),
        "disconnect" => RelationInputType::Disconnect(value.clone()),
        "update" => RelationInputType::Update(value.clone()),
        "updateMany" => if relation.is_vec {
            RelationInputType::Update(value.clone())
        } else {
            return Err(ActionError::invalid_input(path, "Single relationship cannot update many."))
        },
        "upsert" => RelationInputType::Upsert(value.clone(), value.clone()),
        "delete" => RelationInputType::Delete(value.clone()),
        "deleteMany" => if relation.is_vec {
            RelationInputType::Delete(value.clone())
        } else {
            return Err(ActionError::invalid_input(path, "Single relationship cannot delete many."))
        },
        _ => panic!()
    };
    Ok(Input::RelationInput(input))
}
