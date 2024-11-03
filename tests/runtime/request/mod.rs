pub mod app;

#[cfg(test)]
mod tests {
    use std::cell::OnceCell;
    use teo::prelude::App;
    use std::file;
    use hyper::Method;
    use teo::test::schema_path::schema_path_args;
    use serde_json::{json, Value};
    use serial_test::serial;
    use teo::server::server::Server;
    use teo::server::test_request::TestRequest;
    use crate::{assert_json, matcher};
    use crate::runtime::request::app::load_app;

    static mut SERVER: OnceCell<Server> = OnceCell::new();
    static mut BEFORE_ALL_EXECUTED: bool = false;

    fn server() -> &'static Server {
        unsafe { SERVER.get().unwrap() }
    }

    async fn before_all() {
        if unsafe { BEFORE_ALL_EXECUTED } {
            return;
        }
        unsafe {
            SERVER.get_or_init(|| {
                Server::new(load_app().unwrap())
            })
        };
        server().setup_app_for_unit_test().await.unwrap();
        unsafe { BEFORE_ALL_EXECUTED = true; }
    }

    async fn before_each() {
        server().reset_app_for_unit_test().await.unwrap();
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn path() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/").json_body(json!({})).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_eq!(res["path"], "/");
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn query() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/?foo=bar").json_body(json!({})).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_eq!(res["query"], "foo=bar");
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn content_type_from_header() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/?foo=bar")
            .insert_header("content-type", "json").unwrap()
            .json_body(json!({}))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_eq!(res["contentTypeFromHeader"], "application/json");
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn content_type() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/?foo=bar")
            .insert_header("content-type", "json").unwrap()
            .json_body(json!({}))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_eq!(res["contentType"], "application/json");
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn method() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/?foo=bar")
            .insert_header("content-type", "json").unwrap()
            .json_body(json!({}))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_eq!(res["method"], "POST");
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn captures() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::GET, "/echo/foo");
        let res = server().process_test_request(req).await.unwrap().body_as_string();
        assert_eq!(&res, "foo");
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn combined_captures() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::GET, "/echo/foo/bar/echo");
        let res = server().process_test_request(req).await.unwrap().body_as_string();
        assert_eq!(&res, "foo/bar");
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn json_body() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::PATCH, "/echo/jsonBody").json_body(json!({
            "name": "foo",
            "age": 1,
        })).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "name": "foo",
            "age": 1,
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
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
    #[shared_tokio_runtime::runtime_test]
    async fn cookie() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/echo/cookie")
            .append_header("Cookie", "a=b").unwrap()
            .json_body(json!({})).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "cookies": [
                { "name": "a", "value": "b" }
            ]
        }))
    }
}
