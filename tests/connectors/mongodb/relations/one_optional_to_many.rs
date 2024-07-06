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
        let _create_res = req(PORT, "create", "Product", json!({
            "create": {
                "name": "Shampoo",
                "category": {
                    "create": {
                        "name": "Toiletries"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 6 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Shampoo",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Toiletries",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_connect() {
        let _create_res = req(PORT, "create", "Product", json!({
            "create": {
                "name": "Shampoo",
                "category": {
                    "connect": {
                        "name": "Cosmetics"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 6 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Shampoo",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Cosmetics",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_create() {
        let _create_res = req(PORT, "update", "Product", json!({
            "where": {
                "name": "Hair Jelly"
            },
            "update": {
                "category": {
                    "create": {
                        "name": "Toiletries"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Hair Jelly",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Toiletries",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_connect() {
        let _create_res = req(PORT, "update", "Product", json!({
            "where": {
                "name": "Hair Jelly"
            },
            "update": {
                "category": {
                    "connect": {
                        "name": "Cosmetics"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Hair Jelly",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Cosmetics",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_set_to_another_one() {
        let _create_res = req(PORT, "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "set": {
                        "name": "Skincares"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Lipstick",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Skincares",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_set_to_null() {
        let _create_res = req(PORT, "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "set": null
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Lipstick",
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_disconnect() {
        let _create_res = req(PORT, "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "disconnect": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Lipstick",
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update() {
        let _create_res = req(PORT, "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "update": {
                        "name": "Redefined Cosmetics"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Lipstick",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Redefined Cosmetics"
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_delete() {
        let _create_res = req(PORT, "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "delete": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Lipstick",
        }))));
    }
}
