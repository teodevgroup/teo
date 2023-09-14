use async_trait::async_trait;
use chrono::{Duration, Utc};
use crate::core::item::Item;
use crate::core::teon::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::PipelineCtx;

#[derive(Debug, Copy, Clone)]
pub struct ToDateItem {
    timezone: i32
}

impl ToDateItem {
    pub fn new(timezone: i32) -> Self {
        return ToDateItem {
            timezone
        };
    }
}

#[async_trait]
impl Item for ToDateItem {

    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let datetime = ctx.value.as_datetime().unwrap();
        let calculated = datetime + Duration::hours(self.timezone as i64);
        Ok(ctx.with_value(Value::Date(calculated.date_naive())))
    }
}
