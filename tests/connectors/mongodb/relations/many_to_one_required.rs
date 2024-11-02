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
    #[tokio::test]
    async fn create_with_nested_create_many() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn create_with_nested_connect_one() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn create_with_nested_connect_more_than_one() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_create_one() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_create_many() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_connect_one() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_connect_more_than_one() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_update_one() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_update_more_than_one() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_update_many() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_update_many_more() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_delete_one() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_delete_more_than_one() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_delete_many() {
        before_all().await;
        before_each().await;
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
    #[tokio::test]
    async fn update_with_nested_delete_many_more() {
        before_all().await;
        before_each().await;
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
