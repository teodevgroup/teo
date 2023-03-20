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
        let (start, end, closed) = if argument.is_range() {
            let r = self.argument.as_range().unwrap();
            let start = r.start.resolve(ctx.clone()).await?.as_f64().unwrap();
            let end = r.end.resolve(ctx.clone()).await?.as_f64().unwrap();
            (start, end, r.closed)
        } else {
            unreachable!()
        };
        let mut rng = thread_rng();
        let mut random_number;
        if closed {
            random_number = rng.gen_range(start..=end);
        }else {
            random_number = rng.gen_range(start..end);
        }
        Ok(ctx.with_value(Value::F64(random_number)))
    }
}
