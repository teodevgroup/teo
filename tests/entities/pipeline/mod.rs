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
            "data": partial({
                "id": ignore,
                "int32": 10,
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_int64() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "int64": 1,
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "int64": 10,
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_float32() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "float32": 1.0,
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "float32": 10.0,
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_float64() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "float64": 1.0,
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "float64": 10.0,
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_bool() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "bool": false,
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "bool": true,
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_date() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "date": "2005-06-01",
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "date": date_value("2005-06-02"),
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_date_time() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "dateTime": "2024-11-29T14:49:13.498Z",
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "dateTime": date_time_value("2024-11-30T14:49:13.498Z"),
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_decimal() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "decimal": "1",
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "decimal": decimal_value("10"),
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_status() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "status": "open",
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "status": "pending",
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_int32_array() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "int32Array": [1, 1],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "int32Array": [10, 10],
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_int64_array() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "int64Array": [1, 1],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "int64Array": [10, 10],
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_float32_array() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "float32Array": [1.0, 1.0],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "float32Array": [10.0, 10.0],
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_float64_array() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "float64Array": [1.0, 1.0],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "float64Array": [10.0, 10.0],
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_bool_array() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "boolArray": [false, false],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "boolArray": [true, true],
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_date_array() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "dateArray": ["2005-06-01", "2005-06-01"],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "dateArray": [date_value("2005-06-02"), date_value("2005-06-02")],
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_date_time_array() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "dateTimeArray": ["2024-11-29T14:49:13.498Z", "2024-11-29T14:49:13.498Z"],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "dateTimeArray": [date_time_value("2024-11-30T14:49:13.498Z"), date_time_value("2024-11-30T14:49:13.498Z")],
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_decimal_array() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "decimalArray": ["1", "1"],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "decimalArray": [decimal_value("10"), decimal_value("10")],
            })
        }))
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn transform_status_array() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Container/create")
            .json_body(json!({
                "create": {
                    "statusArray": ["open", "open"],
                },
            }))
            .await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": partial({
                "id": ignore,
                "statusArray": ["pending", "pending"],
            })
        }))
    }
}
