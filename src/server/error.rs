use std::fmt::{Display, Formatter};
use actix_http::body::BoxBody;
use actix_http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use teo_teon::Value;

#[derive(Debug)]
pub(super) enum WrapError {
    PathError(teo_runtime::path::Error),
    ResultError(teo_result::Error),
}

impl Display for WrapError {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WrapError::PathError(e) => Display::fmt(e, f),
            WrapError::ResultError(e) => Display::fmt(e, f),
        }
    }
}

impl From<teo_runtime::path::Error> for WrapError {

    fn from(value: teo_runtime::path::Error) -> Self {
        Self::PathError(value)
    }
}

impl From<teo_result::Error> for WrapError {

    fn from(value: teo_result::Error) -> Self {
        Self::ResultError(value)
    }
}

impl ResponseError for WrapError {

    fn status_code(&self) -> StatusCode {
        match self {
            Self::PathError(e) => StatusCode::from_u16(e.code as u16).unwrap(),
            Self::ResultError(e) => StatusCode::from_u16(500).unwrap(),
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let value: Value = match self {
            WrapError::PathError(e) => e.into(),
            WrapError::ResultError(e) => {
                let path_error = teo_runtime::path::Error::from(e);
                path_error.into()
            },
        };
        let json_value: serde_json::Value = value.try_into().unwrap();
        let mut response = HttpResponse::new(self.status_code());
        response.set_body(json_value);
        response
    }
}
