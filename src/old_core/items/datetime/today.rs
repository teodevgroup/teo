use async_trait::async_trait;
use chrono::{Duration, Utc};
use crate::core::item::Item;
use teo_teon::value::Value;
use crate::core::result::Result;
use crate::core::pipeline::ctx::PipelineCtx;

#[derive(Debug, Copy, Clone)]
pub struct TodayItem {
    timezone: i32
}

impl TodayItem {
    pub fn new(timezone: i32) -> Self {
        return TodayItem {
            timezone
        };
    }
}

#[async_trait]
impl Item for TodayItem {

    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        let now = Utc::now();
        let calculated = now + Duration::hours(self.timezone as i64);
        Ok(ctx.with_value(Value::Date(calculated.date_naive())))
    }
}
