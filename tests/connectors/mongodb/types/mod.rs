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
    fn object_id() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "objectId": "123456789012345678901234",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "objectId": "123456789012345678901234",
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

    #[test]
    fn int32_array() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "int32Array": [1, 2, 3],
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "int32Array": [1, 2, 3],
            }
        }))
    }

    #[test]
    fn int64_array() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "int64Array": [1, 2, 3],
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "int64Array": [1, 2, 3],
            }
        }))
    }

    #[test]
    fn float32_array() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "float32Array": [1.5, -1.5],
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "float32Array": [1.5, -1.5],
            }
        }))
    }

    #[test]
    fn float64_array() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "float64Array": [1.2, -1.2],
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "float64Array": [1.2, -1.2],
            }
        }))
    }

    #[test]
    fn bool_array() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "boolArray": [true, false],
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "boolArray": [true, false],
            }
        }))
    }

    #[test]
    fn string_array() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "stringArray": ["foo", "bar"],
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "stringArray": ["foo", "bar"],
            }
        }))
    }

    #[test]
    fn date_array() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "dateArray": ["2005-12-25", "2023-03-27"],
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "dateArray": ["2005-12-25", "2023-03-27"],
            }
        }))
    }

    #[test]
    fn date_time_array() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "dateTimeArray": ["2003-04-17T08:12:34.567Z", "1997-10-19T08:12:34.567Z"],
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "dateTimeArray": [date_time_value("2003-04-17T08:12:34.567Z"), date_time_value("1997-10-19T08:12:34.567Z")],
            }
        }))
    }

    #[test]
    fn object_id_array() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "objectIdArray": ["123456789012345678901234", "432109876543210987654321"],
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "objectIdArray": ["123456789012345678901234", "432109876543210987654321"],
            }
        }))
    }

    #[test]
    fn enum_array() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "sexesArray": ["FEMALE", "MALE"],
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "sexesArray": ["FEMALE", "MALE"],
            }
        }))
    }
}
