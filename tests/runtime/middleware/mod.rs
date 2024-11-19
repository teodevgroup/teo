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
    use super::app::load_app;

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
    async fn middleware_and_request_locals() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/").json_body(json!({})).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "numberFromValues": 100,
            "numberFromObjects": 100,
        }));
    }
}
