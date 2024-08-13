use test_helpers_async::after_each;

#[cfg(test)]
#[after_each]
mod tests {
    use std::cell::OnceCell;
    use actix_web::{http::header::ContentType, test};
    use teo::test::server::make_actix_app;
    use teo::prelude::App;
    use std::file;
    use actix_http::body::MessageBody;
    use actix_http::Method;
    use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
    use teo::test::schema_path::schema_path_args;
    use serde_json::{json, Value};
    use crate::{assert_json, matcher};
    use teo::test::handle::Handle;
    use serial_test::serial;
    use crate::lib::matcher_functions::one_match;
    use teo::test::purge_and_seed::purge_and_seed;
    use teo::test::req::req;

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

    async fn after_each() {
        if let Some(handle) = unsafe { HANDLE.get() } {
            purge_and_seed(handle.teo_app()).await.unwrap();
        }
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_create() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "Author", json!({
            "create": {
                "name": "Jack",
                "posts": {
                    "create": {
                        "name": "PyPy"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Jack",
            "posts": [
                {
                    "id": ignore,
                    "name": "PyPy",
                    "authorId": ignore,
                }
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_create_many() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "Author", json!({
            "create": {
                "name": "Jack",
                "posts": {
                    "create": [
                        {
                            "name": "PyPy"
                        },
                        {
                            "name": "NoNo"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "desc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Jack",
            "posts": [
                {
                    "id": ignore,
                    "name": "PyPy",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "NoNo",
                    "authorId": ignore,
                }
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_connect_one() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "Author", json!({
            "create": {
                "name": "Jack",
                "posts": {
                    "connect": {
                        "name": "Swift 3.0"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Jack",
            "posts": [
                {
                    "id": ignore,
                    "name": "Swift 3.0",
                    "authorId": ignore,
                }
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_connect_more_than_one() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "Author", json!({
            "create": {
                "name": "Jack",
                "posts": {
                    "connect": [
                        {
                            "name": "Swift 2.0"
                        },
                        {
                            "name": "Swift 3.0"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Jack",
            "posts": [
                {
                    "id": ignore,
                    "name": "Swift 2.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 3.0",
                    "authorId": ignore,
                }
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_create_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Author", json!({
            "where": {
                "name": "Paul"
            },
            "update": {
                "posts": {
                    "create": {
                        "name": "Swift 4.0"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Paul",
            "posts": [
                {
                    "id": ignore,
                    "name": "Swift 1.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 2.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 3.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 4.0",
                    "authorId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_create_many() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Author", json!({
            "where": {
                "name": "Paul"
            },
            "update": {
                "posts": {
                    "create": [
                        {
                            "name": "Swift 4.0"
                        },
                        {
                            "name": "Swift 5.0"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Paul",
            "posts": [
                {
                    "id": ignore,
                    "name": "Swift 1.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 2.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 3.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 4.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 5.0",
                    "authorId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_connect_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Author", json!({
            "where": {
                "name": "Paul"
            },
            "update": {
                "posts": {
                    "connect": {
                        "name": "Ruby on Rails 1.0"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Paul",
            "posts": [
                {
                    "id": ignore,
                    "name": "Ruby on Rails 1.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 1.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 2.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 3.0",
                    "authorId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_connect_more_than_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Author", json!({
            "where": {
                "name": "Paul"
            },
            "update": {
                "posts": {
                    "connect": [
                        {
                            "name": "Ruby on Rails 1.0"
                        },
                        {
                            "name": "Ruby on Rails 2.0"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Paul",
            "posts": [
                {
                    "id": ignore,
                    "name": "Ruby on Rails 1.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Ruby on Rails 2.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 1.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 2.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 3.0",
                    "authorId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Author", json!({
            "where": {
                "name": "Paul"
            },
            "update": {
                "posts": {
                    "update": {
                        "where": {
                            "name": "Swift 1.0"
                        },
                        "update": {
                            "name": "SwiftUI"
                        }
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Paul",
            "posts": [
                {
                    "id": ignore,
                    "name": "Swift 2.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 3.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "SwiftUI",
                    "authorId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update_more_than_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Author", json!({
            "where": {
                "name": "Paul"
            },
            "update": {
                "posts": {
                    "update": [
                        {
                            "where": {
                                "name": "Swift 1.0"
                            },
                            "update": {
                                "name": "SwiftUI"
                            }
                        },
                        {
                            "where": {
                                "name": "Swift 2.0"
                            },
                            "update": {
                                "name": "Swift Package Manager"
                            }
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Paul",
            "posts": [
                {
                    "id": ignore,
                    "name": "Swift 3.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift Package Manager",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "SwiftUI",
                    "authorId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update_many() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Author", json!({
            "where": {
                "name": "Paul"
            },
            "update": {
                "posts": {
                    "updateMany": {
                        "where": {
                            "name": "Swift 1.0"
                        },
                        "update": {
                            "name": "SwiftUI"
                        }
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Paul",
            "posts": [
                {
                    "id": ignore,
                    "name": "Swift 2.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 3.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "SwiftUI",
                    "authorId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update_many_more() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Author", json!({
            "where": {
                "name": "Paul"
            },
            "update": {
                "posts": {
                    "updateMany": [
                        {
                            "where": {
                                "name": "Swift 1.0"
                            },
                            "update": {
                                "name": "SwiftUI"
                            }
                        },
                        {
                            "where": {
                                "name": "Swift 2.0"
                            },
                            "update": {
                                "name": "Swift Package Manager"
                            }
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Paul",
            "posts": [
                {
                    "id": ignore,
                    "name": "Swift 3.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift Package Manager",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "SwiftUI",
                    "authorId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_delete_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Author", json!({
            "where": {
                "name": "Paul"
            },
            "update": {
                "posts": {
                    "delete": {
                        "name": "Swift 1.0"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Paul",
            "posts": [
                {
                    "id": ignore,
                    "name": "Swift 2.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 3.0",
                    "authorId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_delete_more_than_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Author", json!({
            "where": {
                "name": "Paul"
            },
            "update": {
                "posts": {
                    "delete": [
                        {
                            "name": "Swift 1.0"
                        },
                        {
                            "name": "Swift 2.0"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Paul",
            "posts": [
                {
                    "id": ignore,
                    "name": "Swift 3.0",
                    "authorId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_delete_many() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Author", json!({
            "where": {
                "name": "Paul"
            },
            "update": {
                "posts": {
                    "deleteMany": {
                        "name": "Swift 1.0"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Paul",
            "posts": [
                {
                    "id": ignore,
                    "name": "Swift 2.0",
                    "authorId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Swift 3.0",
                    "authorId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_delete_many_more() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Author", json!({
            "where": {
                "name": "Paul"
            },
            "update": {
                "posts": {
                    "deleteMany": [
                        {
                            "name": "Swift 1.0"
                        },
                        {
                            "name": "Swift 2.0"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Paul",
            "posts": [
                {
                    "id": ignore,
                    "name": "Swift 3.0",
                    "authorId": ignore,
                },
            ]
        }))));
    }
}
