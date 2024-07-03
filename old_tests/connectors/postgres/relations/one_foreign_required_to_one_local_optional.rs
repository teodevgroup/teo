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
        let _create_res = req(PORT, "create", "Note", json!({
            "create": {
                "name": "Note of The Russian Revolutions",
                "event": {
                    "create": {
                        "name": "The Russian Revolutions"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Note", json!({
            "include": {
                "event": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Note of The Russian Revolutions",
            "event": {
                "id": ignore,
                "name": "The Russian Revolutions",
                "noteId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn create_with_nested_connect() {
        let _create_res = req(PORT, "create", "Note", json!({
            "create": {
                "name": "Note of The Russian Revolutions",
                "event": {
                    "connect": {
                        "name": "The Renaissance"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Note", json!({
            "include": {
                "event": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Note of The Russian Revolutions",
            "event": {
                "id": ignore,
                "name": "The Renaissance",
                "noteId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_create() {
        let _update_res = req(PORT, "update", "Note", json!({
            "where": {
                "name": "Note of The Renaissance",
            },
            "update": {
                "event": {
                    "create": {
                        "name": "Understanding The Renaissance"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Note", json!({
            "include": {
                "event": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Note of The Renaissance",
            "event": {
                "id": ignore,
                "name": "Understanding The Renaissance",
                "noteId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_connect() {
        let _update_res = req(PORT, "update", "Note", json!({
            "where": {
                "name": "Note of The Renaissance",
            },
            "update": {
                "event": {
                    "connect": {
                        "name": "The French Revolution"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Note", json!({
            "include": {
                "event": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Note of The Renaissance",
            "event": {
                "id": ignore,
                "name": "The French Revolution",
                "noteId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_set_to_another_one() {
        let _update_res = req(PORT, "update", "Note", json!({
            "where": {
                "name": "Note of The Renaissance",
            },
            "update": {
                "event": {
                    "set": {
                        "name": "The French Revolution"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Note", json!({
            "include": {
                "event": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Note of The Renaissance",
            "event": {
                "id": ignore,
                "name": "The French Revolution",
                "noteId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_update() {
        let _update_res = req(PORT, "update", "Note", json!({
            "where": {
                "name": "Note of The Renaissance",
            },
            "update": {
                "event": {
                    "update": {
                        "name": "Memorize The French Revolution"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Note", json!({
            "include": {
                "event": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Note of The Renaissance",
            "event": {
                "id": ignore,
                "name": "Memorize The French Revolution",
                "noteId": ignore,
            }
        }))));
    }
}
