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

    async fn before_each() {
        if let Some(handle) = unsafe { HANDLE.get() } {
            purge_and_seed(handle.teo_app()).await.unwrap();
        }
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_create() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "Note", json!({
            "create": {
                "name": "Note of The Russian Revolutions",
                "event": {
                    "create": {
                        "name": "The Russian Revolutions"
                    }
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Note", json!({
            "include": {
                "event": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Note of The Russian Revolutions",
            "event": {
                "id": ignore,
                "name": "The Russian Revolutions",
                "noteId": ignore,
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_connect() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "Note", json!({
            "create": {
                "name": "Note of The Russian Revolutions",
                "event": {
                    "connect": {
                        "name": "The Renaissance"
                    }
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Note", json!({
            "include": {
                "event": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Note of The Russian Revolutions",
            "event": {
                "id": ignore,
                "name": "The Renaissance",
                "noteId": ignore,
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_create() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Note", json!({
            "where": {
                "name": "Note of The Renaissance",
            },
            "update": {
                "event": {
                    "create": {
                        "name": "Understanding The Renaissance"
                    }
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Note", json!({
            "include": {
                "event": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Note of The Renaissance",
            "event": {
                "id": ignore,
                "name": "Understanding The Renaissance",
                "noteId": ignore,
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_connect() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Note", json!({
            "where": {
                "name": "Note of The Renaissance",
            },
            "update": {
                "event": {
                    "connect": {
                        "name": "The French Revolution"
                    }
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Note", json!({
            "include": {
                "event": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Note of The Renaissance",
            "event": {
                "id": ignore,
                "name": "The French Revolution",
                "noteId": ignore,
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_set_to_another_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Note", json!({
            "where": {
                "name": "Note of The Renaissance",
            },
            "update": {
                "event": {
                    "set": {
                        "name": "The French Revolution"
                    }
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Note", json!({
            "include": {
                "event": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Note of The Renaissance",
            "event": {
                "id": ignore,
                "name": "The French Revolution",
                "noteId": ignore,
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Note", json!({
            "where": {
                "name": "Note of The Renaissance",
            },
            "update": {
                "event": {
                    "update": {
                        "name": "Memorize The French Revolution"
                    }
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Note", json!({
            "include": {
                "event": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Note of The Renaissance",
            "event": {
                "id": ignore,
                "name": "Memorize The French Revolution",
                "noteId": ignore,
            }
        }))));
    }
}
