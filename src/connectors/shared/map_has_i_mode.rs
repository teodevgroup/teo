use serde_json::{Value as JsonValue, Map as JsonMap};

pub fn map_has_i_mode(map: &JsonMap<String, JsonValue>) -> bool {
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
