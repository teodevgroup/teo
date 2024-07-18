use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_runtime::app::App;
use teo_runtime::app::cli::SeedCommandAction;
use teo_runtime::connection::transaction;
use teo_runtime::schema::load::load_data_sets::load_data_sets;
use teo::purge::purge;
use teo::result::Result;
use teo::seeder::seed::seed;

pub(crate) async fn purge_and_seed(app: &App) -> Result<()> {
    purge(app).await?;
    let mut diagnostics = Diagnostics::new();
    let data_sets = load_data_sets(app.main_namespace(), None, false, app.schema(), &mut diagnostics)?;
    let transaction_ctx = transaction::Ctx::new(app.conn_ctx().clone());
    seed(SeedCommandAction::Seed, data_sets, transaction_ctx, false).await?;
    Ok(())
}