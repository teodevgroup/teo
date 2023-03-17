use crate::core::pipeline::ctx::Ctx;
use crate::core::pipeline::item::Item;
use crate::core::result::Result;
use crate::core::teon::Value;
use async_trait::async_trait;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct RandomFloatItem {
    argument: Value,
}

impl RandomFloatItem {
    pub fn new(argument: Value) -> Self {
        Self {
            argument: argument.into(),
        }
    }
}

#[async_trait]
impl Item for RandomFloatItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await?;
        if argument.is_range() {
            let r = self.argument.as_range().unwrap();
            let start = r.start.resolve(ctx.clone()).await?.as_f64().unwrap();
            let end = r.end.resolve(ctx.clone()).await?.as_f64().unwrap();
            let mut rng = thread_rng();
            let random_number = rng.gen_range(start..end);
            Ok(ctx.with_value(Value::F64(random_number)))
        } else {
            Err(ctx.with_invalid(format!("Value must be range")))
        }
    }
}
