use actix_http::Request;
use actix_web::test::TestRequest;
use serde_json::{json, Value as JsonValue};
use regex::Regex;

pub fn is_object_id(value: &str) -> bool {
    let regex = Regex::new("[\\da-f]{24}").unwrap();
    regex.is_match(value)
}

pub fn request(url: &str, action: &str, body: JsonValue) -> Request {
    let mut new_body = json!({
        "action": action
    });
    new_body.as_object().unwrap().extend(body);
    TestRequest::post().uri(&format!("/{url}/action")).set_json(new_body).to_request()
}