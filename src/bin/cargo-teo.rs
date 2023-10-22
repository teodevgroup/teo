use tokio::main;
use teo_result::Result;
use teo::app::App;
use teo::app::ctx::Ctx;
use teo::cli::entrance::Entrance;

#[main]
async fn main() -> Result<()> {
    let app = App::new()?;
    Ctx::set_entrance(Entrance::CLI);
    app.run().await
}
