use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use maplit::hashmap;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;
use uuid::Uuid;
use crate::lib::json_match;
use crate::lib::matcher::Matcher;

pub fn date_time_value(val: impl AsRef<str>) -> impl Fn(&Value) -> bool {
    return move |v: &Value| {
        if !v.is_object() { return false }
        let obj = v.as_object().unwrap();
        if obj.len() != 1 { return false }
        if obj.get("$date").is_none() { return false }
        let date_value = obj.get("$date").unwrap();
        if !date_value.is_string() { return false }
        date_value.as_str().unwrap() == val.as_ref()
    }
}

pub fn decimal_value(val: impl AsRef<str>) -> impl Fn(&Value) -> bool {
    return move |v: &Value| {
        if !v.is_object() { return false }
        let obj = v.as_object().unwrap();
        if obj.len() != 1 { return false }
        if obj.get("$decimal").is_none() { return false }
        let date_value = obj.get("$decimal").unwrap();
        if !date_value.is_string() { return false }
        date_value.as_str().unwrap() == val.as_ref()
    }
}

pub fn object_id_value(v: &Value) -> bool {
    if !v.is_string() { return false }
    let regex = Regex::new("[\\da-f]{24}").unwrap();
    regex.is_match(v.as_str().unwrap())
}

pub fn one_match(matcher: impl Borrow<Matcher>) -> impl Fn(&Value) -> bool {
    return move |v: &Value| {
        if !v.is_array() { return false }
        let array = v.as_array().unwrap();
        for value in array {
            let match_result = json_match(value, matcher.borrow());
            if match_result.is_ok() {
                return true
            }
        }
        false
    }
}

static EQUAL_TOKENS: Lazy<Mutex<HashMap<String, Value>>> = Lazy::new(|| {
    Mutex::new(hashmap! {})
});

pub fn equal_token() -> Arc<dyn Fn(&Value) -> bool> {
    let random_key = Uuid::new_v4().to_string();
    return Arc::new(move |v: &Value| {
        let mut map = EQUAL_TOKENS.lock().unwrap();
        if map.contains_key(&random_key) {
            let fetched_value = map.get(&random_key).unwrap();
            println!("compare: v fv: {} {}", v, fetched_value);
            v == fetched_value
        } else {
            map.insert(random_key.clone(), v.clone());
            true
        }
    })
}