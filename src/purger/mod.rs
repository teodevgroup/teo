use crate::app::app_ctx::AppCtx;
use crate::core::result::Result;

pub(crate) async fn purge() -> Result<()> {
    AppCtx::get()?.connector()?.connection().await?.purge().await
}
