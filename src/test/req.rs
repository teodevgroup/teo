use hyper::Method;
use serde_json::Value;
use teo_result::{Error, Result};
use crate::server::server::Server;
use crate::server::test_request::TestRequest;

pub async fn req(server: &Server, action: &str, model: &str, data: Value) -> Result<Value> {
    let test_request = TestRequest::new(Method::POST, &format!("/{}/{}", model, action))
        .insert_header("Content-Type", "application/json")
        .set_body(match serde_json::to_string(&data) {
            Ok(data) => data,
            Err(_) => Err(Error::internal_server_error_message("cannot serialize json data"))?,
        }).await?;
    let response = server.process_test_request(test_request).await?;
    Ok(response.body_as_json()?)
}
