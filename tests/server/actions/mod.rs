#[cfg(test)]
mod tests {
    use std::cell::OnceCell;
    use teo::prelude::App;
    use std::file;
    use std::sync::Once;
    use hyper::header::CONTENT_TYPE;
    use hyper::Method;
    use mime::TEXT_PLAIN;
    use teo::test::schema_path::schema_path_args;
    use serde_json::{json, Value};
    use serial_test::serial;
    use teo::server::server::Server;
    use teo::server::test_request::TestRequest;
    use crate::{assert_json, matcher};

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
                Server::new(App::new_with_argv(
                    schema_path_args(file!(), "schema.teo")
                ).unwrap())
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
    async fn test_get_index() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::GET, "/")
            .insert_header(CONTENT_TYPE, TEXT_PLAIN.as_ref()).unwrap();
        let res = server().process_test_request(req).await.unwrap();
        assert_eq!(res.status().as_u16(), 404);
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn test_create_record() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "string": "lulua",
                    "int": 123456,
                },
            })).await.unwrap();
        let res = server().process_test_request(req).await.unwrap();
        assert_json!(res.body_as_json().unwrap(), matcher!({
            "data": {
                "id": ignore,
                "string": "lulua",
                "int": 123456,
            }
        }));
    }
}
