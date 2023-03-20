use crate::core::pipeline::ctx::Ctx;
use crate::core::pipeline::item::Item;
use crate::core::result::Result;
use crate::core::teon::Value;
use async_trait::async_trait;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct RandomIntItem {
    argument: Value,
}

impl RandomIntItem {
    pub fn new(argument: Value) -> Self {
        Self {
            argument: argument.into(),
        }
    }
}

#[async_trait]
impl Item for RandomIntItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await?;
        let  res = if argument.is_number() {
            let length = argument.as_i32().unwrap() as u32;
            if length > 0 && length < 10 {
                Ok((10_i32.pow(length - 1), 10_i32.pow(length), false))
            } else if length == 10 {
                Ok((10_i32.pow(9), 2147483647, true))
            } else {
                Err("When the parameter is a number, its range must be between 1 and 10.")
            }
        } else if argument.is_range() {
            let r = self.argument.as_range().unwrap();
            let start = r.start.resolve(ctx.clone()).await?.as_i32().unwrap();
            let end = r.end.resolve(ctx.clone()).await?.as_i32().unwrap();
            Ok((start, end, r.closed))
        } else {
            unreachable!()
        };
        match res {
           Ok((start, end, closed)) => {
                let mut rng = thread_rng();
                let mut random_number;
                if closed {
                    random_number = rng.gen_range(start..=end);
                } else {
                    random_number = rng.gen_range(start..end);
                }
                Ok(ctx.with_value(Value::I32(random_number)))
            }
            Err(e) => Err(ctx.internal_server_error(e)),
        }
    }
}
