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
        let _create_res = req(&app, "create", "KOFPlayer", json!({
            "create": {
                "name": "Dan",
                "player": {
                    "create": {
                        "name": "Robert"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Dan",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Robert",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_connect() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "KOFPlayer", json!({
            "create": {
                "name": "Dan",
                "player": {
                    "connect": {
                        "name": "Justin Wong"
                    },
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Dan",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Justin Wong",
            },
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_create() {
        let app = make_app().await;
        let _create_res = req(&app, "update", "KOFPlayer", json!({
            "where": {
                "name": "Laggia"
            },
            "update": {
                "player": {
                    "create": {
                        "name": "Laggia"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Laggia",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Laggia",
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_connect() {
        let app = make_app().await;
        let _create_res = req(&app, "update", "KOFPlayer", json!({
            "where": {
                "name": "Laggia"
            },
            "update": {
                "player": {
                    "connect": {
                        "name": "Justin Wong"
                    },
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Laggia",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Justin Wong",
            },
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_set_to_another_one() {
        let app = make_app().await;
        let _create_res = req(&app, "update", "KOFPlayer", json!({
            "where": {
                "name": "Laggia"
            },
            "update": {
                "player": {
                    "set": {
                        "name": "Justin Wong"
                    },
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Laggia",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Justin Wong",
            },
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_set_to_null() {
        let app = make_app().await;
        let _create_res = req(&app, "update", "KOFPlayer", json!({
            "where": {
                "name": "Laggia"
            },
            "update": {
                "player": {
                    "set": null
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Laggia",
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Justin Wong",
            },
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_disconnect() {
        let app = make_app().await;
        let _create_res = req(&app, "update", "KOFPlayer", json!({
            "where": {
                "name": "Laggia"
            },
            "update": {
                "player": {
                    "disconnect": true
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Laggia",
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Justin Wong",
            },
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update() {
        let app = make_app().await;
        let _create_res = req(&app, "update", "KOFPlayer", json!({
            "where": {
                "name": "Justin Wong plays KOF"
            },
            "update": {
                "player": {
                    "update": {
                        "name": "Justin Wong is Teochew"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Justin Wong is Teochew",
            },
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_delete() {
        let app = make_app().await;
        let _create_res = req(&app, "update", "KOFPlayer", json!({
            "where": {
                "name": "Justin Wong plays KOF"
            },
            "update": {
                "player": {
                    "delete": true
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
        }))));
    }
}
