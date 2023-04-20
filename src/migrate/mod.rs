use crate::app::ctx::AppCtx;
use crate::prelude::{Graph};
use crate::core::result::Result;

pub(crate) async fn migrate(graph: &Graph, _dry_run: bool) -> Result<()> {
    let app_ctx = AppCtx::get()?;
    let result = app_ctx.connector()?.migrate(graph.models(), false).await;
    if result.is_err() {
        panic!("Migration error");
    }
    Ok(())
}
