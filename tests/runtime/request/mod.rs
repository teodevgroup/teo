pub mod app;

#[cfg(test)]
mod tests {
    use std::cell::OnceCell;
    use teo::prelude::App;
    use std::file;
    use std::path::Path;
    use bytes::Bytes;
    use hyper::Method;
    use teo::test::schema_path::schema_path_args;
    use serde_json::{json, Value};
    use serial_test::serial;
    use teo::server::server::Server;
    use teo::server::test_request::TestRequest;
    use form_data_builder::FormData;
    use http_body_util::Full;
    use crate::{assert_json, matcher};
    use crate::lib::matcher_functions::string_ends_with;
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
        before_all().await;
        before_each().await;
        let path = Path::new(file!());
        let source = path.parent().unwrap().join("mai.jpg");
        let mut form = FormData::new(Vec::new());
        form.write_path("avatar", source, "image/jpg").unwrap();
        form.write_field("name", "Shiranui Mai").unwrap();
        let header_value = form.content_type_header();
        let body = form.finish().unwrap();
        let bytes = Bytes::from(body);
        let full = Full::new(bytes);
        let req = TestRequest::new(Method::PATCH, "/echo/formBody")
            .insert_header("content-type", header_value).unwrap().set_body(full).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "name": "Shiranui Mai",
            "avatar": string_ends_with(".jpg"),
        }))
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
