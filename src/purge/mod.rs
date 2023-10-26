use teo_result::Result;
use crate::app::ctx::Ctx;

pub(crate) async fn purge() -> Result<()> {
    let ctx = Ctx::conn_ctx();
    for (namespace_path, connection) in ctx.connections_iter() {
        let namespace = ctx.namespace().namespace_at_path(&namespace_path.iter().map(AsRef::as_ref).collect()).unwrap();
        let transaction = connection.no_transaction().await?;
        transaction.purge(namespace.models_under_connector()).await?;
    }
    Ok(())
}
