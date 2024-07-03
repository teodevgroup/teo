use test_helpers::*;

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
    static PORT: i32 = 4020;

    fn before_all() {
        HANDLE.lock().unwrap().execute(file!(), "serve");
    }

    fn after_all() {
        HANDLE.lock().unwrap().exit();
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
