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
    #[tokio::test]
    async fn create_with_nested_connect() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_create() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_connect() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_set_to_another_one() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_update() {
        before_all().await;
        before_each().await;
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
