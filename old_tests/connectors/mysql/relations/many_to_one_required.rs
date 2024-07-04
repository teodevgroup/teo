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
    static PORT: i32 = 4030;

    fn before_all() {
        HANDLE.lock().unwrap().execute(file!(), "serve");
    }

    fn after_all() {
        HANDLE.lock().unwrap().exit();
    }

    #[serial]
    #[test]
    fn create_with_nested_create() {
        let _create_res = req(PORT, "create", "Author", json!({
            "create": {
                "name": "Jack",
                "posts": {
                    "create": {
                        "name": "PyPy"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": true
            }
        }));
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
    #[test]
    fn create_with_nested_create_many() {
        let _create_res = req(PORT, "create", "Author", json!({
            "create": {
                "name": "Jack",
                "posts": {
                    "createMany": [
                        {
                            "name": "PyPy"
                        },
                        {
                            "name": "NoNo"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "desc"
                    }
                }
            }
        }));
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
    #[test]
    fn create_with_nested_connect_one() {
        let _create_res = req(PORT, "create", "Author", json!({
            "create": {
                "name": "Jack",
                "posts": {
                    "connect": {
                        "name": "Swift 3.0"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": true
            }
        }));
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
    #[test]
    fn create_with_nested_connect_more_than_one() {
        let _create_res = req(PORT, "create", "Author", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
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
    #[test]
    fn update_with_nested_create_one() {
        let _update_res = req(PORT, "update", "Author", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
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
    #[test]
    fn update_with_nested_create_many() {
        let _update_res = req(PORT, "update", "Author", json!({
            "where": {
                "name": "Paul"
            },
            "update": {
                "posts": {
                    "createMany": [
                        {
                            "name": "Swift 4.0"
                        },
                        {
                            "name": "Swift 5.0"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
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
    #[test]
    fn update_with_nested_connect_one() {
        let _update_res = req(PORT, "update", "Author", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
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
    #[test]
    fn update_with_nested_connect_more_than_one() {
        let _update_res = req(PORT, "update", "Author", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
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
    #[test]
    fn update_with_nested_update_one() {
        let _update_res = req(PORT, "update", "Author", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
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
    #[test]
    fn update_with_nested_update_more_than_one() {
        let _update_res = req(PORT, "update", "Author", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
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
    #[test]
    fn update_with_nested_update_many() {
        let _update_res = req(PORT, "update", "Author", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
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
    #[test]
    fn update_with_nested_update_many_more() {
        let _update_res = req(PORT, "update", "Author", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
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
    #[test]
    fn update_with_nested_delete_one() {
        let _update_res = req(PORT, "update", "Author", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
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
    #[test]
    fn update_with_nested_delete_more_than_one() {
        let _update_res = req(PORT, "update", "Author", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
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
    #[test]
    fn update_with_nested_delete_many() {
        let _update_res = req(PORT, "update", "Author", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
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
    #[test]
    fn update_with_nested_delete_many_more() {
        let _update_res = req(PORT, "update", "Author", json!({
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
        }));
        let find_many_res = req(PORT, "findMany", "Author", json!({
            "include": {
                "posts": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
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
