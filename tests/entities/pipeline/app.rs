use futures_util::future::try_join_all;
use teo_runtime::request::Request;
use teo_runtime::response::Response;
use teo_runtime::{pipeline, request, teon, Value};
use teo_runtime::arguments::Arguments;
use teo_runtime::pipeline::item::templates::validator::{ValidatorResult, Validity};
use teo::app::App;
use teo::result::Result;
use teo::test::schema_path::schema_path_args;
use crate::entities::models::types::entities::{SupportCreateInput, SupportFindManyArgs, Teo};

pub fn load_app() -> Result<App> {
    let app = App::new_with_argv(
        schema_path_args(file!(), "schema.teo")
    )?;
    app.main_namespace().define_validator_pipeline_item("myValidator", |args: Arguments| {
        return Ok(|_: Value, ctx: pipeline::Ctx| async move {
            ValidatorResult::Validity(Validity::Valid)
        })
    });
    Ok(app)
}