use test_helpers_async::before_each;

#[cfg(test)]
#[before_each]
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
    use serde_json::json;
    use crate::{assert_json, matcher};
    use crate::lib::handle::Handle;
    use serial_test::serial;
    use crate::lib::purge_and_seed::purge_and_seed;
    use crate::lib::matcher_functions::one_match;
    use crate::lib::req::req;

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
        let app = make_app().await;
        let _create_res = req(&app, "create", "Post", json!({
            "create": {
                "name": "UIKit",
                "author": {
                    "create": {
                        "name": "Jack"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Post", json!({
            "include": {
                "author": true
            }
        })).await;
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
    #[actix_web::test]
    async fn create_with_nested_connect() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "Post", json!({
            "create": {
                "name": "UIKit",
                "author": {
                    "connect": {
                        "name": "Paul"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Post", json!({
            "include": {
                "author": true
            }
        })).await;
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
    #[actix_web::test]
    async fn update_with_nested_create() {
        let app = make_app().await;
        let _create_res = req(&app, "update", "Post", json!({
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
        let find_many_res = req(&app, "findMany", "Post", json!({
            "include": {
                "author": true
            }
        })).await;
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
    #[actix_web::test]
    async fn update_with_nested_connect() {
        let app = make_app().await;
        let _create_res = req(&app, "update", "Post", json!({
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
        let find_many_res = req(&app, "findMany", "Post", json!({
            "include": {
                "author": true
            }
        })).await;
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
    #[actix_web::test]
    async fn update_with_nested_set_to_another_one() {
        let app = make_app().await;
        let _create_res = req(&app, "update", "Post", json!({
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
        let find_many_res = req(&app, "findMany", "Post", json!({
            "include": {
                "author": true
            }
        })).await;
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
    #[actix_web::test]
    async fn update_with_nested_update() {
        let app = make_app().await;
        let _create_res = req(&app, "update", "Post", json!({
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
        let find_many_res = req(&app, "findMany", "Post", json!({
            "include": {
                "author": true
            }
        })).await;
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
