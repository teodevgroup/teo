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
        let _create_res = req(server(), "create", "User", json!({
            "create": {
                "name": "Dan",
                "profile": {
                    "create": {
                        "name": "Dan"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "User", json!({
            "include": {
                "profile": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Dan",
            "profile": {
                "id": ignore,
                "name": "Dan",
                "userId": ignore,
            }
        }))));
    }

    #[serial]
    #[tokio::test]
    async fn update_with_nested_update() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "User", json!({
            "where": {
                "name": "John"
            },
            "update": {
                "profile": {
                    "update": {
                        "name": "Class 1"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "User", json!({
            "include": {
                "profile": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "John",
            "profile": {
                "id": ignore,
                "name": "Class 1",
                "userId": ignore,
            }
        }))));
    }
}
