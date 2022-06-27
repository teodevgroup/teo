use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::test::{call_service, TestRequest};
use serde_json::{json, Value as JsonValue};
use regex::Regex;

pub fn is_object_id(value: &str) -> bool {
    let regex = Regex::new("[\\da-f]{24}").unwrap();
    regex.is_match(value)
}

pub async fn request<S, B, E>(app: &S, url: &str, action: &str, body: JsonValue) -> S::Response where
    S: Service<Request, Response = ServiceResponse<B>, Error = E>,
    E: std::fmt::Debug,
{
    let mut new_body = json!({
        "action": action
    });
    //new_body.as_object().unwrap().extend(body);
    let req = TestRequest::post().uri(&format!("/{url}/action")).set_json(new_body).to_request();
    call_service(&app, req).await
}

