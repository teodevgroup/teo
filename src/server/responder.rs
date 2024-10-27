use std::str::FromStr;
use teo_runtime::response::body::BodyInner;
use teo_runtime::response::Response;

pub trait IntoHttpResponse {
    fn into_http_response(self, http_request: HttpRequest) -> HttpResponse;
}

impl IntoHttpResponse for Response {

    fn into_http_response(self, http_request: HttpRequest) -> HttpResponse {
        if let Some(file) = self.body().as_file() {
            let mut response = NamedFile::open(file).unwrap().into_response(&http_request);
            *response.status_mut() = StatusCode::from_u16(self.code()).unwrap();
            for key in self.headers().keys() {
                response.headers_mut().insert(HeaderName::from_str(&key).unwrap(), self.headers().get(&key).unwrap().as_str().parse().unwrap());
            }
            for c in self.cookies() {
                response.add_cookie(&c).unwrap();
            }
            response
        } else {
            let mut builder = HttpResponse::Ok();
            builder.status(StatusCode::from_u16(self.code()).unwrap());
            for key in self.headers().keys() {
                builder.insert_header((key.clone(), self.headers().get(&key).unwrap().as_str()));
            }
            for c in self.cookies() {
                builder.cookie(c);
            }
            match self.body().inner.as_ref() {
                BodyInner::String(content) => return builder.body(content.to_string()),
                BodyInner::Teon(value) => {
                    builder.content_type("application/json");
                    let json_value = serde_json::Value::try_from(value).unwrap();
                    let string_value = serde_json::to_string(&json_value).unwrap();
                    return builder.body(string_value);
                },
                _ => {},
            }
            builder.finish()
        }
    }
}
