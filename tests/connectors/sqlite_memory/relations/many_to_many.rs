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
    async fn create_with_nested_create_one() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "Artist", json!({
            "create": {
                "name": "Taylor Swift",
                "songs": {
                    "create": {
                        "name": "Love Story"
                    }
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Taylor Swift",
            "songs": [
                {
                    "id": ignore,
                    "name": "Love Story"
                }
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_create_many() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "Artist", json!({
            "create": {
                "name": "Taylor Swift",
                "songs": {
                    "createMany": [
                        {
                            "name": "Love Story"
                        },
                        {
                            "name": "Red"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Taylor Swift",
            "songs": [
                {
                    "id": ignore,
                    "name": "Love Story"
                },
                {
                    "id": ignore,
                    "name": "Red"
                }
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_connect_one() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "Artist", json!({
            "create": {
                "name": "Taylor Swift",
                "songs": {
                    "connect": {
                        "name": "Perfect"
                    },
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Taylor Swift",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
            ]
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Shape of You"
                }
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn create_with_nested_connect_more_than_one() {
        let app = make_app().await;
        let _create_res = req(&app, "create", "Artist", json!({
            "create": {
                "name": "Taylor Swift",
                "songs": {
                    "connect": [
                        {
                            "name": "Perfect"
                        },
                        {
                            "name": "Maps"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Taylor Swift",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Perfect"
                },
            ]
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Shape of You"
                }
            ]
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Maroon 5",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Payphone"
                }
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_create_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "create": {
                        "name": "Photograph"
                    }
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Photograph"
                },
                {
                    "id": ignore,
                    "name": "Shape of You"
                }
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_create_many() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "createMany": [
                        {
                            "name": "Photograph"
                        },
                        {
                            "name": "Castle on the Hill"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Castle on the Hill"
                },
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Photograph"
                },
                {
                    "id": ignore,
                    "name": "Shape of You"
                }
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_connect_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "connect": {
                        "name": "Maps"
                    }
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Shape of You"
                }
            ]
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Maroon 5",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Payphone"
                }
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_connect_more_than_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "connect": [
                        {
                            "name": "Maps"
                        },
                        {
                            "name": "Payphone"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Payphone"
                },
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Shape of You"
                }
            ]
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Maroon 5",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Payphone"
                }
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_set() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "set": [
                        {
                            "name": "Maps"
                        },
                        {
                            "name": "Payphone"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Payphone"
                },
            ]
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Maroon 5",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Payphone"
                }
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_disconnect_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "disconnect": {
                        "name": "Shape of You"
                    }
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_disconnect_more_than_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "disconnect": [
                        {
                            "name": "Shape of You"
                        },
                        {
                            "name": "Perfect"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": []
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "update": {
                        "where": {
                            "name": "Shape of You"
                        },
                        "update": {
                            "name": "Shape of You - Radio Edit"
                        }
                    }
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Shape of You - Radio Edit"
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update_more_than_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "update": [
                        {
                            "where": {
                                "name": "Shape of You"
                            },
                            "update": {
                                "name": "Shape of You - Radio Edit"
                            }
                        },
                        {
                            "where": {
                                "name": "Perfect"
                            },
                            "update": {
                                "name": "Perfect - Radio Edit"
                            }
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect - Radio Edit"
                },
                {
                    "id": ignore,
                    "name": "Shape of You - Radio Edit"
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update_many() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "updateMany": {
                        "where": {
                            "name": "Shape of You"
                        },
                        "update": {
                            "name": "Shape of You - Radio Edit"
                        }
                    },
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Shape of You - Radio Edit"
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update_many_more() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "updateMany": [
                        {
                            "where": {
                                "name": "Shape of You"
                            },
                            "update": {
                                "name": "Shape of You - Radio Edit"
                            }
                        },
                        {
                            "where": {
                                "name": "Perfect"
                            },
                            "update": {
                                "name": "Perfect - Radio Edit"
                            }
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect - Radio Edit"
                },
                {
                    "id": ignore,
                    "name": "Shape of You - Radio Edit"
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_delete_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "delete": {
                        "name": "Shape of You"
                    }
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_delete_more_than_one() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "delete": [
                        {
                            "name": "Shape of You"
                        },
                        {
                            "name": "Perfect"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": []
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_delete_many() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "deleteMany": {
                        "name": "Shape of You"
                    },
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
            ]
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_delete_many_more() {
        let app = make_app().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "deleteMany": [
                        {
                            "name": "Shape of You"
                        },
                        {
                            "name": "Perfect"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": []
        }))));
    }
}
