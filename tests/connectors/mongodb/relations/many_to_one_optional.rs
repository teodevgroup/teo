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
        let _create_res = req(server(), "create", "Category", json!({
            "create": {
                "name": "Toiletries",
                "products": {
                    "create": {
                        "name": "Shampoo"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Toiletries",
            "products": [
                {
                    "id": ignore,
                    "name": "Shampoo",
                    "categoryId": ignore,
                }
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn create_with_nested_create_many() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "create", "Category", json!({
            "create": {
                "name": "Toiletries",
                "products": {
                    "create": [
                        {
                            "name": "Shampoo"
                        },
                        {
                            "name": "Shower Gel"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Toiletries",
            "products": [
                {
                    "id": ignore,
                    "name": "Shampoo",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Shower Gel",
                    "categoryId": ignore,
                }
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn create_with_nested_connect_one() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "create", "Category", json!({
            "create": {
                "name": "Toiletries",
                "products": {
                    "connect": {
                        "name": "Hair Jelly"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Toiletries",
            "products": [
                {
                    "id": ignore,
                    "name": "Hair Jelly",
                    "categoryId": ignore,
                }
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn create_with_nested_connect_more_than_one() {
        before_all().await;
        before_each().await;
        let _create_res = req(server(), "create", "Category", json!({
            "create": {
                "name": "Toiletries",
                "products": {
                    "connect": [
                        {
                            "name": "Hair Jelly"
                        },
                        {
                            "name": "Lipstick"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Toiletries",
            "products": [
                {
                    "id": ignore,
                    "name": "Hair Jelly",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Lipstick",
                    "categoryId": ignore,
                }
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_create_one() {
        before_all().await;
        before_each().await;

        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Skincares"
            },
            "update": {
                "products": {
                    "create": {
                        "name": "Eye Cream"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Skincares",
            "products": [
                {
                    "id": ignore,
                    "name": "Eye Cream",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Lipid Restore",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Sérum",
                    "categoryId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_create_many() {
        before_all().await;
        before_each().await;

        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Skincares"
            },
            "update": {
                "products": {
                    "create": [
                        {
                            "name": "Eye Cream"
                        },
                        {
                            "name": "Concentrate"
                        },
                    ]
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Skincares",
            "products": [
                {
                    "id": ignore,
                    "name": "Concentrate",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Eye Cream",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Lipid Restore",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Sérum",
                    "categoryId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_connect_one() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Skincares"
            },
            "update": {
                "products": {
                    "connect": {
                        "name": "Lipstick"
                    },
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Skincares",
            "products": [
                {
                    "id": ignore,
                    "name": "Lipid Restore",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Lipstick",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Sérum",
                    "categoryId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_connect_more_than_one() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Skincares"
            },
            "update": {
                "products": {
                    "connect": [
                        {
                            "name": "Lipstick"
                        },
                        {
                            "name": "Nail Polish"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Skincares",
            "products": [
                {
                    "id": ignore,
                    "name": "Lipid Restore",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Lipstick",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Nail Polish",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Sérum",
                    "categoryId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_set() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Skincares"
            },
            "update": {
                "products": {
                    "set": [
                        {
                            "name": "Lipstick"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Skincares",
            "products": [
                {
                    "id": ignore,
                    "name": "Lipstick",
                    "categoryId": ignore,
                },
            ]
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Cosmetics",
            "products": [
                {
                    "id": ignore,
                    "name": "Nail Polish",
                    "categoryId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_disconnect_one() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Cosmetics"
            },
            "update": {
                "products": {
                    "disconnect": {
                        "name": "Nail Polish"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Cosmetics",
            "products": [
                {
                    "id": ignore,
                    "name": "Lipstick",
                    "categoryId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_disconnect_more_than_one() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Cosmetics"
            },
            "update": {
                "products": {
                    "disconnect": [
                        {
                            "name": "Nail Polish"
                        },
                        {
                            "name": "Lipstick"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Cosmetics",
            "products": []
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_update_one() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Cosmetics"
            },
            "update": {
                "products": {
                    "update": {
                        "where": {
                            "name": "Lipstick"
                        },
                        "update": {
                            "name": "Chanel Lipstick"
                        }
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Cosmetics",
            "products": [
                {
                    "id": ignore,
                    "name": "Chanel Lipstick",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Nail Polish",
                    "categoryId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_update_more_than_one() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Cosmetics"
            },
            "update": {
                "products": {
                    "update": [
                        {
                            "where": {
                                "name": "Lipstick"
                            },
                            "update": {
                                "name": "Dior Lipstick"
                            }
                        },
                        {
                            "where": {
                                "name": "Nail Polish"
                            },
                            "update": {
                                "name": "Armani Lip Gloss"
                            }
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Cosmetics",
            "products": [
                {
                    "id": ignore,
                    "name": "Armani Lip Gloss",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Dior Lipstick",
                    "categoryId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_update_many() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Cosmetics"
            },
            "update": {
                "products": {
                    "updateMany": {
                        "where": {
                            "name": "Lipstick"
                        },
                        "update": {
                            "name": "Chanel Lipstick"
                        }
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Cosmetics",
            "products": [
                {
                    "id": ignore,
                    "name": "Chanel Lipstick",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Nail Polish",
                    "categoryId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_update_many_more() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Cosmetics"
            },
            "update": {
                "products": {
                    "updateMany": [
                        {
                            "where": {
                                "name": "Lipstick"
                            },
                            "update": {
                                "name": "Dior Lipstick"
                            }
                        },
                        {
                            "where": {
                                "name": "Nail Polish"
                            },
                            "update": {
                                "name": "Armani Lip Gloss"
                            }
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Cosmetics",
            "products": [
                {
                    "id": ignore,
                    "name": "Armani Lip Gloss",
                    "categoryId": ignore,
                },
                {
                    "id": ignore,
                    "name": "Dior Lipstick",
                    "categoryId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_delete_one() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Cosmetics"
            },
            "update": {
                "products": {
                    "delete": {
                        "name": "Lipstick"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Cosmetics",
            "products": [
                {
                    "id": ignore,
                    "name": "Nail Polish",
                    "categoryId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_delete_more_than_one() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Cosmetics"
            },
            "update": {
                "products": {
                    "delete": [
                        {
                            "name": "Lipstick"
                        },
                        {
                            "name": "Nail Polish"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Cosmetics",
            "products": []
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_delete_many() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Cosmetics"
            },
            "update": {
                "products": {
                    "deleteMany": {
                        "name": "Lipstick"
                    }
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": true
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Cosmetics",
            "products": [
                {
                    "id": ignore,
                    "name": "Nail Polish",
                    "categoryId": ignore,
                },
            ]
        }))));
    }

    #[serial]
    #[shared_tokio_runtime::runtime_test]
    async fn update_with_nested_delete_many_more() {
        before_all().await;
        before_each().await;
        let _update_res = req(server(), "update", "Category", json!({
            "where": {
                "name": "Cosmetics"
            },
            "update": {
                "products": {
                    "deleteMany": [
                        {
                            "name": "Lipstick"
                        },
                        {
                            "name": "Nail Polish"
                        }
                    ]
                }
            },
        })).await;
        let find_many_res = req(server(), "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        })).await;
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_matches(matcher!({
            "id": ignore,
            "name": "Cosmetics",
            "products": []
        }))));
    }
}
