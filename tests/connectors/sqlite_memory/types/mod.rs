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

    #[test]
    fn int32() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "int32": 1,
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "int32": 1,
            }
        }))
    }

    #[test]
    fn int64() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "int64": 1,
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "int64": 1,
            }
        }))
    }

    #[test]
    fn float32() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "float32": 1.5,
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "float32": 1.5,
            }
        }))
    }

    #[test]
    fn float64() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "float64": 1.2,
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "float64": 1.2,
            }
        }))
    }

    #[test]
    fn bool() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "bool": true,
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "bool": true,
            }
        }))
    }

    #[test]
    fn string() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "string": "KOF XV",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "string": "KOF XV",
            }
        }))
    }

    #[test]
    fn date() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "date": "2005-12-25",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "date": "2005-12-25",
            }
        }))
    }

    #[test]
    fn date_time() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "dateTime": "2003-04-17T08:12:34.567Z",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "dateTime": date_time_value("2003-04-17T08:12:34.567Z"),
            }
        }))
    }

    #[test]
    fn decimal() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "decimal": "5.78",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "decimal": decimal_value("5.78"),
            }
        }))
    }

    #[test]
    fn r#enum() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "sex": "FEMALE",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "sex": "FEMALE",
            }
        }))
    }
}
