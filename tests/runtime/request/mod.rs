pub mod app;

#[cfg(test)]
mod tests {
    use std::cell::OnceCell;
    use actix_web::{http::header::ContentType, test};
    use teo::test::server::make_actix_app;
    use teo::prelude::App;
    use std::file;
    use actix_http::body::MessageBody;
    use actix_http::Method;
    use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
    use teo::test::schema_path::schema_path_args;
    use serde_json::{json, Value};
    use serial_test::serial;
    use test_helpers_async::*;
    use crate::{assert_json, matcher};
    use teo::test::handle::Handle;
    use crate::runtime::request::app::load_app;
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
    async fn path() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/")
            .set_json(json!({}))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res["path"], "/");
    }

    #[serial]
    #[actix_web::test]
    async fn query_string() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/?foo=bar")
            .set_json(json!({}))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res["queryString"], "foo=bar");
    }

    #[serial]
    #[actix_web::test]
    async fn content_type_from_header() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/?foo=bar")
            .insert_header(("content-type", "json"))
            .set_json(json!({}))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res["contentTypeFromHeader"], "application/json");
    }

    #[serial]
    #[actix_web::test]
    async fn content_type() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/?foo=bar")
            .insert_header(("content-type", "json"))
            .set_json(json!({}))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res["contentType"], "application/json");
    }

    #[serial]
    #[actix_web::test]
    async fn method() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/?foo=bar")
            .insert_header(("content-type", "json"))
            .set_json(json!({}))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res["method"], "POST");
    }

    #[serial]
    #[actix_web::test]
    async fn path_argument() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::GET)
            .uri("/echo/foo")
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res.as_ref(), "foo".as_bytes());
    }

    #[serial]
    #[actix_web::test]
    async fn path_combined_argument() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::GET)
            .uri("/echo/foo/bar/echo")
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res.as_ref(), "foo/bar".as_bytes());
    }

    #[serial]
    #[actix_web::test]
    async fn json_body() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::PATCH)
            .uri("/echo/jsonBody")
            .set_json(json!({
                "name": "foo",
                "age": 1,
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "name": "foo",
            "age": 1,
        }))
    }

    #[serial]
    #[actix_web::test]
    async fn form_body() {
        // TODO: complete this
//         let app = make_app().await;
//         let avatar = create_form_data_payload_and_headers("avatar", Some("a.jpg".to_owned()), None, Bytes::from_static(b"Lorem ipsum."));
//         let name = create_form_data_payload_and_headers("name", None, None, Bytes::from_static(b"foo"));
//         let req = test::TestRequest::default()
//             .method(Method::PATCH)
//             .uri("/echo/formBody")
//             .
// //            .set_payload(avatar.0 + name.0)
//             .to_request();
//         let res: Value = test::call_and_read_body_json(&app, req).await;
//         assert_json!(res, matcher!({
//             "name": "foo",
//             "age": 1,
//         }))
    }

    #[serial]
    #[actix_web::test]
    async fn cookie() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/echo/cookie")
            .cookie(Cookie::new("a", "b"))
            .set_json(json!({}))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "cookies": [
                { "name": "a", "value": "b" }
            ]
        }))
    }
}
