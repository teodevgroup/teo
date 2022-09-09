use async_trait::async_trait;
use crate::core::pipeline::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::stage::Stage;
use crate::core::value::Value;


#[derive(Debug, Copy, Clone)]
pub struct ObjectValueModifier {
    key: &'static str
}

impl ObjectValueModifier {
    pub fn new(key: &'static str) -> Self {
        ObjectValueModifier { key }
    }
}

#[async_trait]
impl Modifier for ObjectValueModifier {

    fn name(&self) -> &'static str {
        "object_value"
    }

    async fn call(&self, _stage: Stage, object: &Object) -> Stage {
        let map = object.inner.value_map.lock().unwrap();
        let value = map.get(self.key);
        if value.is_none() {
            Stage::Value(Value::Null)
        } else {
            Stage::Value(value.unwrap().clone())
        }
    }
}
