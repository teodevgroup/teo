use crate::app::ctx::AppCtx;
use crate::prelude::Graph;
use crate::core::result::Result;

pub(crate) async fn purge(graph: &Graph) -> Result<()> {
    AppCtx::get()?.connector()?.purge(graph).await
}
