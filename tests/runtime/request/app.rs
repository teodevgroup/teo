use teo_runtime::request::Request;
use teo_runtime::response::Response;
use teo_runtime::{request, teon};
use teo::app::App;
use teo::result::Result;
use crate::lib::schema_path::schema_path_args;

pub fn load_app() -> Result<App> {
    let app = App::new_with_argv(
        schema_path_args(file!(), "schema.teo")
    )?;
    app.namespace_builder().define_handler("inspect", |req: Request| async move {
        let content_type = req.headers().get("content-type").unwrap().to_str().unwrap();
        Ok(Response::teon(teon!({
            "path": req.path(),
            "queryString": req.query_string(),
            "contentTypeFromHeader": content_type,
            "contentType": req.content_type(),
            "method": req.method(),
        })))
    });
    app.namespace_builder().define_handler("echo", |ctx: request::Ctx| async move {
        let captures = ctx.handler_match().captures();
        let echo = captures.get("data").unwrap();
        Ok(Response::string(echo, "text/plain"))
    });
    app.namespace_builder().define_handler("echoMore", |ctx: request::Ctx| async move {
        let captures = ctx.handler_match().captures();
        let echo = captures.get("data").unwrap();
        Ok(Response::string(echo, "text/plain"))
    });
    Ok(app)
}