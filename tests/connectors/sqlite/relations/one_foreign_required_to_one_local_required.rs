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
        let _create_res = req(PORT, "create", "User", json!({
            "create": {
                "name": "Dan",
                "profile": {
                    "create": {
                        "name": "Dan"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "User", json!({
            "include": {
                "profile": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Dan",
            "profile": {
                "id": ignore,
                "name": "Dan",
                "userId": ignore,
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_update() {
        let update_res = req(PORT, "update", "User", json!({
            "where": {
                "name": "John"
            },
            "update": {
                "profile": {
                    "update": {
                        "name": "Class 1"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "User", json!({
            "include": {
                "profile": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "John",
            "profile": {
                "id": ignore,
                "name": "Class 1",
                "userId": ignore,
            }
        }))));
    }
}
