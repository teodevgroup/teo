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
        let _create_res = req(PORT, "create", "CommandList", json!({
            "create": {
                "name": "KOFXIV Command List",
                "game": {
                    "create": {
                        "name": "KOFXIV"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        }));
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
    #[actix_web::test]
    async fn create_with_nested_connect() {
        let _create_res = req(PORT, "create", "CommandList", json!({
            "create": {
                "name": "KOFXIV Command List",
                "game": {
                    "connect": {
                        "name": "KOFXV"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        }));
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
    #[actix_web::test]
    async fn update_with_nested_create() {
        let _update_res = req(PORT, "update", "CommandList", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        }));
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
    #[actix_web::test]
    async fn update_with_nested_connect() {
        let _update_res = req(PORT, "update", "CommandList", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        }));
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
    #[actix_web::test]
    async fn update_with_nested_set_to_another_one() {
        let _update_res = req(PORT, "update", "CommandList", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        }));
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
    #[actix_web::test]
    async fn update_with_nested_update() {
        let _update_res = req(PORT, "update", "CommandList", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        }));
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
