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
    use crate::{assert_json, matcher};
    use crate::lib::handle::Handle;
    use serial_test::serial;
    use crate::lib::purge_and_seed::purge_and_seed;

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
                    App::new_with_argv(
                        schema_path_args(file!(), "schema.teo")
                    ).unwrap()
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
    async fn create_with_nested_create() {
        let _create_res = req(PORT, "create", "Profile", json!({
            "create": {
                "name": "Dan",
                "user": {
                    "create": {
                        "name": "Dan"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Profile", json!({
            "include": {
                "user": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Dan",
            "userId": ignore,
            "user": {
                "id": ignore,
                "name": "Dan",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update() {
        let _update_res = req(PORT, "update", "Profile", json!({
            "where": {
                "name": "John's profile"
            },
            "update": {
                "user": {
                    "update": {
                        "name": "Class 1"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Profile", json!({
            "include": {
                "user": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "John's profile",
            "userId": ignore,
            "user": {
                "id": ignore,
                "name": "Class 1",
            }
        }))));
    }
}
