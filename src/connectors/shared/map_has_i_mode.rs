use std::collections::HashMap;
use crate::prelude::Value;

pub fn map_has_i_mode(map: &HashMap<String, Value>) -> bool {
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
