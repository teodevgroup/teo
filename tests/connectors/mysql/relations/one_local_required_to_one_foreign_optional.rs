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
        let _create_res = req(PORT, "create", "CommandList", json!({
            "create": {
                "name": "KOFXIV Command List",
                "game": {
                    "create": {
                        "name": "KOFXIV"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOFXIV Command List",
            "gameId": ignore,
            "game": {
                "id": ignore,
                "name": "KOFXIV",
            }
        }))));
    }

    #[serial]
    #[test]
    fn create_with_nested_connect() {
        let _create_res = req(PORT, "create", "CommandList", json!({
            "create": {
                "name": "KOFXIV Command List",
                "game": {
                    "connect": {
                        "name": "KOFXV"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOFXIV Command List",
            "gameId": ignore,
            "game": {
                "id": ignore,
                "name": "KOFXV",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_create() {
        let _update_res = req(PORT, "update", "CommandList", json!({
            "where": {
                "name": "KOF97 Command List"
            },
            "update": {
                "game": {
                    "create": {
                        "name": "KOF96"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOF97 Command List",
            "gameId": ignore,
            "game": {
                "id": ignore,
                "name": "KOF96",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_connect() {
        let _update_res = req(PORT, "update", "CommandList", json!({
            "where": {
                "name": "KOF97 Command List"
            },
            "update": {
                "game": {
                    "connect": {
                        "name": "KOFXV"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOF97 Command List",
            "gameId": ignore,
            "game": {
                "id": ignore,
                "name": "KOFXV",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_set_to_another_one() {
        let _update_res = req(PORT, "update", "CommandList", json!({
            "where": {
                "name": "KOF97 Command List"
            },
            "update": {
                "game": {
                    "set": {
                        "name": "KOFXV"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOF97 Command List",
            "gameId": ignore,
            "game": {
                "id": ignore,
                "name": "KOFXV",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_update() {
        let _update_res = req(PORT, "update", "CommandList", json!({
            "where": {
                "name": "KOF97 Command List"
            },
            "update": {
                "game": {
                    "update": {
                        "name": "KOF95"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "CommandList", json!({
            "include": {
                "game": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "KOF97 Command List",
            "gameId": ignore,
            "game": {
                "id": ignore,
                "name": "KOF95",
            }
        }))));
    }
}
