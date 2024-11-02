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
    #[tokio::test]
    async fn int32() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Support/create")
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
                "int32": 1,
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn int64() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "int64": 1,
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "float32": 1.5,
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "float64": 1.2,
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "bool": true,
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "string": "KOF XV",
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "date": "2005-12-25",
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "dateTime": "2003-04-17T08:12:34.567Z",
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "decimal": "5.78",
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "sex": "FEMALE",
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "int32Array": [1, 2, 3],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "int64Array": [1, 2, 3],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "float32Array": [1.5, -1.5],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "float64Array": [1.2, -1.2],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "boolArray": [true, false],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "stringArray": ["foo", "bar"],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "dateArray": ["2005-12-25", "2023-03-27"],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "dateTimeArray": ["2003-04-17T08:12:34.567Z", "1997-10-19T08:12:34.567Z"],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "decimalArray": ["5.78", "-5.78"],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
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
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "sexesArray": ["FEMALE", "MALE"],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "sexesArray": ["FEMALE", "MALE"],
            }
        }))
    }
}
