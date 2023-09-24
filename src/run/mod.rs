use crate::app::app_ctx::AppCtx;
use crate::core::result::Result;
use crate::prelude::UserCtx;

pub async fn run_program(name: &str) -> Result<()> {
    let app_ctx = AppCtx::get()?;
    if let Some(program) = app_ctx.program(name) {
        let user_ctx = UserCtx::new(app_ctx.connector()?.connection().await?, None);
        program.call(user_ctx).await?;
    }
    Ok(())
}