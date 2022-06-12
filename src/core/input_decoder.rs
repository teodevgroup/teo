use serde_json::{Map, Value as JsonValue};
use chrono::{Date, DateTime, NaiveDate, Utc};

use crate::core::field::{Field, Optionality};
use crate::core::field_type::FieldType;
use crate::core::graph::Graph;
use crate::core::input::Input;
use crate::core::input::Input::SetValue;
use crate::core::object::Object;
use crate::core::relation::Relation;
use crate::core::value::Value;
use crate::error::ActionError;


enum NumberInputType {
    Int,
    UInt,
    Float,
    Decimal,
}

struct InputDecoder { }

impl InputDecoder {

    fn one_length_json_obj(&self, json_value: &JsonValue, path: &str) -> Result<(&str, &JsonValue), ActionError> {
        let json_obj = json_value.as_object().unwrap();
        if json_obj.keys().len() != 1 {
            Err(ActionError::wrong_input_updator(path))
        } else {
            for (key, value) in json_obj {
                Ok((key.as_str(), value))
            }
        }
    }

    fn decode_null(&self, field: &Field, path: &str) -> Result<Input, ActionError> {
        if field.optionality == Optionality::Optional {
            Ok(SetValue(Value::Null))
        } else {
            Err(ActionError::unexpected_null(path))
        }
    }

    fn str_to_target_type(&self, json_str: &str, target: &FieldType, graph: &Graph) -> Result<Value, ActionError> {
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
                if vals.contains(&json_str.to_string()) {
                    Ok(Value::String(json_str.into()))
                } else {
                    Err(ActionError::wrong_enum_choice())
                }
            }
            _ => panic!("Unknown json string type.")
        }
    }

    fn decode_string_input(&self, object: &Object, json_value: &JsonValue, field: &Field, path: &str) -> Result<Input, ActionError> {
        if json_value.is_string() {
            Ok(SetValue(self.str_to_target_type(json_value.as_str().unwrap(), &field.field_type, object.graph())?))
        } else if json_value.is_object() {
            let (key, value) = self.one_length_json_obj(json_value, path)?;
            match key {
                "set" => {
                    match value {
                        JsonValue::Null => {
                            self.decode_null(field, path)
                        }
                        JsonValue::String(string_value) => {
                            Ok(SetValue(self.str_to_target_type(string_value.as_str(), &field.field_type, object.graph())?))
                        }
                        _ => {
                            Err(ActionError::wrong_input_type())
                        }
                    }
                }
                _ => {
                    Err(ActionError::wrong_input_updator(path))
                }
            }
        } else {
            Err(ActionError::wrong_input_type())
        }
    }

    fn decode_bool_input(&self, object: &Object, json_value: &JsonValue, field: &Field, path: &str) -> Result<Input, ActionError> {
        let is_new = object.is_new();
        if json_value.is_boolean() {
            Ok(SetValue(Value::Bool(json_value.as_bool().unwrap())))
        } else if json_value.is_object() {
            let (key, value) = self.one_length_json_obj(json_value, path)?;
            match key {
                "set" => {
                    match value {
                        JsonValue::Null => {
                            self.decode_null(field, path)
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
                    Err(ActionError::wrong_input_updator(path))
                }
            }
        } else {
            Err(ActionError::wrong_input_type())
        }
    }

    pub(crate) fn decode_field_input(&self, object: &Object, json_value: &JsonValue, field: &Field, path: &str) -> Result<Input, ActionError> {
        // value is JSON null
        if json_value == &JsonValue::Null {
            return if field.optionality == Optionality::Optional {
                Ok(SetValue(Value::Null))
            } else {
                Err(ActionError::unexpected_null(path_name))
            }
        }
        // value is present
        let is_new = object.is_new();
        let graph = object.graph();
        match &field.field_type {
            FieldType::Undefined => { panic!("Field type should not be undefined!") }
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => {
                self.decode_string_input(object, json_value, field, &field.name)
            }
            FieldType::String | FieldType::Date | FieldType::DateTime | FieldType::Enum(_) => {
                self.decode_string_input(object, json_value, field, &field.name)
            }
            FieldType::Bool => {
                self.decode_bool_input(object, json_value, field, &field.name)
            }

        }
        Ok(SetValue(Value::Null))
    }

    fn decode_relation_input(&self, object: Object, json_value: &JsonValue, relation: &Relation, path_name: &str) -> Result<Input, ActionError> {

    }
}