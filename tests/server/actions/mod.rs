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
                let app = App::new_with_argv(
                    schema_path_args(file!(), "schema.teo")
                ).unwrap();
                let mut server = Server::new(app);
                server
            })
        };
        server().setup_app_for_unit_test().await.unwrap();
        unsafe { BEFORE_ALL_EXECUTED = true; }
    }

    async fn before_each() {
        server().reset_app_for_unit_test().await.unwrap();
    }

    #[serial]
    #[tokio::test]
    async fn test_get_index() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::GET, "/")
            .insert_header(CONTENT_TYPE, TEXT_PLAIN.essence_str());
        let res = server().process_test_request(req).await.unwrap();
        assert_eq!(res.status().as_u16(), 404);
    }
    //
    // #[serial]
    // async fn test_create_record() {
    //     let app = make_app().await;
    //     let req = test::TestRequest::default()
    //         .method(Method::POST)
    //         .uri("/Support/create")
    //         .set_json(json!({
    //             "create": {
    //                 "string": "lulua",
    //                 "int": 123456,
    //             },
    //         }))
    //         .to_request();
    //     let res_body: Value = test::call_and_read_body_json(&app, req).await;
    //     assert_json!(res_body, matcher!({
    //         "data": {
    //             "id": ignore,
    //             "string": "lulua",
    //             "int": 123456,
    //         }
    //     }));
    // }
}
