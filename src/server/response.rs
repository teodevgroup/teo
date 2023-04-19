use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder};
use serde_json::json;
use crate::core::error::Error;

impl Into<HttpResponse> for Error {
    fn into(self) -> HttpResponse {
        HttpResponseBuilder::new(StatusCode::from_u16(status_code_for_error(&self)).unwrap()).json(json!({"error": self}))
    }
}

fn status_code_for_error(error: &Error) -> u16 {
    match error {
        Error::ServerError(_) => 500,
        Error::RuntimeError(_) => 500,
        Error::FatalError(_) => 500,
        Error::UserError(user_error) => user_error.code(),
    }
}

pub(super) fn error_to_json(error: &Error) -> serde_json::Value {
    match error {
        Error::ServerError(server_error) => json!({"type": "InternalServerError", "message": server_error.0}),
        Error::FatalError(fatal_error) => json!({"type": "InternalServerError", "message": fatal_error.0}),
        Error::RuntimeError(runtime_error) => json!({"type": "InternalServerError", "message": runtime_error.message()}),
        Error::UserError(user_error) => user_error.into(),
    }
}