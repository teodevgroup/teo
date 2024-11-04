pub mod app;

mod tests {
    use std::cell::OnceCell;
    use teo::prelude::App;
    use std::file;
    use bytes::Bytes;
    use hyper::Method;
    use teo::test::schema_path::schema_path_args;
    use serde_json::{json, Value};
    use serial_test::serial;
    use teo::server::server::Server;
    use teo::server::test_request::TestRequest;
    use crate::{assert_json, matcher};
    use crate::runtime::response::app::load_app;

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
    async fn text_response() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::GET, "/textResponse");
        let res = server().process_test_request(req).await.unwrap();
        assert_eq!(res.body_as_string().as_str(), "foo");
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn json_response() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::GET, "/jsonResponse");
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "foo": "bar",
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn file_response() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::GET, "/fileResponse");
        let res = server().process_test_request(req).await.unwrap();
        let res_body = res.body();
        assert_eq!(res_body, &Bytes::from("foo".to_owned()));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn cookie_in_text_response() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::GET, "/textResponse");
        let res = server().process_test_request(req).await.unwrap();
        assert_eq!(res.headers().get("set-cookie").unwrap().to_str().unwrap(), "foo=bar");
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn cookie_in_json_response() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::GET, "/jsonResponse");
        let res = server().process_test_request(req).await.unwrap();
        assert_eq!(res.headers().get("set-cookie").unwrap().to_str().unwrap(), "foo=bar");
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn cookie_in_file_response() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::GET, "/fileResponse");
        let res = server().process_test_request(req).await.unwrap();
        assert_eq!(res.headers().get("set-cookie").unwrap().to_str().unwrap(), "foo=bar");
    }
}
