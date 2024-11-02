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
    use crate::{assert_json, matcher};
    use teo::test::handle::Handle;
    use crate::lib::matcher_functions::{date_time_value, decimal_value, date_value};
    use teo::test::purge_and_seed::purge_and_seed;

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
    #[tokio::test]
    async fn int32() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "int32": 1,
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "int32": 1,
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn int64() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "int64": 1,
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "int64": 1,
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn float32() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "float32": 1.5,
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "float32": 1.5,
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn float64() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "float64": 1.2,
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "float64": 1.2,
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn bool() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "bool": true,
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "bool": true,
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn string() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "string": "KOF XV",
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "string": "KOF XV",
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn date() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "date": "2005-12-25",
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "date": date_value("2005-12-25"),
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn date_time() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "dateTime": "2003-04-17T08:12:34.567Z",
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "dateTime": date_time_value("2003-04-17T08:12:34.567Z"),
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn decimal() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "decimal": "5.78",
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "decimal": decimal_value("5.78"),
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn r#enum() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "sex": "FEMALE",
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "sex": "FEMALE",
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn int32_array() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "int32Array": [1, 2, 3],
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "int32Array": [1, 2, 3],
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn int64_array() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "int64Array": [1, 2, 3],
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "int64Array": [1, 2, 3],
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn float32_array() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "float32Array": [1.5, -1.5],
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "float32Array": [1.5, -1.5],
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn float64_array() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "float64Array": [1.2, -1.2],
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "float64Array": [1.2, -1.2],
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn bool_array() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "boolArray": [true, false],
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "boolArray": [true, false],
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn string_array() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "stringArray": ["foo", "bar"],
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "stringArray": ["foo", "bar"],
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn date_array() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "dateArray": ["2005-12-25", "2023-03-27"],
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "dateArray": [date_value("2005-12-25"), date_value("2023-03-27")],
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn date_time_array() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "dateTimeArray": ["2003-04-17T08:12:34.567Z", "1997-10-19T08:12:34.567Z"],
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "dateTimeArray": [date_time_value("2003-04-17T08:12:34.567Z"), date_time_value("1997-10-19T08:12:34.567Z")],
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn decimal_array() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "decimalArray": ["5.78", "-5.78"],
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "decimalArray": [decimal_value("5.78"),decimal_value("-5.78")],
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn enum_array() {
        before_all().await;
        before_each().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "sexesArray": ["FEMALE", "MALE"],
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "sexesArray": ["FEMALE", "MALE"],
            }
        }))
    }
}
