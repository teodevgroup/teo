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
        let _create_res = req(PORT, "create", "Player", json!({
            "create": {
                "name": "Dan",
                "kof": {
                    "create": {
                        "name": "Robert"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Dan",
            "kof": {
                "id": ignore,
                "name": "Robert",
                "playerId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn create_with_nested_connect() {
        let _create_res = req(PORT, "create", "Player", json!({
            "create": {
                "name": "Dan",
                "kof": {
                    "connect": {
                        "name": "Justin Wong plays KOF"
                    },
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Dan",
            "kof": {
                "id": ignore,
                "name": "Justin Wong plays KOF",
                "playerId": ignore,
            },
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_create() {
        let _update_res = req(PORT, "update", "Player", json!({
            "where": {
                "name": "Justin Wong"
            },
            "update": {
                "kof": {
                    "create": {
                        "name": "Ash Crimson"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong",
            "kof": {
                "id": ignore,
                "name": "Ash Crimson",
                "playerId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_connect() {
        let _update_res = req(PORT, "update", "Player", json!({
            "where": {
                "name": "Justin Wong"
            },
            "update": {
                "kof": {
                    "connect": {
                        "name": "Laggia"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong",
            "kof": {
                "id": ignore,
                "name": "Laggia",
                "playerId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_set_to_another_one() {
        let _update_res = req(PORT, "update", "Player", json!({
            "where": {
                "name": "Justin Wong"
            },
            "update": {
                "kof": {
                    "set": {
                        "name": "Laggia"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong",
            "kof": {
                "id": ignore,
                "name": "Laggia",
                "playerId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_set_to_null() {
        let _update_res = req(PORT, "update", "Player", json!({
            "where": {
                "name": "Justin Wong"
            },
            "update": {
                "kof": {
                    "set": null
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong",
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_disconnect() {
        let _update_res = req(PORT, "update", "Player", json!({
            "where": {
                "name": "Justin Wong"
            },
            "update": {
                "kof": {
                    "disconnect": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong",
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_update() {
        let _update_res = req(PORT, "update", "Player", json!({
            "where": {
                "name": "Justin Wong"
            },
            "update": {
                "kof": {
                    "update": {
                        "name": "Ash Crimson"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong",
            "kof": {
                "id": ignore,
                "name": "Ash Crimson",
                "playerId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_delete() {
        let _update_res = req(PORT, "update", "Player", json!({
            "where": {
                "name": "Justin Wong"
            },
            "update": {
                "kof": {
                    "delete": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Player", json!({
            "include": {
                "kof": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong",
        }))));
    }
}
