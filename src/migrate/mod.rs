use teo_result::{Error, Result};
use crate::app::ctx::Ctx;

pub async fn migrate(dry_run: bool, reset: bool, silent: bool) -> Result<()> {
    let ctx = Ctx::conn_ctx();
    for (namespace_path, connection) in ctx.connections_iter() {
        let namespace = ctx.namespace().namespace_at_path(&namespace_path.iter().map(AsRef::as_ref).collect()).unwrap();
        let transaction = connection.no_transaction().await?;
        transaction.migrate(namespace.models_under_connector(), dry_run, reset, silent).await?;
    }
    Ok(())
}
