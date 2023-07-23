use crate::app::app_ctx::AppCtx;
use crate::prelude::Graph;
use crate::core::result::Result;

pub(crate) async fn purge(graph: &Graph) -> Result<()> {
    AppCtx::get()?.connector()?.connection().await?.purge(graph).await
}
