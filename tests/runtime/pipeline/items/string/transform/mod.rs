#[cfg(test)]
mod tests {
    use std::cell::OnceCell;
    use actix_web::{http::header::ContentType, test};
    use teo::test::server::make_actix_app;
    use teo::prelude::App;
    use std::file;
    use actix_http::body::MessageBody;
    use actix_http::Method;
    use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
    use teo::test::schema_path::schema_path_args;
    use serde_json::{json, Value};
    use serial_test::serial;
    use test_helpers_async::*;
    use crate::{assert_json, matcher};
    use teo::test::handle::Handle;

    static mut HANDLE: OnceCell<Handle> = OnceCell::new();

    async fn make_app() -> impl Service<
        actix_http::Request,
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
    > {
        unsafe {
            let teo_app = HANDLE.get_or_init(|| {
                let mut h = Handle::new();
                h.load(|| {
                    App::new_with_argv(
                        schema_path_args(file!(), "schema.teo")
                    ).unwrap()
                });
                h
            }).teo_app();
            test::init_service(
                make_actix_app(
                    &teo_app
                ).await.unwrap()
            ).await
        }
    }

    #[serial]
    #[actix_web::test]
    async fn to_word_case() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "toWordCase": "fooBar",
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toWordCase": "foo bar",
            }
        }))
    }

    #[serial]
    #[actix_web::test]
    async fn to_lower_case() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "toLowerCase": "Foo BaR",
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toLowerCase": "foo bar",
            }
        }))
    }

    #[serial]
    #[actix_web::test]
    async fn to_upper_case() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "toUpperCase": "foo bar",
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toUpperCase": "FOO BAR",
            }
        }))
    }

    #[serial]
    #[actix_web::test]
    async fn to_sentence_case() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "toSentenceCase": "fooBar",
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toSentenceCase": "Foo bar",
            }
        }))
    }

    #[serial]
    #[actix_web::test]
    async fn to_title_case() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "toTitleCase": "foo bar",
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "toTitleCase": "Foo Bar",
            }
        }))
    }

    #[serial]
    #[actix_web::test]
    async fn trim() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "trim": " abc def\t",
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "trim": "abc def",
            }
        }))
    }

    #[serial]
    #[actix_web::test]
    async fn pad_end() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "padEnd": "123",
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "padEnd": "123__",
            }
        }))
    }

    #[serial]
    #[actix_web::test]
    async fn pad_start() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "padStart": "123",
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "padStart": "__123",
            }
        }))
    }

    #[serial]
    #[actix_web::test]
    async fn regex_replace() {
        let app = make_app().await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "regexReplace": "foo_bar",
                },
            }))
            .to_request();
        let res: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res, matcher!({
            "data": {
                "id": ignore,
                "regexReplace": "foo-bar",
            }
        }))
    }
}
