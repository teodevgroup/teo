use crate::prelude::Value;

pub(crate) fn json_get(value: &Value, path: Vec<String>) -> Option<&Value> {
    let mut retval = Some(value);
    for item in path.iter() {
        if retval == None { return None; }
        if let Some(object) = value.as_object() {
            retval = object.get(item);
        } else if let Some(array) = value.as_array() {
            retval = array.get(item.parse::<usize>().unwrap());
        } else {
            return None;
        }
    }
    retval
}

pub(crate) fn json_has(value: &Value, path: Vec<String>) -> bool {
    json_get(value, path).is_some()
}

pub(crate) fn json_set(object_or_array: &Value, path: Vec<String>, value: Value) -> Value {
    let mut retval = object_or_array.clone();
    let mut cursor = Some(&mut retval);
    let key = path.last().unwrap();
    let path_but_last = &path[0..path.len() - 1];
    for item in path_but_last.iter() {
        if let Some(some_cursor) = cursor {
            if some_cursor.is_object() {
                let object = some_cursor.as_object_mut().unwrap();
                cursor = object.get_mut(item);
            } else if some_cursor.is_array() {
                let array = some_cursor.as_array_mut().unwrap();
                cursor = array.get_mut(item.parse::<usize>().unwrap());
            } else {
                return retval;
            }
        } else {
            return retval;
        }
    }
    if let Some(cursor) = cursor {
        if let Some(object) = cursor.as_object_mut() {
            object.insert(key.clone(), value);
        } else if let Some(array) = cursor.as_array_mut() {
            array.insert(key.parse::<usize>().unwrap(), value);
        } else {
            return retval;
        }
    }
    retval
}
