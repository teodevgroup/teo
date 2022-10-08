use std::collections::HashMap;
use crate::core::input::Input::{AtomicUpdator, SetValue};
use crate::core::tson::Value;

pub enum RelationInput {
    // both create and update
    Create(Value),
    Set(Value),
    Connect(Value),
    ConnectOrCreate { r#where: Value, create: Value },

    // update only
    Disconnect(Value),
    Update(Value),
    Upsert { r#where: Value, create: Value,  update: Value },
    Delete(Value),
}

pub(crate) enum Input {
    SetValue(Value),
    AtomicUpdator(Value),
}

impl Input {
    pub(crate) fn decode_field(value: &Value) -> Input {
        if let Some(value) = value.as_hashmap() {
            let key = value.keys().next().unwrap();
            let value = value.values().next().unwrap();
            if key.as_str() == "set" {
                SetValue(value.clone())
            } else {
                AtomicUpdator(value.clone())
            }
        } else {
            SetValue(value.clone())
        }
    }

    pub(crate) fn key_value(value: &HashMap<String, Value>) -> (&str, &Value) {
        (value.keys().next().unwrap().as_str(), value.values().next().unwrap())
    }

    // pub(crate) fn decode_relation(value: &Value) -> Input {
    //
    // }
}
