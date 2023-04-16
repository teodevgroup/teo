use async_trait::async_trait;
use crate::core::teon::Value;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::items::object::get::GetItem;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct SetItem {
    key: Option<Value>,
    value: Value,
}

impl SetItem {
    pub fn new(key: Option<Value>, value: Value) -> Self {
        Self {
            key,
            value,
        }
    }
}

#[async_trait]
impl Item for SetItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match self.key.as_ref() {
            None => Ok(ctx.with_value(self.value.resolve(ctx.clone()).await?)),
            Some(key) => {
                let key = key.resolve(ctx.clone()).await?;
                match key {
                    Value::Vec(keys) => {
                        if keys.len() == 0 {
                            Err(ctx.internal_server_error("set: empty key path received"))
                        } else if keys.len() == 1 {
                            Ok(SetItem::new(Some(keys.get(0).unwrap().clone()), self.value.clone()).call(ctx).await?)
                        } else {
                            let current_key = keys.get(0).unwrap();
                            let current_value = GetItem::new(current_key.clone()).call(ctx.clone()).await?.get_value();
                            Ok(SetItem::new(Some(current_key.clone()), {
                                let mut rest_keys = keys.clone();
                                rest_keys.remove(0);
                                SetItem::new(Some(Value::Vec(rest_keys)), self.value.clone()).call(ctx.with_value(current_value)).await?.get_value()
                            }).call(ctx).await?)
                        }
                    },
                    Value::RawEnumChoice(e, _) => match ctx.value.as_object() {
                        Some(object) => {
                            let value = self.value.resolve(ctx.clone()).await?;
                            object.set_value(e.as_str(), value).unwrap();
                            Ok(ctx.with_value(Value::Object(object.clone())))
                        }
                        None => Err(ctx.internal_server_error("set: ctx value is not object"))
                    }
                    Value::I32(i) => match ctx.value.as_vec() {
                        Some(vec) => {
                            let value = self.value.resolve(ctx.clone()).await?;
                            let mut new_vec = vec.clone();
                            new_vec[i as usize] = value;
                            Ok(ctx.with_value(Value::Vec(new_vec)))
                        }
                        None => Err(ctx.internal_server_error("set: ctx value is not array"))
                    }
                    Value::I64(i) => match ctx.value.as_vec() {
                        Some(vec) => {
                            let value = self.value.resolve(ctx.clone()).await?;
                            let mut new_vec = vec.clone();
                            new_vec[i as usize] = value;
                            Ok(ctx.with_value(Value::Vec(new_vec)))
                        }
                        None => Err(ctx.internal_server_error("set: ctx value is not array"))
                    }
                    Value::String(s) => match &ctx.value {
                        Value::HashMap(hashmap) => {
                            let value = self.value.resolve(ctx.clone()).await?;
                            let mut new_map = hashmap.clone();
                            new_map.insert(s.to_owned(), value);
                            Ok(ctx.with_value(Value::HashMap(new_map)))
                        }
                        Value::BTreeMap(btreemap) => {
                            let value = self.value.resolve(ctx.clone()).await?;
                            let mut new_map = btreemap.clone();
                            new_map.insert(s.to_owned(), value);
                            Ok(ctx.with_value(Value::BTreeMap(new_map)))
                        }
                        _ => Err(ctx.internal_server_error("set: ctx value is not map"))
                    }
                    _ => Err(ctx.internal_server_error("set: incorrect key type"))
                }
            }
        }
    }
}
