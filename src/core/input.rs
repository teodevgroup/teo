use std::collections::HashMap;
use crate::core::input::Input::{AtomicUpdator, SetValue};
use crate::core::teon::Value;


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

    pub(crate) fn has_i_mode(map: &HashMap<String, Value>) -> bool {
        match map.get("mode") {
            Some(val) => {
                if let Some(str) = val.as_str() {
                    return str == "caseInsensitive"
                } else {
                    false
                }
            }
            None => {
                false
            }
        }
    }

    pub(crate) fn has_negative_take(json_value: &Value) -> bool {
        if json_value.is_hashmap() {
            let take = json_value.as_hashmap().unwrap().get("take");
            if take.is_some() {
                let take = take.unwrap();
                if take.is_number() {
                    let take = take.as_i64().unwrap();
                    return take < 0;
                }
            }
        }
        false
    }
}
