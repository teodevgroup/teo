use hyper::Method;
use serde_json::Value;
use teo_result::Error;
use crate::server::server::Server;
use crate::server::test_request::TestRequest;

pub async fn req(server: &Server, action: &str, model: &str, data: Value) -> Value {
    let test_request = TestRequest::new(Method::POST, &format!("/{}/{}", model, action))
        .insert_header("Content-Type", "application/json")
        .set_body(serde_json::to_string(&data).unwrap_or_else(|_| Err(Error::internal_server_error_message("cannot serialize json data")).unwrap())).await.unwrap();
    let response = server.process_test_request(test_request).await.unwrap();
    response.body_as_json().unwrap()
}
