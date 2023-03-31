use test_helpers::*;

#[before_all]
#[after_all]
mod test {
    use std::sync::Mutex;
    use serde_json::{json};
    use crate::lib::{ExecutionHandle, req};
    
    use crate::{assert_json, matcher};
    use once_cell::sync::Lazy;

    static HANDLE: Lazy<Mutex<ExecutionHandle>> = Lazy::new(|| {
        Mutex::new(ExecutionHandle::new())
    });
    static PORT: i32 = 4015;

    fn before_all() {
        HANDLE.lock().unwrap().execute(file!(), "serve");
    }

    fn after_all() {
        HANDLE.lock().unwrap().exit();
    }

    #[test]
    fn to_word_case() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "toWordCase": "fooBar",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toWordCase": "foo bar",
            }
        }))
    }

    #[test]
    fn to_lower_case() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "toLowerCase": "Foo BaR",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toLowerCase": "foo bar",
            }
        }))
    }

    #[test]
    fn to_upper_case() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "toUpperCase": "foo bar",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toUpperCase": "FOO BAR",
            }
        }))
    }

    #[test]
    fn to_sentence_case() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "toSentenceCase": "fooBar",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toSentenceCase": "Foo bar",
            }
        }))
    }

    #[test]
    fn to_title_case() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "toTitleCase": "foo bar",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toTitleCase": "Foo Bar",
            }
        }))
    }

    #[test]
    fn trim() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "trim": " abc def\t",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "trim": "abc def",
            }
        }))
    }

    #[test]
    fn pad_end() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "padEnd": "123",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "padEnd": "123__",
            }
        }))
    }

    #[test]
    fn pad_start() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "padStart": "123",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "padStart": "__123",
            }
        }))
    }

    #[test]
    fn regex_replace() {
        let res = req(PORT, "create", "Support", json!({
            "create": {
                "regexReplace": "foo_bar",
            },
        }));
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "regexReplace": "foo-bar",
            }
        }))
    }
}
