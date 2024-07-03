#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};

    #[actix_web::test]
    async fn test_index_get() {
        // let app = test::init_service(App::new().service(index)).await;
        // let req = test::TestRequest::default()
        //     .insert_header(ContentType::plaintext())
        //     .to_request();
        // let resp = test::call_service(&app, req).await;
        // assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_index_post() {
        // let app = test::init_service(App::new().service(index)).await;
        // let req = test::TestRequest::post().uri("/").to_request();
        // let resp = test::call_service(&app, req).await;
        // assert!(resp.status().is_client_error());
    }
}