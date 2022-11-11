use std::sync::Arc;
use crate::core::tson::Value;

pub(crate) trait Object {

    fn get_object(&self, key: &str) -> Arc<dyn Object>;

    fn set_object(&mut self, key: &str, obj: Arc<dyn Object>);

    fn get_value(&self, key: &str) -> Arc<Value>;

    fn set_value(&self, key: &str, value: &Value);
}
