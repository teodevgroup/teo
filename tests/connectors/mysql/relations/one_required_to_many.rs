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
    #[test]
    fn create_with_nested_create() {
        let _create_res = req(PORT, "create", "Post", json!({
            "create": {
                "name": "UIKit",
                "author": {
                    "create": {
                        "name": "Jack"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Post", json!({
            "include": {
                "author": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 6 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[test]
    fn create_with_nested_connect() {
        let _create_res = req(PORT, "create", "Post", json!({
            "create": {
                "name": "UIKit",
                "author": {
                    "connect": {
                        "name": "Paul"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Post", json!({
            "include": {
                "author": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 6 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[test]
    fn update_with_nested_create() {
        let _create_res = req(PORT, "update", "Post", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Post", json!({
            "include": {
                "author": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[test]
    fn update_with_nested_connect() {
        let _create_res = req(PORT, "update", "Post", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Post", json!({
            "include": {
                "author": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[test]
    fn update_with_nested_set_to_another_one() {
        let _create_res = req(PORT, "update", "Post", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Post", json!({
            "include": {
                "author": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[test]
    fn update_with_nested_update() {
        let _create_res = req(PORT, "update", "Post", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Post", json!({
            "include": {
                "author": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
