use tokio::main;
use teo_result::Result;
use teo_runtime::app::App;
use teo_runtime::app::entrance::Entrance;
use teo::AppExt;

#[main]
async fn main() -> Result<()> {
    let app = App::new_with_entrance_and_runtime_version(Some(Entrance::CLI), None, None)?;
    app.run().await
}
