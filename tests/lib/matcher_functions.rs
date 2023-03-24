use serde_json::Value;

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
