use std::env;
use crate::parser::ast::accessible::Accessible;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub(crate) struct EnvObject { }

impl EnvObject {

    pub(crate) fn get_value(&self, key: &str) -> Value {
        match env::var(key) {
            Ok(s) => Value::String(s),
            Err(_) => Value::Null,
        }
    }

    pub(crate) fn set_value(&self, key: &str, value: &Value) {
        env::set_var(key, value.as_str().unwrap())
    }
}
