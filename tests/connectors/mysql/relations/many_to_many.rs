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
    use test_helpers_async::after_each;
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
    async fn create_with_nested_create_one() {
        before_all().await;
        before_each().await;
        let _create_res = req(&app, "create", "Artist", json!({
            "create": {
                "name": "Taylor Swift",
                "songs": {
                    "create": {
                        "name": "Love Story"
                    }
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        })).await;
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
    #[tokio::test]
    async fn create_with_nested_create_many() {
        before_all().await;
        before_each().await;
        let _create_res = req(&app, "create", "Artist", json!({
            "create": {
                "name": "Taylor Swift",
                "songs": {
                    "create": [
                        {
                            "name": "Love Story"
                        },
                        {
                            "name": "Red"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
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
    #[tokio::test]
    async fn create_with_nested_connect_one() {
        before_all().await;
        before_each().await;
        let _create_res = req(&app, "create", "Artist", json!({
            "create": {
                "name": "Taylor Swift",
                "songs": {
                    "connect": {
                        "name": "Perfect"
                    },
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
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
    #[tokio::test]
    async fn create_with_nested_connect_more_than_one() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
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
    #[tokio::test]
    async fn update_with_nested_create_one() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
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
    #[tokio::test]
    async fn update_with_nested_create_many() {
        before_all().await;
        before_each().await;
        let _update_res = req(&app, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "create": [
                        {
                            "name": "Photograph"
                        },
                        {
                            "name": "Castle on the Hill"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
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
    #[tokio::test]
    async fn update_with_nested_connect_one() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
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
    #[tokio::test]
    async fn update_with_nested_connect_more_than_one() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
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
    #[tokio::test]
    async fn update_with_nested_set() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
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
    #[tokio::test]
    async fn update_with_nested_disconnect_one() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        })).await;
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
    #[tokio::test]
    async fn update_with_nested_disconnect_more_than_one() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": []
        }))));
    }

    #[serial]
    #[tokio::test]
    async fn update_with_nested_update_one() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
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
    #[tokio::test]
    async fn update_with_nested_update_more_than_one() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
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
    #[tokio::test]
    async fn update_with_nested_update_many() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
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
    #[tokio::test]
    async fn update_with_nested_update_many_more() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
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
    #[tokio::test]
    async fn update_with_nested_delete_one() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        })).await;
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
    #[tokio::test]
    async fn update_with_nested_delete_more_than_one() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": []
        }))));
    }

    #[serial]
    #[tokio::test]
    async fn update_with_nested_delete_many() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        })).await;
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
    #[tokio::test]
    async fn update_with_nested_delete_many_more() {
        before_all().await;
        before_each().await;
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
        })).await;
        let find_many_res = req(&app, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": []
        }))));
    }
}
