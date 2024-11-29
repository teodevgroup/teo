use std::borrow::Borrow;
use std::collections::HashSet;
use key_path::{KeyPath, path};
use serde_json::{Map, Number, Value};
use crate::lib::matcher::Matcher;

pub fn json_match<J: Borrow<Value>, M: Borrow<Matcher>>(value: J, matcher: M) -> Result<(), String> {
    json_match_internal(value.borrow(), matcher.borrow(), &path![])
}

fn json_match_internal(value: &Value, matcher: &Matcher, path: &KeyPath) -> Result<(), String> {
    if matcher.is_ignore() {
        return Ok(());
    }
    match value {
        Value::Null => json_match_null(matcher, path)?,
        Value::String(string) => json_match_string(value, string, matcher, path)?,
        Value::Bool(bool) => json_match_bool(value, bool, matcher, path)?,
        Value::Number(number) => json_match_number(value, number, matcher, path)?,
        Value::Array(array) => json_match_array(value, array, matcher, path)?,
        Value::Object(object) => json_match_object(value, object, matcher, path)?,
    }
    Ok(())
}

fn json_match_null(matcher: &Matcher, path: &KeyPath) -> Result<(), String> {
    if matcher.is_null() {
        return Ok(());
    }
    json_match_error(&Value::Null, path)
}

fn json_match_string(value: &Value, string: &String, matcher: &Matcher, path: &KeyPath) -> Result<(), String> {
    match matcher {
        Matcher::String(s) => json_match_error_if_not(s == string, value, path),
        Matcher::ValueMatcher(m) => json_match_error_if_not(m(value), value, path),
        _ => json_match_error(value, path),
    }
}

fn json_match_bool(value: &Value, bool: &bool, matcher: &Matcher, path: &KeyPath) -> Result<(), String> {
    match matcher {
        Matcher::Bool(b) => json_match_error_if_not(b == bool, value, path),
        Matcher::ValueMatcher(m) => json_match_error_if_not(m(value), value, path),
        _ => json_match_error(value, path),
    }
}

fn json_match_number(value: &Value, number: &Number, matcher: &Matcher, path: &KeyPath) -> Result<(), String> {
    match matcher {
        Matcher::Number(n) => json_match_error_if_not(n == number, value, path),
        Matcher::ValueMatcher(m) => json_match_error_if_not(m(value), value, path),
        _ => json_match_error(value, path),
    }
}

fn json_match_array(value: &Value, array: &Vec<Value>, matcher: &Matcher, path: &KeyPath) -> Result<(), String> {
    match matcher {
        Matcher::Array(a) => {
            json_match_error_if_not(a.len() == array.len(), value, path)?;
            for (index, matcher) in a.iter().enumerate() {
                let array_value = array.get(index).unwrap();
                json_match_internal(array_value, matcher, &(path + index))?;
            }
            Ok(())
        },
        Matcher::ValueMatcher(m) => json_match_error_if_not(m(value), value, path),
        _ => json_match_error(value, path),
    }
}

fn json_match_object(value: &Value, map: &Map<String, Value>, matcher: &Matcher, path: &KeyPath) -> Result<(), String> {
    match matcher {
        Matcher::Object(m, partial) => {
            // compare keys
            if !partial {
                json_match_error_if_not(m.len() == map.len(), value, path)?;
            }
            let m_keys: HashSet<&str> = m.keys().into_iter().map(|k| k.as_str()).collect();
            let map_keys: HashSet<&str> = map.keys().into_iter().map(|k| k.as_str()).collect();
            if *partial {
                json_match_error_if_not(map_keys.is_superset(&m_keys), value, path)?;
            } else {
                json_match_error_if_not(m_keys == map_keys, value, path)?;
            }
            for (key, matcher) in m.iter() {
                let map_value = map.get(key).unwrap();
                json_match_internal(map_value, matcher, &(path + key))?;
            }
            Ok(())
        },
        Matcher::ValueMatcher(m) => json_match_error_if_not(m(value), value, path),
        _ => json_match_error(value, path),
    }
}

fn json_match_error(value: &Value, path: &KeyPath) -> Result<(), String> {
    if path.is_empty() {
        Err(format!("value `{}` does not match json matcher.", value.to_string()))
    } else {
        Err(format!("value `{}` at `{}` does not match json matcher.", value.to_string(), path.to_string()))
    }
}

fn json_match_error_if_not(result: bool, value: &Value, path: &KeyPath) -> Result<(), String> {
    if !result {
        json_match_error(value, path)
    } else {
        Ok(())
    }
}

#[macro_export]
macro_rules! assert_json {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                use std::borrow::Borrow;
                if let Err(string) = $crate::lib::json::json_match(left_val.borrow(), right_val.borrow()) {
                    panic!("{}", string)
                }
            }
        }
    };
}
