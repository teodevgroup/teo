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

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_create() {
        let _create_res = req(PORT, "create", "KOFPlayer", json!({
            "create": {
                "name": "Dan",
                "player": {
                    "create": {
                        "name": "Robert"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
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
        let _create_res = req(PORT, "create", "KOFPlayer", json!({
            "create": {
                "name": "Dan",
                "player": {
                    "connect": {
                        "name": "Justin Wong"
                    },
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
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
        let _create_res = req(PORT, "update", "KOFPlayer", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
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
        let _create_res = req(PORT, "update", "KOFPlayer", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
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
        let _create_res = req(PORT, "update", "KOFPlayer", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
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
        let _create_res = req(PORT, "update", "KOFPlayer", json!({
            "where": {
                "name": "Laggia"
            },
            "update": {
                "player": {
                    "set": null
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
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
        let _create_res = req(PORT, "update", "KOFPlayer", json!({
            "where": {
                "name": "Laggia"
            },
            "update": {
                "player": {
                    "disconnect": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
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
        let _create_res = req(PORT, "update", "KOFPlayer", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
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
        let _create_res = req(PORT, "update", "KOFPlayer", json!({
            "where": {
                "name": "Justin Wong plays KOF"
            },
            "update": {
                "player": {
                    "delete": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
        }))));
    }
}
