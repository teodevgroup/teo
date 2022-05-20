use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::object::Object;
use crate::core::stage::Stage;


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

    async fn call(&self, _stage: Stage, object: Object) -> Stage {
        let value = object.inner.value_map.borrow().get(self.key).unwrap().clone();
        return Stage::Value(value);
    }
}
