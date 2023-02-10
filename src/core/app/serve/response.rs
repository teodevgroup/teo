use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder};
use serde_json::json;
use crate::core::error::Error;

impl Into<HttpResponse> for Error {
    fn into(self) -> HttpResponse {
        HttpResponseBuilder::new(StatusCode::from_u16(self.r#type.code()).unwrap()).json(json!({"error": self}))
    }
}
