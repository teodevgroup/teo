use teo_result::Result;
use crate::app::App;

pub async fn purge(app: &App) -> Result<()> {
    let ctx = app.conn_ctx();
    for (namespace_path, connection) in ctx.connections_iter() {
        let namespace = ctx.namespace().namespace_at_path(namespace_path).unwrap();
        let transaction = connection.no_transaction().await?;
        transaction.purge(namespace.models_under_connector()).await?;
    }
    Ok(())
}
