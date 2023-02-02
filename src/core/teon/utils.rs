use key_path::KeyPath;
use crate::core::error::ActionError;
use crate::core::result::ActionResult;
use crate::prelude::Value;

pub(crate) struct TsonUtils { }

impl TsonUtils {

    pub(crate) fn value_set<'a>(root: &Value, path: KeyPath<'a>, value: &Value) -> ActionResult<Value> {
        if path.is_empty() {
            return Ok(value.clone());
        }
        let first_path_item = path.get(0).unwrap();
        let rest = KeyPath::from(&path[1..path.len() - 1]);
        match root {
            Value::HashMap(map) => {
                let mut map = map.clone();
                let item = first_path_item.as_key().unwrap();
                let new_root = map.get(item).unwrap_or(&Value::Null);
                let transformed = TsonUtils::value_set(new_root, rest.clone(), value)?;
                map.insert(first_path_item.as_key().unwrap().to_string(), transformed);
                Ok(Value::HashMap(map))
            }
            Value::BTreeMap(map) => {
                let mut map = map.clone();
                let item = first_path_item.as_key().unwrap();
                let new_root = map.get(item).unwrap_or(&Value::Null);
                let transformed = TsonUtils::value_set(new_root, rest.clone(), value)?;
                map.insert(first_path_item.as_key().unwrap().to_string(), transformed);
                Ok(Value::BTreeMap(map))
            }
            Value::Vec(vec) => {
                let mut vec = vec.clone();
                let item = first_path_item.as_index().unwrap();
                let new_root = vec.get(item).unwrap_or(&Value::Null);
                let transformed = TsonUtils::value_set(new_root, rest.clone(), value)?;
                vec[item] = transformed;
                Ok(Value::Vec(vec))
            }
            _ => {
                Err(ActionError::invalid_operation("Invalid key access."))
            }
        }
    }

    pub(crate) fn value_get<'a>(root: &'a Value, path: KeyPath<'a>) -> ActionResult<Value> {
        if path.is_empty() {
            return Ok(root.clone());
        }
        let first_path_item = path.get(0).unwrap();
        let rest = KeyPath::from(&path[1..path.len() - 1]);
        match root {
            Value::HashMap(map) => {
                let item = first_path_item.as_key().unwrap();
                let new_root = map.get(item).unwrap_or(&Value::Null);
                return TsonUtils::value_get(new_root, rest.clone());
            }
            Value::BTreeMap(map) => {
                let item = first_path_item.as_key().unwrap();
                let new_root = map.get(item).unwrap_or(&Value::Null);
                return TsonUtils::value_get(new_root, rest.clone());
            }
            Value::Vec(vec) => {
                let item = first_path_item.as_index().unwrap();
                let new_root = vec.get(item).unwrap_or(&Value::Null);
                return TsonUtils::value_get(new_root, rest.clone());
            }
            _ => {
                Err(ActionError::invalid_operation("Invalid key access."))
            }
        }
    }

    pub(crate) fn value_has(root: &Value, path: KeyPath) -> ActionResult<bool> {
        let value = Self::value_get(root, path)?;
        Ok(!value.is_null())
    }
}
