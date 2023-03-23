use test_helpers::*;

#[before_all]
#[after_all]
mod test {
    use std::sync::Mutex;
    use serde_json::{json, Value};
    use crate::lib::{ExecutionHandle, req};
    use crate::{assert_json, matcher};
    use once_cell::sync::Lazy;

    static HANDLE: Lazy<Mutex<ExecutionHandle>> = Lazy::new(|| {
        Mutex::new(ExecutionHandle::new())
    });
    static PORT: i32 = 4012;

    fn before_all() {
        HANDLE.lock().unwrap().execute(file!(), "serve");
    }

    fn after_all() {
        HANDLE.lock().unwrap().exit();
    }

    #[test]
    fn int32() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "int32": 1,
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "int32": 1,
            }
        }))
    }

    #[test]
    fn int64() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "int64": 1,
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "int64": 1,
            }
        }))
    }

    #[test]
    fn float32() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "float32": 1.5,
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "float32": 1.5,
            }
        }))
    }

    #[test]
    fn float64() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "float64": 1.2,
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "float64": 1.2,
            }
        }))
    }

    #[test]
    fn bool() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "bool": true,
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "bool": true,
            }
        }))
    }

    #[test]
    fn string() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "string": "KOF XV",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "string": "KOF XV",
            }
        }))
    }
}
