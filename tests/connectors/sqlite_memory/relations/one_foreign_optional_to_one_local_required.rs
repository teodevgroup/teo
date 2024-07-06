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

    async fn before_each() {
        if let Some(handle) = unsafe { HANDLE.get() } {
            purge_and_seed(handle.teo_app()).await.unwrap();
        }
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_create() {
        let _create_res = req(PORT, "create", "Game", json!({
            "create": {
                "name": "KOFXIII",
                "commandList": {
                    "create": {
                        "name": "KOFXIII Command List"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 4 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn create_with_nested_connect() {
        let _create_res = req(PORT, "create", "Game", json!({
            "create": {
                "name": "KOFXIII",
                "commandList": {
                    "connect": {
                        "name": "KOF98 Command List"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 4 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_create() {
        let _update_res = req(PORT, "update", "Game", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_connect() {
        let _update_res = req(PORT, "update", "Game", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_set_to_another_one() {
        let _update_res = req(PORT, "update", "Game", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_update() {
        let _update_res = req(PORT, "update", "Game", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_delete() {
        let _update_res = req(PORT, "update", "Game", json!({
            "where": {
                "name": "KOF98"
            },
            "update": {
                "commandList": {
                    "delete": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOF98",
        }))));
    }
}
