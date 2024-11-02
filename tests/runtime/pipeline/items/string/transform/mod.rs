#[cfg(test)]
mod tests {
    use std::cell::OnceCell;
    use teo::prelude::App;
    use std::file;
    use hyper::Method;
    use teo::test::schema_path::schema_path_args;
    use serde_json::{json, Value};
    use serial_test::serial;
    use test_helpers_async::*;
    use teo::server::server::Server;
    use teo::server::test_request::TestRequest;
    use crate::{assert_json, matcher};

    static mut SERVER: OnceCell<Server> = OnceCell::new();
    static mut BEFORE_ALL_EXECUTED: bool = false;

    fn server() -> &'static Server {
        unsafe { SERVER.get().unwrap() }
    }

    async fn before_all() {
        if unsafe { BEFORE_ALL_EXECUTED } {
            return;
        }
        unsafe {
            SERVER.get_or_init(|| {
                Server::new(App::new_with_argv(
                    schema_path_args(file!(), "schema.teo")
                ).unwrap())
            })
        };
        server().setup_app_for_unit_test().await.unwrap();
        unsafe { BEFORE_ALL_EXECUTED = true; }
    }

    async fn before_each() {
        server().reset_app_for_unit_test().await.unwrap();
    }

    #[serial]
    #[tokio::test]
    async fn to_word_case() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "toWordCase": "fooBar",
                },
            })).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toWordCase": "foo bar",
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn to_lower_case() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "toLowerCase": "Foo BaR",
                },
            })).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toLowerCase": "foo bar",
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn to_upper_case() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "toUpperCase": "foo bar",
                },
            })).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toUpperCase": "FOO BAR",
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn to_sentence_case() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "toSentenceCase": "fooBar",
                },
            })).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toSentenceCase": "Foo bar",
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn to_title_case() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "toTitleCase": "foo bar",
                },
            })).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toTitleCase": "Foo Bar",
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn trim() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "trim": " abc def\t",
                },
            })).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "trim": "abc def",
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn pad_end() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "padEnd": "123",
                },
            })).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "padEnd": "123__",
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn pad_start() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "padStart": "123",
                },
            })).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "padStart": "__123",
            }
        }))
    }

    #[serial]
    #[tokio::test]
    async fn regex_replace() {
        before_all().await;
        before_each().await;
        let req = TestRequest::new(Method::POST, "/Support/create")
            .json_body(json!({
                "create": {
                    "regexReplace": "foo_bar",
                },
            })).await.unwrap();
        let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "regexReplace": "foo-bar",
            }
        }))
    }
}
