use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use maplit::hashmap;
use crate::parser::ast::object::Accessible;
use crate::prelude::Value;

pub(crate) struct EnvObject { }

impl EnvObject {

    fn get_value(&self, key: &str) -> Arc<Value> {
        match env::var(key) {
            Ok(s) => Arc::new(Value::String(s)),
            Err(_) => Arc::new(Value::Null),
        }
    }

    fn set_value(&self, key: &str, value: &Value) {
        env::set_var(key, value.as_str().unwrap())
    }
}
