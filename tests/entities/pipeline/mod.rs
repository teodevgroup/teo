pub mod app;
mod entities;

#[cfg(test)]
mod tests {
    use std::cell::OnceCell;
    use teo::prelude::App;
    use std::file;
    use hyper::Method;
    use teo::server::{server::Server, test_request::TestRequest};
    use teo::test::schema_path::schema_path_args;
    use serde_json::{json, Value};
    use serial_test::serial;
    use crate::{assert_json, matcher};
    use crate::entities::pipeline::app::load_app;
    use crate::lib::matcher_functions::{date_time_value, decimal_value, date_value};

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
    async fn transform_int32() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "int32": 1,
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "int32": 10,
            }
        }))
    }
}
