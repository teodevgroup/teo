use async_trait::async_trait;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct IsItem {
    value: Value,
    relation_name: Option<String>,
}

impl IsItem {
    pub fn new(value: Value, relation_name: Option<String>) -> Self {
        Self { value, relation_name }
    }
}

#[async_trait]
impl Item for IsItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        match ctx.value.as_ref().as_object() {
            None => {
                if ctx.value.as_ref().is_null() {
                    Err(ctx.with_invalid("value object is null"))
                } else {
                    Err(ctx.internal_server_error("is: value is not object"))
                }
            }
            Some(o) => {
                let rhs = self.value.resolve(ctx.clone()).await?;
                if !rhs.is_object() {
                    return Err(ctx.internal_server_error("is: argument 0 is not object"))
                }
                let rhs = rhs.as_object().unwrap();
                match &self.relation_name {
                    None => {
                        if o.model().name() != rhs.model().name() {
                            return Err(ctx.with_invalid("value object is not argument object"))
                        }
                        if o.identifier() != rhs.identifier() {
                            return Err(ctx.with_invalid("value object is not argument object"))
                        }
                    }
                    Some(relation_name) => {
                        let rhs_relation = rhs.model().relation(relation_name).unwrap();
                        if o.model().name() != rhs_relation.model() {
                            return Err(ctx.with_invalid("value object is not target object"))
                        }
                        for (f, r) in rhs_relation.iter() {
                            if rhs.get_value(f) != o.get_value(r) {
                                return Err(ctx.with_invalid("value object is not target object"))
                            }
                        }
                    }
                }
                Ok(ctx)
            }

        }
    }
}
