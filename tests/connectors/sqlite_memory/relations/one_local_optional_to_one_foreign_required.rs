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
        let _create_res = req(&app, "create", "Event", json!({
            "create": {
                "name": "The Enlightenment",
                "note": {
                    "create": {
                        "name": "Note of The Enlightenment"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Event", json!({
            "include": {
                "note": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 4 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "The Enlightenment",
            "noteId": ignore,
            "note": {
                "id": ignore,
                "name": "Note of The Enlightenment",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_connect() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "Event", json!({
            "create": {
                "name": "The Enlightenment",
                "note": {
                    "connect": {
                        "name": "Note of The Renaissance"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Event", json!({
            "include": {
                "note": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 4 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "The Enlightenment",
            "noteId": ignore,
            "note": {
                "id": ignore,
                "name": "Note of The Renaissance",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_create() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Event", json!({
            "where": {
                "name": "The Industrial Revolution"
            },
            "update": {
                "note": {
                    "create": {
                        "name": "Note of The Industrial Revolution"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Event", json!({
            "include": {
                "note": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "The Industrial Revolution",
            "noteId": ignore,
            "note": {
                "id": ignore,
                "name": "Note of The Industrial Revolution",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_connect() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Event", json!({
            "where": {
                "name": "The Industrial Revolution"
            },
            "update": {
                "note": {
                    "connect": {
                        "name": "Note of The Renaissance"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Event", json!({
            "include": {
                "note": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "The Industrial Revolution",
            "noteId": ignore,
            "note": {
                "id": ignore,
                "name": "Note of The Renaissance",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_set_to_another_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Event", json!({
            "where": {
                "name": "The Industrial Revolution"
            },
            "update": {
                "note": {
                    "set": {
                        "name": "Note of The Renaissance"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Event", json!({
            "include": {
                "note": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "The Industrial Revolution",
            "noteId": ignore,
            "note": {
                "id": ignore,
                "name": "Note of The Renaissance",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Event", json!({
            "where": {
                "name": "The Renaissance"
            },
            "update": {
                "note": {
                    "create": {
                        "name": "Note of The Renaissance II"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Event", json!({
            "include": {
                "note": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "The Renaissance",
            "noteId": ignore,
            "note": {
                "id": ignore,
                "name": "Note of The Renaissance II",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_delete() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Event", json!({
            "where": {
                "name": "The Renaissance"
            },
            "update": {
                "note": {
                    "delete": true
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Event", json!({
            "include": {
                "note": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "The Renaissance",
        }))));
    }
}
