use test_helpers_async::after_each;

#[cfg(test)]
#[after_each]
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
    use crate::{assert_json, matcher};
    use teo::test::handle::Handle;
    use serial_test::serial;
    use teo::test::purge_and_seed::purge_and_seed;
    use crate::lib::matcher_functions::one_match;
    use teo::test::req::req;

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
        let _create_res = req(&app, "create", "CommandList", json!({
            "create": {
                "name": "KOFXIV Command List",
                "game": {
                    "create": {
                        "name": "KOFXIV"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOFXIV Command List",
            "gameId": ignore,
            "game": {
                "id": ignore,
                "name": "KOFXIV",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_connect() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "CommandList", json!({
            "create": {
                "name": "KOFXIV Command List",
                "game": {
                    "connect": {
                        "name": "KOFXV"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOFXIV Command List",
            "gameId": ignore,
            "game": {
                "id": ignore,
                "name": "KOFXV",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_create() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "CommandList", json!({
            "where": {
                "name": "KOF97 Command List"
            },
            "update": {
                "game": {
                    "create": {
                        "name": "KOF96"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOF97 Command List",
            "gameId": ignore,
            "game": {
                "id": ignore,
                "name": "KOF96",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_connect() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "CommandList", json!({
            "where": {
                "name": "KOF97 Command List"
            },
            "update": {
                "game": {
                    "connect": {
                        "name": "KOFXV"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOF97 Command List",
            "gameId": ignore,
            "game": {
                "id": ignore,
                "name": "KOFXV",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_set_to_another_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "CommandList", json!({
            "where": {
                "name": "KOF97 Command List"
            },
            "update": {
                "game": {
                    "set": {
                        "name": "KOFXV"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOF97 Command List",
            "gameId": ignore,
            "game": {
                "id": ignore,
                "name": "KOFXV",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "CommandList", json!({
            "where": {
                "name": "KOF97 Command List"
            },
            "update": {
                "game": {
                    "update": {
                        "name": "KOF95"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOF97 Command List",
            "gameId": ignore,
            "game": {
                "id": ignore,
                "name": "KOF95",
            }
        }))));
    }
}
