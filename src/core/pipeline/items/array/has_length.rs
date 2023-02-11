use async_trait::async_trait;
use crate::core::pipeline::ctx::Ctx;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct HasLengthModifier {
    argument: Value
}

impl HasLengthModifier {
    pub fn new(argument: Value) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Item for HasLengthModifier {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let argument = self.argument.resolve(ctx.clone()).await?;
        let (lower, upper, closed) = if argument.is_number() {
            let n = self.argument.as_usize().unwrap();
            (n, n, true)
        } else if argument.is_range() {
            let r = self.argument.as_range().unwrap();
            let start = r.start.resolve(ctx.clone()).await.as_usize().unwrap();
            let end = r.end.resolve(ctx.clone()).await.as_usize().unwrap();
            (start, end, r.closed)
        } else {
            panic!()
        };
        let len = match &ctx.value {
            Value::String(s) => s.len(),
            Value::Vec(v) => v.len(),
            _ => {
                return ctx.invalid("Value doesn't have length.");
            }
        };
        if len < lower {
            return ctx.invalid(format!("Value length is less than {lower}."));
        }
        if closed {
            if len > upper {
                Err(ctx.invalid(format!("Value length is greater than {upper}.")))
            } else {
                Ok(ctx.clone())
            }
        } else {
            if len >= upper {
                Err(ctx.invalid(format!("Value length is greater than or equal to {upper}.")))
            } else {
                Ok(ctx.clone())
            }
        }
    }
}
