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
        let _create_res = req(server(), "create", "Note", json!({
            "create": {
                "name": "Note of The Russian Revolutions",
                "event": {
                    "create": {
                        "name": "The Russian Revolutions"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Note", json!({
            "include": {
                "event": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
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
    #[shared_tokio_runtime::runtime_test]
    async fn create_with_nested_connect() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "create", "Note", json!({
            "create": {
                "name": "Note of The Russian Revolutions",
                "event": {
                    "connect": {
                        "name": "The Renaissance"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Note", json!({
            "include": {
                "event": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
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
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_create() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Note", json!({
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
        })).await;
        let find_many_res = req(server(), "findMany", "Note", json!({
            "include": {
                "event": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
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
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_connect() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Note", json!({
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
        })).await;
        let find_many_res = req(server(), "findMany", "Note", json!({
            "include": {
                "event": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
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
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_set_to_another_one() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Note", json!({
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
        })).await;
        let find_many_res = req(server(), "findMany", "Note", json!({
            "include": {
                "event": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
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
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_update() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Note", json!({
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
        })).await;
        let find_many_res = req(server(), "findMany", "Note", json!({
            "include": {
                "event": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
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
