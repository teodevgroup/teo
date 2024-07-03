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
        let _create_res = req(PORT, "create", "KOFPlayer", json!({
            "create": {
                "name": "Dan",
                "player": {
                    "create": {
                        "name": "Robert"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Dan",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Robert",
            }
        }))));
    }

    #[serial]
    #[test]
    fn create_with_nested_connect() {
        let _create_res = req(PORT, "create", "KOFPlayer", json!({
            "create": {
                "name": "Dan",
                "player": {
                    "connect": {
                        "name": "Justin Wong"
                    },
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Dan",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Justin Wong",
            },
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_create() {
        let _create_res = req(PORT, "update", "KOFPlayer", json!({
            "where": {
                "name": "Laggia"
            },
            "update": {
                "player": {
                    "create": {
                        "name": "Laggia"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Laggia",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Laggia",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_connect() {
        let _create_res = req(PORT, "update", "KOFPlayer", json!({
            "where": {
                "name": "Laggia"
            },
            "update": {
                "player": {
                    "connect": {
                        "name": "Justin Wong"
                    },
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Laggia",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Justin Wong",
            },
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_set_to_another_one() {
        let _create_res = req(PORT, "update", "KOFPlayer", json!({
            "where": {
                "name": "Laggia"
            },
            "update": {
                "player": {
                    "set": {
                        "name": "Justin Wong"
                    },
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Laggia",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Justin Wong",
            },
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_set_to_null() {
        let _create_res = req(PORT, "update", "KOFPlayer", json!({
            "where": {
                "name": "Laggia"
            },
            "update": {
                "player": {
                    "set": null
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Laggia",
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Justin Wong",
            },
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_disconnect() {
        let _create_res = req(PORT, "update", "KOFPlayer", json!({
            "where": {
                "name": "Laggia"
            },
            "update": {
                "player": {
                    "disconnect": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Laggia",
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Justin Wong",
            },
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_update() {
        let _create_res = req(PORT, "update", "KOFPlayer", json!({
            "where": {
                "name": "Justin Wong plays KOF"
            },
            "update": {
                "player": {
                    "update": {
                        "name": "Justin Wong is Teochew"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
            "playerId": ignore,
            "player": {
                "id": ignore,
                "name": "Justin Wong is Teochew",
            },
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_delete() {
        let _create_res = req(PORT, "update", "KOFPlayer", json!({
            "where": {
                "name": "Justin Wong plays KOF"
            },
            "update": {
                "player": {
                    "delete": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "KOFPlayer", json!({
            "include": {
                "player": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Justin Wong plays KOF",
        }))));
    }
}
