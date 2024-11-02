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
    app.main_namespace().define_handler("inspect", |req: Request| async move {
        let content_type = req.headers().get("content-type").unwrap().to_str().unwrap();
        Ok(Response::teon(teon!({
            "path": req.path(),
            "queryString": req.query().map(|s| s.to_string()),
            "contentTypeFromHeader": content_type,
            "contentType": req.content_type().unwrap().map(|s| s.to_string()),
            "method": req.method().to_string(),
        })))
    });
    app.main_namespace().define_handler("echo", |req: Request| async move {
        let binding = req.handler_match().unwrap();
        let captures = binding.captures();
        let echo = captures.get("data").unwrap();
        Ok(Response::string(echo, "text/plain"))
    });
    app.main_namespace().define_handler("echoMore", |req: Request| async move {
        let binding = req.handler_match().unwrap();
        let captures = binding.captures();
        let echo = captures.get("data").unwrap();
        Ok(Response::string(echo, "text/plain"))
    });
    app.main_namespace().define_handler("echoJsonBody", |req: Request| async move {
        Ok(Response::teon(req.body_value().as_ref().clone()))
    });
    app.main_namespace().define_handler("echoFormBody", |req: Request| async move {
        let filepath = req.body_value().get("avatar").unwrap().as_file().unwrap().filepath.clone();
        Ok(Response::teon(teon!({
            "name": req.body_value().get("name").unwrap(),
            "avatar": filepath
        })))
    });
    app.main_namespace().define_handler("echoCookie", |req: Request| async move {
        let cookies = req.cookies()?;
        let mut result: Vec<Value> = vec![];
        for cookie in cookies.iter() {
            let value = teon!({
                "name": cookie.name(),
                "value": cookie.value(),
            });
            result.push(value);
        }
        Ok(Response::teon(teon!({
            "cookies": result
        })))
    });
    Ok(app)
}