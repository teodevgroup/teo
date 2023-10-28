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
        let mut builder = HttpResponse::Ok();
        builder.status(StatusCode::from_u16(self.code()).unwrap());
        for key in self.headers().keys() {
            builder.insert_header((key.clone(), self.headers().get(&key).unwrap().as_str()));
        }
        match self.body().inner.as_ref() {
            BodyInner::Empty => (),
            BodyInner::String(content) => return builder.body(content.to_string()),
            BodyInner::File(file) => return builder.body(file.to_str().unwrap().to_string()),
        }
        builder.finish()
    }
}
