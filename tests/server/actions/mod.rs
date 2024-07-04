use test_helpers_async::*;

#[cfg(test)]
#[before_all]
#[after_all_async]
mod tests {
    use actix_web::{http::header::ContentType, test};
    use crate::lib::server::make_actix_app;
    use teo::prelude::App;
    use std::file;
    use actix_http::Method;
    use crate::lib::schema_path::schema_path_args;
    use serde_json::{json, Value};
    use test_helpers_async::*;
    use crate::{assert_json, matcher};



    fn before_all() {
        println!("before all");
    }

    fn after_all_async() {
        println!("after all");
    }

    #[actix_web::test]
    async fn test_get_index() {
        let teo_app = App::new_with_argv(
            schema_path_args(file!(), "schema.teo")
        ).unwrap();
        let app = test::init_service(
            make_actix_app(
                &teo_app
            ).await.unwrap()
        ).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().as_u16() == 404);
    }

    #[actix_web::test]
    async fn test_create_record() {
        let teo_app = App::new_with_argv(
            schema_path_args(file!(), "schema.teo")
        ).unwrap();
        let app = test::init_service(
            make_actix_app(
                &teo_app
            ).await.unwrap()
        ).await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/Support/create")
            .set_json(json!({
                "create": {
                    "string": "lulua",
                    "int": 123456,
                },
            }))
            .to_request();
        let res_body: Value = test::call_and_read_body_json(&app, req).await;
        assert_json!(res_body, matcher!({
            "data": {
                "id": ignore,
                "string": "lulua",
                "int": 123456,
            }
        }));
    }
}
