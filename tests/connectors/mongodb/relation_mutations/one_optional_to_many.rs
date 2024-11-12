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
        let _create_res = req(server(), "create", "Product", json!({
            "create": {
                "name": "Shampoo",
                "category": {
                    "create": {
                        "name": "Toiletries"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 6 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Shampoo",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Toiletries",
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn create_with_nested_connect() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "create", "Product", json!({
            "create": {
                "name": "Shampoo",
                "category": {
                    "connect": {
                        "name": "Cosmetics"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 6 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Shampoo",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Cosmetics",
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_create() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "Product", json!({
            "where": {
                "name": "Hair Jelly"
            },
            "update": {
                "category": {
                    "create": {
                        "name": "Toiletries"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Hair Jelly",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Toiletries",
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_connect() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "Product", json!({
            "where": {
                "name": "Hair Jelly"
            },
            "update": {
                "category": {
                    "connect": {
                        "name": "Cosmetics"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Hair Jelly",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Cosmetics",
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_set_to_another_one() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "set": {
                        "name": "Skincares"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Lipstick",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Skincares",
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_set_to_null() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "set": null
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Lipstick",
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_disconnect() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "disconnect": true
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Lipstick",
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_update() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "update": {
                        "name": "Redefined Cosmetics"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Lipstick",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Redefined Cosmetics"
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_delete() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "delete": true
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Product", json!({
            "include": {
                "category": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Lipstick",
        }))));
    }
}
