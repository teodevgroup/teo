use tokio::main;
use teo::app::entrance::Entrance;
use teo::core::result::Result;
use teo::app::app::App;
use teo::app::app_ctx::AppCtx;

#[main]
async fn main() -> Result<()> {
    let app = App::new()?;
    AppCtx::get()?.set_entrance(Entrance::CLI)?;
    app.run().await
}
