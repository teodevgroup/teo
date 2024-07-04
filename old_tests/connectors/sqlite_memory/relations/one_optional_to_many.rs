use test_helpers_async::*;

#[before_all]
#[after_all]
mod test {
    use serial_test::serial;
    use std::sync::Mutex;
    use serde_json::{json};
    use crate::lib::{ExecutionHandle, req};
    use crate::{assert_json, matcher};
    use crate::lib::matcher_functions::one_match;
    use once_cell::sync::Lazy;

    static HANDLE: Lazy<Mutex<ExecutionHandle>> = Lazy::new(|| {
        Mutex::new(ExecutionHandle::new())
    });
    static PORT: i32 = 4050;

    fn before_all() {
        HANDLE.lock().unwrap().execute(file!(), "serve");
    }

    fn after_all() {
        HANDLE.lock().unwrap().exit();
    }

    #[serial]
    #[test]
    fn create_with_nested_create() {
        let _create_res = req(PORT, "create", "Product", json!({
            "create": {
                "name": "Shampoo",
                "category": {
                    "create": {
                        "name": "Toiletries"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 6 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Shampoo",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Toiletries",
            }
        }))));
    }

    #[serial]
    #[test]
    fn create_with_nested_connect() {
        let _create_res = req(PORT, "create", "Product", json!({
            "create": {
                "name": "Shampoo",
                "category": {
                    "connect": {
                        "name": "Cosmetics"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 6 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Shampoo",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Cosmetics",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_create() {
        let _create_res = req(PORT, "update", "Product", json!({
            "where": {
                "name": "Hair Jelly"
            },
            "update": {
                "category": {
                    "create": {
                        "name": "Toiletries"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Hair Jelly",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Toiletries",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_connect() {
        let _create_res = req(PORT, "update", "Product", json!({
            "where": {
                "name": "Hair Jelly"
            },
            "update": {
                "category": {
                    "connect": {
                        "name": "Cosmetics"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Hair Jelly",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Cosmetics",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_set_to_another_one() {
        let _create_res = req(PORT, "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "set": {
                        "name": "Skincares"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Lipstick",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Skincares",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_set_to_null() {
        let _create_res = req(PORT, "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "set": null
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Lipstick",
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_disconnect() {
        let _create_res = req(PORT, "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "disconnect": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Lipstick",
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_update() {
        let _create_res = req(PORT, "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "update": {
                        "name": "Redefined Cosmetics"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Lipstick",
            "categoryId": ignore,
            "category": {
                "id": ignore,
                "name": "Redefined Cosmetics"
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_delete() {
        let _create_res = req(PORT, "update", "Product", json!({
            "where": {
                "name": "Lipstick"
            },
            "update": {
                "category": {
                    "delete": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Product", json!({
            "include": {
                "category": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 5 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Lipstick",
        }))));
    }
}
