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
        let _create_res = req(PORT, "create", "Category", json!({
            "create": {
                "name": "Toiletries",
                "products": {
                    "create": {
                        "name": "Shampoo"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn create_with_nested_create_many() {
        let _create_res = req(PORT, "create", "Category", json!({
            "create": {
                "name": "Toiletries",
                "products": {
                    "createMany": [
                        {
                            "name": "Shampoo"
                        },
                        {
                            "name": "Shower Gel"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn create_with_nested_connect_one() {
        let _create_res = req(PORT, "create", "Category", json!({
            "create": {
                "name": "Toiletries",
                "products": {
                    "connect": {
                        "name": "Hair Jelly"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn create_with_nested_connect_more_than_one() {
        let _create_res = req(PORT, "create", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_create_one() {
        let _update_res = req(PORT, "update", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_create_many() {
        let _update_res = req(PORT, "update", "Category", json!({
            "where": {
                "name": "Skincares"
            },
            "update": {
                "products": {
                    "createMany": [
                        {
                            "name": "Eye Cream"
                        },
                        {
                            "name": "Concentrate"
                        },
                    ]
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_connect_one() {
        let _update_res = req(PORT, "update", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_connect_more_than_one() {
        let _update_res = req(PORT, "update", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_set() {
        let _update_res = req(PORT, "update", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_disconnect_one() {
        let _update_res = req(PORT, "update", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_disconnect_more_than_one() {
        let _update_res = req(PORT, "update", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Cosmetics",
            "products": []
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_update_one() {
        let _update_res = req(PORT, "update", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_update_more_than_one() {
        let _update_res = req(PORT, "update", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_update_many() {
        let _update_res = req(PORT, "update", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_update_many_more() {
        let _update_res = req(PORT, "update", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_delete_one() {
        let _update_res = req(PORT, "update", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_delete_more_than_one() {
        let _update_res = req(PORT, "update", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Cosmetics",
            "products": []
        }))));
    }

    #[serial]
    #[actix_web::test]
    async fn update_with_nested_delete_many() {
        let _update_res = req(PORT, "update", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
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
    #[actix_web::test]
    async fn update_with_nested_delete_many_more() {
        let _update_res = req(PORT, "update", "Category", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Category", json!({
            "include": {
                "products": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Cosmetics",
            "products": []
        }))));
    }
}
