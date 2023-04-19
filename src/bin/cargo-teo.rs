use tokio::main;
use teo::app::entrance::Entrance;
use teo::prelude::AppBuilder;

#[main]
async fn main() -> Result<()> {
    let app_builder = AppBuilder::new_with_entrance(Entrance::CLI);
    let app = app_builder.build().await;
    app.run().await
}
