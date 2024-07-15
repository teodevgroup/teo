pub mod app;

mod tests {
    use std::cell::OnceCell;
    use actix_web::{http::header::ContentType, test};
    use crate::lib::server::make_actix_app;
    use teo::prelude::App;
    use std::file;
    use actix_http::body::MessageBody;
    use actix_http::Method;
    use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
    use crate::lib::schema_path::schema_path_args;
    use serde_json::{json, Value};
    use serial_test::serial;
    use test_helpers_async::*;
    use crate::{assert_json, matcher};
    use crate::lib::handle::Handle;
    use crate::runtime::response::app::load_app;
    use actix_multipart::test::create_form_data_payload_and_headers;
    use actix_web::cookie::Cookie;
    use actix_web::web::Bytes;

    static mut HANDLE: OnceCell<Handle> = OnceCell::new();

    async fn make_app() -> impl Service<
        actix_http::Request,
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
    > {
        unsafe {
            let teo_app = HANDLE.get_or_init(|| {
                let mut h = Handle::new();
                h.load(|| {
                    load_app().unwrap()
                });
                h
            }).teo_app();
            test::init_service(
                make_actix_app(
                    &teo_app
                ).await.unwrap()
            ).await
        }
    }

    #[serial]
    #[actix_web::test]
    async fn text_response() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::GET)
            .uri("/textResponse")
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, "foo");
    }

    #[serial]
    #[actix_web::test]
    async fn json_response() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::GET)
            .uri("/jsonResponse")
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "foo": "bar",
        }))
    }

    #[serial]
    #[actix_web::test]
    async fn file_response() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::GET)
            .uri("/fileResponse")
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, "foo");
    }

    #[serial]
    #[actix_web::test]
    async fn cookie_in_text_response() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::GET)
            .uri("/textResponse")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.headers().get("set-cookie").unwrap().to_str().unwrap(), "foo=bar");
    }

    #[serial]
    #[actix_web::test]
    async fn cookie_in_json_response() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::GET)
            .uri("/jsonResponse")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.headers().get("set-cookie").unwrap().to_str().unwrap(), "foo=bar");
    }

    #[serial]
    #[actix_web::test]
    async fn cookie_in_file_response() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::GET)
            .uri("/fileResponse")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.headers().get("set-cookie").unwrap().to_str().unwrap(), "foo=bar");
    }
}
