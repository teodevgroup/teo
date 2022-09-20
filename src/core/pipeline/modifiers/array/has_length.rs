use async_trait::async_trait;
use std::ops::Range;
use crate::core::pipeline::argument::Argument;

use crate::core::pipeline::context::Context;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;

#[derive(Debug, Clone)]
pub struct LengthArgument {
    lower: Argument,
    upper: Argument,
    closed: bool,
}

impl<T> Into<LengthArgument> for Range<T> where T: Into<Value> {
    fn into(self) -> LengthArgument {
        let start_value: Value = self.start.into();
        let end_value: Value = self.end.into();
        LengthArgument {
            lower: Argument::ValueArgument(start_value),
            upper: Argument::ValueArgument(end_value),
            closed: false,
        }
    }
}

impl<T> From<T> for LengthArgument where T: Into<Argument> {
    fn from(arg: T) -> Self {
        let value: Argument = arg.into();
        LengthArgument {
            lower: value.clone(),
            upper: value.clone(),
            closed: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HasLengthModifier {
    argument: LengthArgument
}

impl HasLengthModifier {
    pub fn new(argument: impl Into<LengthArgument>) -> Self {
        Self { argument: argument.into() }
    }
}

#[async_trait]
impl Modifier for HasLengthModifier {

    fn name(&self) -> &'static str {
        "hasLength"
    }

    async fn call(&self, context: Context) -> Context {
        let (lower, upper) = match &self.argument.lower {
            Argument::ValueArgument(l) => {
                (l.as_usize().unwrap(), self.argument.upper.as_value().unwrap().as_usize().unwrap())
            }
            Argument::PipelineArgument(p) => {
                match p.process(context.clone()).await.value.as_vec() {
                    Some(v) => {
                        if v.len() == 2 {
                            (v[0].as_usize().unwrap(), v[1].as_usize().unwrap())
                        } else {
                            return context.invalid("Pipeline argument does not resolve into a 2 length vector.");
                        }
                    }
                    None => {
                        return context.invalid("Pipeline argument does not resolve into a vector.");
                    }
                }
            }
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
        if self.argument.closed {
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
