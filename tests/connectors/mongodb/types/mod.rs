use test_helpers::*;

#[before_all]
#[after_all]
mod test {
    use std::sync::Mutex;
    use serde_json::{json, Value};
    use crate::lib::{ExecutionHandle, req};
    use crate::matcher;
    use once_cell::sync::Lazy;

    static HANDLE: Lazy<Mutex<ExecutionHandle>> = Lazy::new(|| {
        Mutex::new(ExecutionHandle::new())
    });
    static PORT: i32 = 4010;

    fn before_all() {
        HANDLE.lock().unwrap().execute(file!(), "serve");
    }

    fn after_all() {
        HANDLE.lock().unwrap().exit();
    }

    #[test]
    fn int32() {
        // let res = req(PORT, "create", "Support", json!({
        //     "create": {
        //         "int32": 1,
        //     },
        // }));
        let m = matcher!({
            "data": {
                "id": ignore,
                "int32": 1,
            }
        });
    }

    #[test]
    fn int64() {

    }

    #[test]
    fn float32() {

    }
}
