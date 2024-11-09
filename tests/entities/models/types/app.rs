use teo_runtime::request::Request;
use teo_runtime::response::Response;
use teo_runtime::{request, teon, Value};
use teo::app::App;
use teo::result::Result;
use teo::test::schema_path::schema_path_args;

pub fn load_app() -> Result<App> {
    let app = App::new_with_argv(
        schema_path_args(file!(), "schema.teo")
    )?;
    Ok(app)
}