use test_helpers::*;

#[before_all]
#[after_all]
mod test {
    use std::sync::Mutex;
    use serde_json::{json, Value};
    use crate::lib::{ExecutionHandle, req};
    use crate::{assert_json, matcher};
    use crate::lib::matcher_functions::{date_time_value, decimal_value};
    use once_cell::sync::Lazy;

    static HANDLE: Lazy<Mutex<ExecutionHandle>> = Lazy::new(|| {
        Mutex::new(ExecutionHandle::new())
    });
    static PORT: i32 = 4013;

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

    #[test]
    fn date() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "date": "2005-12-25",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "date": "2005-12-25",
            }
        }))
    }

    #[test]
    fn date_time() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "dateTime": "2003-04-17T08:12:34.567Z",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "dateTime": date_time_value("2003-04-17T08:12:34.567Z"),
            }
        }))
    }

    #[test]
    fn decimal() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "decimal": "5.78",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "decimal": decimal_value("5.78"),
            }
        }))
    }

    #[test]
    fn r#enum() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "sex": "FEMALE",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "sex": "FEMALE",
            }
        }))
    }

    #[test]
    fn int32_array() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "int32Array": [1, 2, 3],
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "int32Array": [1, 2, 3],
            }
        }))
    }

    #[test]
    fn date_time_array() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "dateTimeArray": ["2003-04-17T08:12:34.567Z", "1997-10-19T08:12:34.567Z"],
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "dateTimeArray": [date_time_value("2003-04-17T08:12:34.567Z"), date_time_value("1997-10-19T08:12:34.567Z")],
            }
        }))
    }
}
