use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use teo_teon::Value;

pub trait ErrorIntoHttpResponse {
    fn error_into_http_response(self) -> HttpResponse;
}

impl ErrorIntoHttpResponse for teo_runtime::path::Error {

    fn error_into_http_response(self) -> HttpResponse {
        let value: Value = self.into();
        let json_value: serde_json::Value = value.try_into().unwrap();
        let mut response = HttpResponse::new(StatusCode::from_u16(self.code as u16).unwrap());
        response.set_body(json_value);
        response
    }
}