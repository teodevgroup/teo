use crate::app::ctx::AppCtx;
use crate::prelude::{Graph};

pub(crate) async fn migrate(graph: &Graph, _dry_run: bool) {
    let app_ctx = AppCtx::get_mut()?;
    let result = app_ctx.connector_mut()?.migrate(graph.models(), false).await;
    if result.is_err() {
        panic!("Migration error");
    }
}
