use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use teo_teon::value::Value;
use crate::core::result::Result;

#[derive(Debug, Clone)]
pub struct GetItem {
    key: Value
}

impl GetItem {
    pub fn new(key: Value) -> Self {
        Self { key }
    }
}

#[async_trait]
impl Item for GetItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let key = self.key.resolve(ctx.clone()).await?;
        match key {
            Value::Vec(keys) => {
                let mut new_ctx = ctx.clone();
                for key in keys.iter() {
                    new_ctx = GetItem::new(key.clone()).call(new_ctx).await?;
                }
                Ok(new_ctx)
            },
            Value::RawEnumChoice(e, _) => match ctx.value.as_object() {
                Some(object) => {
                    Ok(ctx.with_value(object.get_value(e.as_str()).unwrap()))
                }
                None => Err(ctx.internal_server_error("get: ctx value is not object"))
            }
            Value::I32(i) => match ctx.value.as_vec() {
                Some(vec) => {
                    match vec.get(i as usize) {
                        Some(val) => Ok(ctx.with_value(val.clone())),
                        None => Err(ctx.internal_server_error("get: index out of bound"))
                    }
                }
                None => Err(ctx.internal_server_error("get: ctx value is not array"))
            }
            Value::I64(i) => match ctx.value.as_vec() {
                Some(vec) => {
                    match vec.get(i as usize) {
                        Some(val) => Ok(ctx.with_value(val.clone())),
                        None => Err(ctx.internal_server_error("get: index out of bound"))
                    }
                }
                None => Err(ctx.internal_server_error("get: ctx value is not array"))
            }
            Value::String(s) => match &ctx.value {
                Value::Dictionary(hashmap) => {
                    match hashmap.get(&s) {
                        Some(val) => Ok(ctx.with_value(val.clone())),
                        None => {
                            Err(ctx.internal_server_error("get: value at key does not exist"))
                        }
                    }
                }
                Value::BTreeMap(btreemap) => {
                    match btreemap.get(&s) {
                        Some(val) => Ok(ctx.with_value(val.clone())),
                        None => Err(ctx.internal_server_error("get: value at key does not exist"))
                    }
                }
                _ => Err(ctx.internal_server_error("get: ctx value is not map"))
            }
            _ => Err(ctx.internal_server_error("get: incorrect key type"))
        }
    }
}
