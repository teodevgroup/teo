#[cfg(feature = "data-source-mongodb")]
use bson::oid::ObjectId;
use chrono::{Date, DateTime, NaiveDate, Utc};
use key_path::KeyPath;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromStr;
use crate::core::field::Field;
use crate::core::field::optionality::Optionality;
use crate::core::field::r#type::FieldType;
use crate::core::graph::Graph;
use crate::core::input::AtomicUpdateType::{Decrement, Divide, Increment, Multiply, Push};
use crate::core::input::{Input, RelationInputType};
use crate::core::input::Input::{AtomicUpdate, SetValue};
use crate::core::object::Object;
use crate::core::relation::Relation;
use crate::core::tson::Value;
use crate::core::error::ActionError;

enum NumberInputType {
    Int,
    UInt,
    Float,
    Decimal,
}

pub(crate) fn input_to_vec<'a, 'b>(json_value: &'b Value, path: &'a KeyPath<'a>) -> Result<Vec<&'b Value>, ActionError> {
    if json_value.is_hashmap() {
        Ok(vec![json_value])
    } else if json_value.is_vec() {
        let array = json_value.as_vec().unwrap();
        let mapped: Vec<&Value> = array.iter().map(|i| i).collect();
        Ok(mapped)
    } else {
        Err(ActionError::unexpected_input_type("object or array", path))
    }
}

pub(crate) fn one_length_json_obj<'a>(json_value: &'a Value, path: &KeyPath) -> Result<(&'a str, &'a Value), ActionError> {
    let json_obj = json_value.as_hashmap().unwrap();
    if json_obj.keys().len() != 1 {
        Err(ActionError::unexpected_object_length(1, path))
    } else {
        for (key, value) in json_obj {
            return Ok((key.as_str(), value));
        }
        Err(ActionError::unexpected_object_length(1, path))
    }
}
