
#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test};
    use crate::lib::server::make_actix_app;
    use teo::prelude::App;
    use std::file;
    use crate::lib::schema_path::schema_path_args;

    #[actix_web::test]
    async fn test_index_get() {
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
}