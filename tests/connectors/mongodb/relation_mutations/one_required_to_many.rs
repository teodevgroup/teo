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
        let _create_res = req(server(), "create", "Post", json!({
            "create": {
                "name": "UIKit",
                "author": {
                    "create": {
                        "name": "Jack"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Post", json!({
            "include": {
                "author": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 6 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "UIKit",
            "authorId": ignore,
            "author": {
                "id": ignore,
                "name": "Jack",
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn create_with_nested_connect() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "create", "Post", json!({
            "create": {
                "name": "UIKit",
                "author": {
                    "connect": {
                        "name": "Paul"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Post", json!({
            "include": {
                "author": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 6 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "UIKit",
            "authorId": ignore,
            "author": {
                "id": ignore,
                "name": "Paul",
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_create() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "Post", json!({
            "where": {
                "name": "Swift 1.0"
            },
            "update": {
                "author": {
                    "create": {
                        "name": "London"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Post", json!({
            "include": {
                "author": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Swift 1.0",
            "authorId": ignore,
            "author": {
                "id": ignore,
                "name": "London",
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_connect() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "Post", json!({
            "where": {
                "name": "Swift 1.0"
            },
            "update": {
                "author": {
                    "connect": {
                        "name": "David"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Post", json!({
            "include": {
                "author": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Swift 1.0",
            "authorId": ignore,
            "author": {
                "id": ignore,
                "name": "David",
            }
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_set_to_another_one() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "Post", json!({
            "where": {
                "name": "Swift 1.0"
            },
            "update": {
                "author": {
                    "set": {
                        "name": "David"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Post", json!({
            "include": {
                "author": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Swift 1.0",
            "authorId": ignore,
            "author": {
                "id": ignore,
                "name": "David",
            }
        }))));
    }


    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_update() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "update", "Post", json!({
            "where": {
                "name": "Swift 1.0"
            },
            "update": {
                "author": {
                    "update": {
                        "name": "Paul Hudson"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Post", json!({
            "include": {
                "author": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Swift 1.0",
            "authorId": ignore,
            "author": {
                "id": ignore,
                "name": "Paul Hudson"
            }
        }))));
    }
}
