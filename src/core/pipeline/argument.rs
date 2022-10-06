use std::fmt::{Debug};
use chrono::{Date, DateTime, Utc};
use crate::core::pipeline::argument::Argument::{PipelineArgument, ValueArgument};
use crate::core::pipeline::builder::PipelineBuilder;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;
use crate::core::tson::Value;

#[derive(Debug, Clone)]
pub enum Argument {
    ValueArgument(Value),
    PipelineArgument(Pipeline),
}

impl Argument {

    pub(crate) fn as_value(&self) -> Option<&Value> {
        match self {
            ValueArgument(v) => Some(v),
            PipelineArgument(_) => None
        }
    }

    pub(crate) fn as_pipeline(&self) -> Option<&Pipeline> {
        match self {
            ValueArgument(_) => None,
            PipelineArgument(p) => Some(p)
        }
    }

    pub(crate) async fn resolve(&self, context: Context<'_>) -> Value {
        match self {
            ValueArgument(v) => v.clone(),
            PipelineArgument(p) => p.process(context).await.value
        }
    }
}

impl From<&str> for Argument {
    fn from(v: &str) -> Self {
        ValueArgument(Value::String(v.to_string()))
    }
}

impl From<String> for Argument {
    fn from(v: String) -> Self {
        ValueArgument(Value::String(v))
    }
}

impl From<bool> for Argument {
    fn from(v: bool) -> Self {
        ValueArgument(Value::Bool(v))
    }
}

impl From<i8> for Argument {
    fn from(v: i8) -> Self {
        ValueArgument(Value::I8(v))
    }
}

impl From<i16> for Argument {
    fn from(v: i16) -> Self {
        ValueArgument(Value::I16(v))
    }
}

impl From<i32> for Argument {
    fn from(v: i32) -> Self {
        ValueArgument(Value::I32(v))
    }
}

impl From<i64> for Argument {
    fn from(v: i64) -> Self {
        ValueArgument(Value::I64(v))
    }
}

impl From<i128> for Argument {
    fn from(v: i128) -> Self {
        ValueArgument(Value::I128(v))
    }
}

impl From<u8> for Argument {
    fn from(v: u8) -> Self {
        ValueArgument(Value::U8(v))
    }
}

impl From<u16> for Argument {
    fn from(v: u16) -> Self {
        ValueArgument(Value::U16(v))
    }
}

impl From<u32> for Argument {
    fn from(v: u32) -> Self {
        ValueArgument(Value::U32(v))
    }
}

impl From<u64> for Argument {
    fn from(v: u64) -> Self {
        ValueArgument(Value::U64(v))
    }
}

impl From<u128> for Argument {
    fn from(v: u128) -> Self {
        ValueArgument(Value::U128(v))
    }
}

impl From<f32> for Argument {
    fn from(v: f32) -> Self {
        ValueArgument(Value::F32(v))
    }
}

impl From<f64> for Argument {
    fn from(v: f64) -> Self {
        ValueArgument(Value::F64(v))
    }
}

impl From<Date<Utc>> for Argument {
    fn from(v: Date<Utc>) -> Self {
        ValueArgument(Value::Date(v))
    }
}

impl From<DateTime<Utc>> for Argument {
    fn from(v: DateTime<Utc>) -> Self {
        ValueArgument(Value::DateTime(v))
    }
}

impl From<Value> for Argument {
    fn from(v: Value) -> Self {
        ValueArgument(v)
    }
}

impl<T> From<Vec<T>> for Argument where T: Into<Value> + Clone, Value: From<T> {
    fn from(vec: Vec<T>) -> Self {
        let result: Vec<Value> = vec.iter().map(|v| {
            Value::from(v.clone())
        }).collect::<Vec<Value>>();
        ValueArgument(Value::Vec(result))
    }
}

impl<F> From<F> for Argument where F: Fn(&mut PipelineBuilder) +  Clone + 'static {
    fn from(v: F) -> Self {
        let mut p = PipelineBuilder::new();
        v(&mut p);
        PipelineArgument(p.build())
    }
}
