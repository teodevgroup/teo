use std::collections::HashSet;
use key_path::KeyPath;
use serde_json::{Value as JsonValue};
use crate::core::error::ActionError;

pub(crate) fn check_json_keys<'a>(value: &JsonValue, allowed: &HashSet<&str>, path: &KeyPath<'a>) -> Result<(), ActionError> {
    if let Some(unallowed) = value.as_object().unwrap().keys().find(|k| !allowed.contains(k.as_str())) {
        return Err(ActionError::unexpected_input_key(unallowed, path + unallowed));
    }
    Ok(())
}
