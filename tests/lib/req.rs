use actix_http::body::MessageBody;
use actix_http::Method;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::test;
use serde_json::Value;

pub(crate) async fn req(app: &impl Service<
    actix_http::Request,
    Response = ServiceResponse<impl MessageBody>,
    Error = actix_web::Error,
>, action: &str, model: &str, data: Value) -> Value {
    let req = test::TestRequest::default()
        .method(Method::POST)
        .uri(&format!("/{}/{}", model, action))
        .set_json(data)
        .to_request();
    let res: Value = test::call_and_read_body_json(app, req).await;
    return res
}
