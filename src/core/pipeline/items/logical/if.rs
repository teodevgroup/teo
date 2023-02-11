use async_trait::async_trait;
use crate::core::pipeline::item::Item;

use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct IfModifier {
    cond: Value,
    then: Option<Value>,
    r#else: Option<Value>,
}

impl IfModifier {
    pub fn new(cond: Value, then: Option<Value>, r#else: Option<Value>) -> Self {
        return IfModifier {
            cond, then, r#else,
        };
    }
}

#[async_trait]
impl Item for IfModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Ctx<'a> {
        let mut valid = false;
        match &self.cond {
            Value::Null => valid = false,
            Value::Bool(b) => valid = *b,
            Value::Pipeline(p) => valid = p.process(ctx.clone()).await.is_valid(),
            _ => {
                panic!("Only null, bool, pipeline can be passed to `if`.")
            }
        }
        return if valid {
            if let Some(then) = &self.then {
                match then {
                    Value::Pipeline(p) => p.process(ctx.clone()).await,
                    _ => ctx.with_value(then.clone())
                }
            } else {
                ctx.clone()
            }
        } else {
            if let Some(r#else) = &self.r#else {
                match r#else {
                    Value::Pipeline(p) => p.process(ctx.clone()).await,
                    _ => ctx.with_value(r#else.clone())
                }
            } else {
                ctx.clone()
            }
        }
    }
}
