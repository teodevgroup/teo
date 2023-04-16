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
        let _create_res = req(PORT, "create", "Game", json!({
            "create": {
                "name": "KOFXIII",
                "commandList": {
                    "create": {
                        "name": "KOFXIII Command List"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 4 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOFXIII",
            "commandList": {
                "id": ignore,
                "name": "KOFXIII Command List",
                "gameId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn create_with_nested_connect() {
        let _create_res = req(PORT, "create", "Game", json!({
            "create": {
                "name": "KOFXIII",
                "commandList": {
                    "connect": {
                        "name": "KOF98 Command List"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 4 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOFXIII",
            "commandList": {
                "id": ignore,
                "name": "KOF98 Command List",
                "gameId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_create() {
        let _update_res = req(PORT, "update", "Game", json!({
            "where": {
                "name": "KOFXV"
            },
            "update": {
                "commandList": {
                    "create": {
                        "name": "KOFXV Command List"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOFXV",
            "commandList": {
                "id": ignore,
                "name": "KOFXV Command List",
                "gameId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_connect() {
        let _update_res = req(PORT, "update", "Game", json!({
            "where": {
                "name": "KOFXV"
            },
            "update": {
                "commandList": {
                    "connect": {
                        "name": "KOF97 Command List"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOFXV",
            "commandList": {
                "id": ignore,
                "name": "KOF97 Command List",
                "gameId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_set_to_another_one() {
        let _update_res = req(PORT, "update", "Game", json!({
            "where": {
                "name": "KOFXV"
            },
            "update": {
                "commandList": {
                    "set": {
                        "name": "KOF97 Command List"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOFXV",
            "commandList": {
                "id": ignore,
                "name": "KOF97 Command List",
                "gameId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_update() {
        let _update_res = req(PORT, "update", "Game", json!({
            "where": {
                "name": "KOF98"
            },
            "update": {
                "commandList": {
                    "update": {
                        "name": "KOF98UM Command List"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOF98",
            "commandList": {
                "id": ignore,
                "name": "KOF98UM Command List",
                "gameId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_delete() {
        let _update_res = req(PORT, "update", "Game", json!({
            "where": {
                "name": "KOF98"
            },
            "update": {
                "commandList": {
                    "delete": true
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Game", json!({
            "include": {
                "commandList": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOF98",
        }))));
    }
}
