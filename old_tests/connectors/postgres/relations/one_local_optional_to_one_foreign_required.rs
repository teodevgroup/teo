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
    static PORT: i32 = 4040;

    fn before_all() {
        HANDLE.lock().unwrap().execute(file!(), "serve");
    }

    fn after_all() {
        HANDLE.lock().unwrap().exit();
    }

    #[serial]
    #[test]
    fn create_with_nested_create() {
        let _create_res = req(PORT, "create", "Event", json!({
            "create": {
                "name": "The Enlightenment",
                "note": {
                    "create": {
                        "name": "Note of The Enlightenment"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Event", json!({
            "include": {
                "note": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 4 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "The Enlightenment",
            "noteId": ignore,
            "note": {
                "id": ignore,
                "name": "Note of The Enlightenment",
            }
        }))));
    }

    #[serial]
    #[test]
    fn create_with_nested_connect() {
        let _create_res = req(PORT, "create", "Event", json!({
            "create": {
                "name": "The Enlightenment",
                "note": {
                    "connect": {
                        "name": "Note of The Renaissance"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Event", json!({
            "include": {
                "note": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 4 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "The Enlightenment",
            "noteId": ignore,
            "note": {
                "id": ignore,
                "name": "Note of The Renaissance",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_create() {
        let _update_res = req(PORT, "update", "Event", json!({
            "where": {
                "name": "The Industrial Revolution"
            },
            "update": {
                "note": {
                    "create": {
                        "name": "Note of The Industrial Revolution"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Event", json!({
            "include": {
                "note": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "The Industrial Revolution",
            "noteId": ignore,
            "note": {
                "id": ignore,
                "name": "Note of The Industrial Revolution",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_connect() {
        let _update_res = req(PORT, "update", "Event", json!({
            "where": {
                "name": "The Industrial Revolution"
            },
            "update": {
                "note": {
                    "connect": {
                        "name": "Note of The Renaissance"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Event", json!({
            "include": {
                "note": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "The Industrial Revolution",
            "noteId": ignore,
            "note": {
                "id": ignore,
                "name": "Note of The Renaissance",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_set_to_another_one() {
        let _update_res = req(PORT, "update", "Event", json!({
            "where": {
                "name": "The Industrial Revolution"
            },
            "update": {
                "note": {
                    "set": {
                        "name": "Note of The Renaissance"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Event", json!({
            "include": {
                "note": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "The Industrial Revolution",
            "noteId": ignore,
            "note": {
                "id": ignore,
                "name": "Note of The Renaissance",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_update() {
        let _update_res = req(PORT, "update", "Event", json!({
            "where": {
                "name": "The Renaissance"
            },
            "update": {
                "note": {
                    "create": {
                        "name": "Note of The Renaissance II"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Event", json!({
            "include": {
                "note": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "The Renaissance",
            "noteId": ignore,
            "note": {
                "id": ignore,
                "name": "Note of The Renaissance II",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_delete() {
        let _update_res = req(PORT, "update", "Event", json!({
            "where": {
                "name": "The Renaissance"
            },
            "update": {
                "note": {
                    "delete": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Event", json!({
            "include": {
                "note": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "The Renaissance",
        }))));
    }
}
