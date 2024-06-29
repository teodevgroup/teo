use teo_result::{Error, Result};
use crate::app::App;

pub async fn migrate(app: &App, dry_run: bool, reset: bool, silent: bool) -> Result<()> {
    let ctx = app.conn_ctx();
    for (namespace_path, connection) in ctx.connections_iter() {
        let namespace = ctx.namespace().namespace_at_path(namespace_path).unwrap();
        let transaction = connection.no_transaction().await?;
        transaction.migrate(namespace.models_under_connector(), dry_run, reset, silent).await?;
    }
    Ok(())
}
