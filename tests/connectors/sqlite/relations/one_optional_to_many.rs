use test_helpers_async::after_each;

#[cfg(test)]
#[after_each]
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
    use crate::lib::matcher_functions::one_match;
    use crate::lib::req::req;

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

    async fn after_each() {
        if let Some(handle) = unsafe { HANDLE.get() } {
            purge_and_seed(handle.teo_app()).await.unwrap();
        }
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_create() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "Product", json!({
            "create": {
                "name": "Shampoo",
                "category": {
                    "create": {
                        "name": "Toiletries"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
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
        let app = make_app().await;
        let _create_res = req(&app, "create", "Product", json!({
            "create": {
                "name": "Shampoo",
                "category": {
                    "connect": {
                        "name": "Cosmetics"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
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
        let app = make_app().await;
        let _create_res = req(&app, "update", "Product", json!({
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
        })).await;
        let find_many_res = req(&app, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
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
        let app = make_app().await;
        let _create_res = req(&app, "update", "Product", json!({
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
        })).await;
        let find_many_res = req(&app, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
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
        let app = make_app().await;
        let _create_res = req(&app, "update", "Product", json!({
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
        })).await;
        let find_many_res = req(&app, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
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
        let app = make_app().await;
        let _create_res = req(&app, "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "set": null
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Lipstick",
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_disconnect() {
        let app = make_app().await;
        let _create_res = req(&app, "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "disconnect": true
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Lipstick",
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update() {
        let app = make_app().await;
        let _create_res = req(&app, "update", "Product", json!({
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
        })).await;
        let find_many_res = req(&app, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
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
        let app = make_app().await;
        let _create_res = req(&app, "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "delete": true
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Lipstick",
        }))));
    }
}
