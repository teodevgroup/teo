use std::fmt::{Display, Formatter};
use actix_http::body::BoxBody;
use actix_http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use teo_teon::Value;
use teo_result::Error;

#[derive(Debug)]
pub(super) struct WrapError(Error);

impl Display for WrapError {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<Error> for WrapError {

    fn from(value: Error) -> Self {
        Self(value)
    }
}

impl ResponseError for WrapError {

    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.0.code).unwrap()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let value: Value = (&self.0).into();
        let json_value: serde_json::Value = value.try_into().unwrap();
        HttpResponse::Ok().status(self.status_code()).json(json!({
            "error": json_value
        }))
    }
}
