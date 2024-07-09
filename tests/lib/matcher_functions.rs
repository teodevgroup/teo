use std::borrow::Borrow;
use regex::Regex;
use serde_json::Value;
use crate::lib::json::json_match;
use crate::lib::matcher::Matcher;

pub fn date_value(val: impl AsRef<str>) -> impl Fn(&Value) -> bool {
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

pub fn date_time_value(val: impl AsRef<str>) -> impl Fn(&Value) -> bool {
    return move |v: &Value| {
        if !v.is_object() { return false }
        let obj = v.as_object().unwrap();
        if obj.len() != 1 { return false }
        if obj.get("$datetime").is_none() { return false }
        let date_value = obj.get("$datetime").unwrap();
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
