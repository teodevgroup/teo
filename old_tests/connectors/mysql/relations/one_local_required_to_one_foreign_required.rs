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
        let _create_res = req(PORT, "create", "Profile", json!({
            "create": {
                "name": "Dan",
                "user": {
                    "create": {
                        "name": "Dan"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Profile", json!({
            "include": {
                "user": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Dan",
            "userId": ignore,
            "user": {
                "id": ignore,
                "name": "Dan",
            }
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_update() {
        let _update_res = req(PORT, "update", "Profile", json!({
            "where": {
                "name": "John's profile"
            },
            "update": {
                "user": {
                    "update": {
                        "name": "Class 1"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Profile", json!({
            "include": {
                "user": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "John's profile",
            "userId": ignore,
            "user": {
                "id": ignore,
                "name": "Class 1",
            }
        }))));
    }
}
