use std::future::Future;
use std::sync::Arc;
use crate::app::ctx::AppCtx;
use crate::core::connector::connection::Connection;
use crate::core::ctx::model::ModelCtx;
use crate::core::result::Result;

pub struct UserCtx {
    conn: Arc<dyn Connection>,
}

impl UserCtx {

    pub(crate) fn new(conn: Arc<dyn Connection>) -> Self {
        Self { conn }
    }

    pub fn model_ctx(&self, name: &str) -> Result<ModelCtx> {
        let model = AppCtx::get()?.graph()?.model(name)?;
        Ok(ModelCtx {
            conn: self.conn.clone(),
            model,
        })
    }

    pub async fn transaction<F, Fut, C>(&self, f: F) -> Result<()> where F: Fn(C) -> Fut, C: From<UserCtx>, Fut: Future<Output = Result<()>> {
        let conn_with_transaction = self.conn.transaction().await?;
        let tran_ctx = UserCtx {
            conn: conn_with_transaction.clone()
        };
        f(tran_ctx.into()).await?;
        conn_with_transaction.commit().await?;
        Ok(())
    }
}
