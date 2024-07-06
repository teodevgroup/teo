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
        let _create_res = req(PORT, "create", "Player", json!({
            "create": {
                "name": "Dan",
                "kof": {
                    "create": {
                        "name": "Robert"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Dan",
            "kof": {
                "id": ignore,
                "name": "Robert",
                "playerId": ignore,
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_connect() {
        let _create_res = req(PORT, "create", "Player", json!({
            "create": {
                "name": "Dan",
                "kof": {
                    "connect": {
                        "name": "Justin Wong plays KOF"
                    },
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Dan",
            "kof": {
                "id": ignore,
                "name": "Justin Wong plays KOF",
                "playerId": ignore,
            },
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_create() {
        let _update_res = req(PORT, "update", "Player", json!({
            "where": {
                "name": "Justin Wong"
            },
            "update": {
                "kof": {
                    "create": {
                        "name": "Ash Crimson"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong",
            "kof": {
                "id": ignore,
                "name": "Ash Crimson",
                "playerId": ignore,
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_connect() {
        let _update_res = req(PORT, "update", "Player", json!({
            "where": {
                "name": "Justin Wong"
            },
            "update": {
                "kof": {
                    "connect": {
                        "name": "Laggia"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong",
            "kof": {
                "id": ignore,
                "name": "Laggia",
                "playerId": ignore,
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_set_to_another_one() {
        let _update_res = req(PORT, "update", "Player", json!({
            "where": {
                "name": "Justin Wong"
            },
            "update": {
                "kof": {
                    "set": {
                        "name": "Laggia"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong",
            "kof": {
                "id": ignore,
                "name": "Laggia",
                "playerId": ignore,
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_set_to_null() {
        let _update_res = req(PORT, "update", "Player", json!({
            "where": {
                "name": "Justin Wong"
            },
            "update": {
                "kof": {
                    "set": null
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong",
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_disconnect() {
        let _update_res = req(PORT, "update", "Player", json!({
            "where": {
                "name": "Justin Wong"
            },
            "update": {
                "kof": {
                    "disconnect": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong",
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update() {
        let _update_res = req(PORT, "update", "Player", json!({
            "where": {
                "name": "Justin Wong"
            },
            "update": {
                "kof": {
                    "update": {
                        "name": "Ash Crimson"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong",
            "kof": {
                "id": ignore,
                "name": "Ash Crimson",
                "playerId": ignore,
            }
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_delete() {
        let _update_res = req(PORT, "update", "Player", json!({
            "where": {
                "name": "Justin Wong"
            },
            "update": {
                "kof": {
                    "delete": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong",
        }))));
    }
}
