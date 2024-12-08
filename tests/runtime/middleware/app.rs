use teo_runtime::request::Request;
use teo_runtime::response::Response;
use teo_runtime::{request, teon, Value};
use teo_runtime::arguments::Arguments;
use teo_runtime::middleware::next::{Next, NextImp};
use teo::app::App;
use teo::result::Result;
use teo::test::schema_path::schema_path_args;

struct NumberContainer {
    number: i32
}

pub fn load_app() -> Result<App> {
    let app = App::new_with_argv(
        schema_path_args(file!(), "schema.teo")
    )?;
    app.main_namespace().define_handler("inspect", |req: Request| async move {
        let number_from_values: i32 = req.local_values().get("number")?;
        let number_from_objects: &NumberContainer = req.local_objects().get("number").unwrap();
        Ok(Response::teon(teon!({
            "numberFromValues": number_from_values,
            "numberFromObjects": number_from_objects.number,
        })))
    });
    app.main_namespace().define_request_middleware("requestOuter", |arguments: Arguments| {
        Ok(|req: Request, next: Next| async move {
            req.local_values().insert("number", 42);
            Ok(next.call(req).await?)
        })
    });
    app.main_namespace().define_request_middleware("requestMiddle", |arguments: Arguments| {
        Ok(|req: Request, next: Next| async move {
            let number: Option<i32> = req.local_values().get("number")?;
            req.local_values().insert("number", number.unwrap() * 2);
            Ok(next.call(req).await?)
        })
    });
    app.main_namespace().define_request_middleware("requestInner", |arguments: Arguments| {
        Ok(|req: Request, next: Next| async move {
            let number: Option<i32> = req.local_values().get("number")?;
            req.local_values().insert("number", number.unwrap() + 16);
            Ok(next.call(req).await?)
        })
    });
    app.main_namespace().define_handler_middleware("handlerOuter", |arguments: Arguments| {
        Ok(|req: Request, next: Next| async move {
            req.local_objects().insert("number", NumberContainer { number: 24 });
            Ok(next.call(req).await?)
        })
    });
    app.main_namespace().define_handler_middleware("handlerMiddle", |arguments: Arguments| {
        Ok(|req: Request, next: Next| async move {
            let number: &mut NumberContainer = req.local_objects().get_mut("number").unwrap();
            number.number *= 4;
            Ok(next.call(req).await?)
        })
    });
    app.main_namespace().define_handler_middleware("handlerInner", |arguments: Arguments| {
        Ok(|req: Request, next: Next| async move {
            let number: &mut NumberContainer = req.local_objects().get_mut("number").unwrap();
            number.number += 4;
            Ok(next.call(req).await?)
        })
    });
    Ok(app)
}