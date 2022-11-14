use async_trait::async_trait;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::modifier::Modifier;
use crate::core::tson::Value;

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
impl Modifier for HasLengthModifier {

    fn name(&self) -> &'static str {
        "hasLength"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        let argument = self.argument.resolve(context.clone()).await;
        let (lower, upper, closed) = if argument.is_number() {
            let n = self.argument.as_usize().unwrap();
            (n, n, true)
        } else if argument.is_range() {
            let r = self.argument.as_range().unwrap();
            let start = r.start.resolve(context.clone()).await.as_usize().unwrap();
            let end = r.end.resolve(context.clone()).await.as_usize().unwrap();
            (start, end, r.closed)
        } else {
            panic!()
        };
        let len = match &context.value {
            Value::String(s) => s.len(),
            Value::Vec(v) => v.len(),
            _ => {
                return context.invalid("Value doesn't have length.");
            }
        };
        if len < lower {
            return context.invalid(format!("Value length is less than {lower}."));
        }
        if closed {
            if len > upper {
                context.invalid(format!("Value length is greater than {upper}."))
            } else {
                context.clone()
            }
        } else {
            if len >= upper {
                context.invalid(format!("Value length is greater than or equal to {upper}."))
            } else {
                context.clone()
            }
        }
    }
}
