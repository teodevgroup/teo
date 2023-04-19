use async_trait::async_trait;
use crate::app::ctx::AppCtx;
use crate::core::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::result::Result;
use crate::prelude::Value;

#[derive(Debug, Clone)]
pub struct QueryRawItem {
    query: Value,
}

impl QueryRawItem {
    pub fn new(query: Value) -> Self {
        QueryRawItem { query }
    }
}

#[async_trait]
impl Item for QueryRawItem {
    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        let result = AppCtx::get()?.connector()?.query_raw(&self.query).await;
        match result {
            Err(err) => Err(err),
            Ok(val) => Ok(ctx.with_value(val)),
        }
    }
}
