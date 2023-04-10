use test_helpers::*;

#[before_all]
#[after_all]
mod test {
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

    #[test]
    fn create_with_nested_create_one() {
        let _create_res = req(PORT, "create", "Artist", json!({
            "create": {
                "name": "Taylor Swift",
                "songs": {
                    "create": {
                        "name": "Love Story"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Taylor Swift",
            "songs": [
                {
                    "id": ignore,
                    "name": "Love Story"
                }
            ]
        }))));
    }

}
