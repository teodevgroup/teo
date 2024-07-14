use std::fs::File;
use std::io::Write;
use std::path::Path;
use teo_runtime::request::Request;
use teo_runtime::response::Response;
use teo_runtime::{request, teon, Value};
use teo::app::App;
use teo::result::Result;
use crate::lib::schema_path::schema_path_args;

pub fn load_app() -> Result<App> {
    let app = App::new_with_argv(
        schema_path_args(file!(), "schema.teo")
    )?;
    app.main_namespace().define_handler("textResponse", |req: Request| async move {
        Ok(Response::string("foo", "text/plain"))
    });
    app.main_namespace().define_handler("jsonResponse", |req: Request| async move {
        Ok(Response::teon(teon!({
            "foo": "bar"
        })))
    });
    app.main_namespace().define_handler("fileResponse", |req: Request| async move {
        let path = Path::new(file!());
        let source = path.parent().unwrap().join("response.txt");
        Ok(Response::file(source))
    });
    Ok(app)
}
