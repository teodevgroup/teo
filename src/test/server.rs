use actix_http::body::MessageBody;
use actix_web::App;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_runtime::connection::transaction;
use teo_runtime::schema::load::load_data_sets::load_data_sets;
use crate::cli::command::SeedCommandAction;
use crate::database::connect_databases;
use crate::migrate::migrate;
use crate::purge::purge;
use crate::result::Result;
use crate::seeder::seed::seed;
use crate::server::make::make_server_app;

pub async fn prepare_app_for_test(app: &crate::prelude::App) -> Result<()> {
    app.prepare_for_run().await?;
    connect_databases(app, app.compiled_main_namespace(), true).await?;
    migrate(app, false, false, true).await?;
    if app.compiled_main_namespace().database().is_some() {
        let mut diagnostics = Diagnostics::new();
        let data_sets = load_data_sets(app.main_namespace(), None, false, app.schema(), &mut diagnostics)?;
        let transaction_ctx = transaction::Ctx::new(app.conn_ctx().clone());
        purge(app).await?;
        seed(SeedCommandAction::Seed, data_sets, transaction_ctx, false).await?;
    }
    // setup
    if let Some(setup) = app.get_setup() {
        let transaction_ctx = transaction::Ctx::new(app.conn_ctx().clone());
        setup.call(transaction_ctx).await?;
    }
    Ok(())
}

pub async fn make_actix_app_without_prepare(app: &crate::prelude::App) -> Result<App<impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<impl MessageBody>,
    Config = (),
    InitError = (),
    Error = actix_web::Error,
> + 'static>> {
    let conn_ctx = app.conn_ctx();
    let namespace = conn_ctx.namespace();
    let server_conf = conn_ctx.namespace().server().unwrap();
    Ok(make_server_app(namespace, server_conf))
}

pub async fn make_actix_app(app: &crate::prelude::App) -> Result<App<impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<impl MessageBody>,
    Config = (),
    InitError = (),
    Error = actix_web::Error,
> + 'static>> {
    prepare_app_for_test(app).await?;
    make_actix_app_without_prepare(app).await
}
