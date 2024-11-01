// use hyper::body::Incoming;
// use hyper::Method;
// use serde_json::Value;
// use teo_result::Result;
// use crate::server::server::Server;
//
// pub async fn req(server: &Server, action: &str, model: &str, data: Value) -> Result<Value> {
//     let request = hyper::Request::builder()
//         .method(Method::POST)
//         .uri(format!("/{}/{}", model, action)).body(Incoming::from(serde_json::to_string(&data)?))?;
//     let response = server.process_test_request(request).await?;
//     Ok(response.body().as_teon().unwrap().clone())
// }
