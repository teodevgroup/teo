#[cfg(test)]
mod tests {
    use std::cell::OnceCell;
    use teo::prelude::App;
    use std::file;
    use teo::server::server::Server;
    use teo::test::schema_path::schema_path_args;
    use serde_json::{json, Value};
    use crate::{assert_json, matcher};
    use serial_test::serial;
    use crate::lib::matcher_functions::one_match;
    use teo::test::req::req;
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
                Server::new(App::new_with_argv(
                    schema_path_args(file!(), "schema.teo")
                ).unwrap())
            })
        };
        server().setup_app_for_unit_test().await.unwrap();
        unsafe { BEFORE_ALL_EXECUTED = true; }
    }

    async fn before_each() {
        server().reset_app_for_unit_test().await.unwrap();
    }

    #[serial]
    #[tokio::test]
    async fn create_with_nested_create() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "create", "KOFPlayer", json!({
            "create": {
                "name": "Dan",
                "player": {
                    "create": {
                        "name": "Robert"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "KOFPlayer", json!({
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
    #[tokio::test]
    async fn create_with_nested_connect() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "create", "KOFPlayer", json!({
            "create": {
                "name": "Dan",
                "player": {
                    "connect": {
                        "name": "Justin Wong"
                    },
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "KOFPlayer", json!({
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
    #[tokio::test]
    async fn update_with_nested_create() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "KOFPlayer", json!({
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
        let find_many_res = req(server(), "findMany", "KOFPlayer", json!({
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
    #[tokio::test]
    async fn update_with_nested_connect() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "KOFPlayer", json!({
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
        let find_many_res = req(server(), "findMany", "KOFPlayer", json!({
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
    #[tokio::test]
    async fn update_with_nested_set_to_another_one() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "KOFPlayer", json!({
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
        let find_many_res = req(server(), "findMany", "KOFPlayer", json!({
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
    #[tokio::test]
    async fn update_with_nested_set_to_null() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "KOFPlayer", json!({
            "where": {
                "name": "Laggia"
            },
            "update": {
                "player": {
                    "set": null
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "KOFPlayer", json!({
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
    #[tokio::test]
    async fn update_with_nested_disconnect() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "KOFPlayer", json!({
            "where": {
                "name": "Laggia"
            },
            "update": {
                "player": {
                    "disconnect": true
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "KOFPlayer", json!({
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
    #[tokio::test]
    async fn update_with_nested_update() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "KOFPlayer", json!({
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
        let find_many_res = req(server(), "findMany", "KOFPlayer", json!({
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
    #[tokio::test]
    async fn update_with_nested_delete() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "KOFPlayer", json!({
            "where": {
                "name": "Justin Wong plays KOF"
            },
            "update": {
                "player": {
                    "delete": true
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "KOFPlayer", json!({
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
