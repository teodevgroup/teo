use std::collections::HashSet;
use key_path::KeyPath;
use crate::core::error::ActionError;
use crate::core::result::ActionResult;
use crate::prelude::Value;

pub(crate) fn check_json_keys<'a>(value: &Value, allowed: &HashSet<&str>, path: &KeyPath<'a>) -> ActionResult<()> {
    if let Some(unallowed) = value.as_hashmap().unwrap().keys().find(|k| !allowed.contains(k.as_str())) {
        return Err(ActionError::unexpected_input_key(unallowed, path + unallowed));
    }
    Ok(())
}
