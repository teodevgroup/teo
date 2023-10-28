use actix_web::HttpResponse;
use crate::server::error::ErrorIntoHttpResponse;

pub trait ResultIntoHttpResponse<T> {
    fn result_into_http_response(self) -> Result<T, HttpResponse>;
}

impl<T> ResultIntoHttpResponse<T> for teo_runtime::path::Result<T> {

    fn result_into_http_response(self) -> Result<T, HttpResponse> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(e.error_into_http_response()),
        }
    }
}

impl<T> ResultIntoHttpResponse<T> for teo_result::Result<T> {

    fn result_into_http_response(self) -> Result<T, HttpResponse> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let path_error: teo_runtime::path::Error = e.into();
                Err(path_error.error_into_http_response())
            },
        }
    }
}