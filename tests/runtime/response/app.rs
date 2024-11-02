use std::fs::File;
use std::io::Write;
use std::path::Path;
use teo_runtime::request::Request;
use teo_runtime::response::Response;
use teo_runtime::{request, teon, Value};
use teo::app::App;
use teo::result::Result;
use teo::test::schema_path::schema_path_args;
use cookie::Cookie;

pub fn load_app() -> Result<App> {
    let app = App::new_with_argv(
        schema_path_args(file!(), "schema.teo")
    )?;
    app.main_namespace().define_handler("textResponse", |req: Request| async move {
        let response = Response::string("foo", "text/plain");
        response.add_cookie(Cookie::new("foo", "bar"));
        Ok(response)
    });
    app.main_namespace().define_handler("jsonResponse", |req: Request| async move {
        let response = Response::teon(teon!({
            "foo": "bar"
        }));
        response.add_cookie(Cookie::new("foo", "bar"));
        Ok(response)
    });
    app.main_namespace().define_handler("fileResponse", |req: Request| async move {
        let path = Path::new(file!());
        let source = path.parent().unwrap().join("response.txt");
        let response = Response::file(source);
        response.add_cookie(Cookie::new("foo", "bar"));
        Ok(response)
    });
    Ok(app)
}
