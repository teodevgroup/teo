use test_helpers::*;

#[before_all]
#[after_all]
mod test {
    use std::sync::Mutex;
    use serde_json::{json, Value};
    use crate::lib::{ExecutionHandle, req};
    use crate::lib::matcher_functions::{date_time_value, decimal_value};
    use crate::{assert_json, matcher};
    use once_cell::sync::Lazy;

    static HANDLE: Lazy<Mutex<ExecutionHandle>> = Lazy::new(|| {
        Mutex::new(ExecutionHandle::new())
    });
    static PORT: i32 = 4019;

    fn before_all() {
        HANDLE.lock().unwrap().execute(file!(), "serve");
    }

    fn after_all() {
        HANDLE.lock().unwrap().exit();
    }

    #[test]
    fn create() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "string": "vavotitsiangvuntiu",
                "int": 123456,
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "string": "vavotitsiangvuntiu",
                "int": 123456,
            }
        }))
    }

    #[test]
    fn update() {

    }
}
