use std::str::FromStr;
use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::http::StatusCode;
use teo_runtime::response::body::BodyInner;
use teo_runtime::response::Response;

pub trait IntoHttpResponse {
    fn into_http_response(self, http_request: HttpRequest) -> HttpResponse;
}

impl IntoHttpResponse for Response {

    fn into_http_response(self, http_request: HttpRequest) -> HttpResponse {
        let mut http_response = HttpResponse::new(StatusCode::from_u16(self.code()).unwrap());
        for key in self.headers().keys() {
            http_response.headers_mut().insert(HeaderName::from_str(&key).unwrap(), HeaderValue::from_str(self.headers().get(&key).unwrap().as_str()).unwrap());
        }
        match self.body().inner.as_ref() {
            BodyInner::Empty => { return http_response; }
            BodyInner::String(content) => { return http_response.set_body(content); },
            BodyInner::File(file) => { return http_response.set_body(file.to_str().unwrap().to_string()); },
        }
    }
}
