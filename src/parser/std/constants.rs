use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use maplit::hashmap;
use crate::parser::ast::object::Object;
use crate::prelude::Value;

struct EnvObject { }

impl Object for EnvObject {
    fn get_object(&self, key: &str) -> Arc<dyn Object> {
        panic!("ENV has no properties.")
    }

    fn set_object(&mut self, key: &str, obj: Arc<dyn Object>) {
        panic!("ENV has no properties.")
    }

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

pub(crate) struct GlobalObject {
    objects: HashMap<String, Arc<dyn Object>>
}

impl GlobalObject {
    pub(crate) fn new() -> GlobalObject {
        let mut objects: HashMap<String, Arc<dyn Object>> = HashMap::new();
        objects.insert("ENV".to_owned(), Arc::new(EnvObject {}));
        Self { objects }
    }
}

impl Object for GlobalObject {

    fn get_object(&self, key: &str) -> Arc<dyn Object> {
        match self.objects.get(key) {
            Some(o) => o.clone(),
            None => panic!("Object with key '{}' is not found.", key),
        }
    }

    fn set_object(&mut self, key: &str, obj: Arc<dyn Object>) {
        panic!("Std global object is readonly.")
    }
    
    fn get_value(&self, _key: &str) -> Arc<Value> {
        panic!("Std global object cannot behave as dictionary.")
    }

    fn set_value(&self, _key: &str, _value: &Value) {
        panic!("Cannot set value to std global object.")
    }
}
