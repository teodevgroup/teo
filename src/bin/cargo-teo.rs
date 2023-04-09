use tokio::main;
use teo::core::app::entrance::Entrance;
use teo::prelude::AppBuilder;

#[main]
async fn main() -> std::io::Result<()> {
    let app_builder = AppBuilder::new_with_entrance(Entrance::CLI);
    let app = app_builder.build().await;
    app.run().await
}
