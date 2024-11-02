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
    #[tokio::test]
    async fn create_with_nested_connect() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_create() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_connect() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_set_to_another_one() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_update() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_delete() {
        before_all().await;
        before_each().await;
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
