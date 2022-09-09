use serde_json::{Value as JsonValue};

pub(crate) fn json_get(value: &JsonValue, path: Vec<String>) -> Option<&JsonValue> {
    let mut retval = Some(value);
    for item in path.iter() {
        if retval == None { return None; }
        if let Some(object) = value.as_object() {
            retval = object.get(item);
        } else if let Some(array) = value.as_array() {
            retval = array.get(item.parse::<i32>().unwrap());
        } else {
            return None;
        }
    }
    retval
}
