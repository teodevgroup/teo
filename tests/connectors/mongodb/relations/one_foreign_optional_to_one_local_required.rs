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
    use crate::lib::matcher_functions::one_matches;
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
    #[shared_tokio_runtime::runtime_test]
    async fn create_with_nested_create() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "create", "Game", json!({
            "create": {
                "name": "KOFXIII",
                "commandList": {
                    "create": {
                        "name": "KOFXIII Command List"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 4 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "KOFXIII",
            "commandList": {
                "id": ignore,
                "name": "KOFXIII Command List",
                "gameId": ignore,
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn create_with_nested_connect() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "create", "Game", json!({
            "create": {
                "name": "KOFXIII",
                "commandList": {
                    "connect": {
                        "name": "KOF98 Command List"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 4 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "KOFXIII",
            "commandList": {
                "id": ignore,
                "name": "KOF98 Command List",
                "gameId": ignore,
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_create() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Game", json!({
            "where": {
                "name": "KOFXV"
            },
            "update": {
                "commandList": {
                    "create": {
                        "name": "KOFXV Command List"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "KOFXV",
            "commandList": {
                "id": ignore,
                "name": "KOFXV Command List",
                "gameId": ignore,
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_connect() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Game", json!({
            "where": {
                "name": "KOFXV"
            },
            "update": {
                "commandList": {
                    "connect": {
                        "name": "KOF97 Command List"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "KOFXV",
            "commandList": {
                "id": ignore,
                "name": "KOF97 Command List",
                "gameId": ignore,
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_set_to_another_one() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Game", json!({
            "where": {
                "name": "KOFXV"
            },
            "update": {
                "commandList": {
                    "set": {
                        "name": "KOF97 Command List"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "KOFXV",
            "commandList": {
                "id": ignore,
                "name": "KOF97 Command List",
                "gameId": ignore,
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_update() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Game", json!({
            "where": {
                "name": "KOF98"
            },
            "update": {
                "commandList": {
                    "update": {
                        "name": "KOF98UM Command List"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "KOF98",
            "commandList": {
                "id": ignore,
                "name": "KOF98UM Command List",
                "gameId": ignore,
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_delete() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Game", json!({
            "where": {
                "name": "KOF98"
            },
            "update": {
                "commandList": {
                    "delete": true
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "KOF98",
        }))));
    }
}
