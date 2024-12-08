use bytes::Bytes;
use http_body_util::{Either, Full};
use hyper::body::Body;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use mime::APPLICATION_JSON;
use teo_result::{Error, Result};
use teo_runtime::request::Request;
use teo_runtime::response::body::BodyInner;
use teo_runtime::response::Response;
use tower_http::services::fs::ServeFileSystemResponseBody;
use tower_http::services::ServeFile;

pub async fn hyper_response_from(request: Request, response: Response) -> Result<hyper::Response<Either<Full<Bytes>, ServeFileSystemResponseBody>>> {
    let mut hyper_response = {
        match response.body().inner.as_ref() {
            BodyInner::Empty => {
                let builder = hyper::Response::builder().status(response.code());
                let body_bytes = "".to_owned();
                Ok(builder.body(Either::Left(body_bytes.into())).unwrap())
            },
            BodyInner::String(content) => {
                let builder = hyper::Response::builder().status(response.code());
                let body_bytes = content.to_string();
                Ok(builder.body(Either::Left(body_bytes.into())).unwrap())
            },
            BodyInner::Teon(value) => {
                let mut builder = hyper::Response::builder().status(response.code());
                builder = builder.header(CONTENT_TYPE, APPLICATION_JSON.to_string());
                let json_value = serde_json::Value::try_from(value).unwrap();
                let string_value = serde_json::to_string(&json_value).unwrap();
                Ok(builder.body(Either::Left(string_value.into())).unwrap())
            },
            BodyInner::File(path_buf) => {
                let result = ServeFile::new(path_buf).try_call(request.clone_hyper_request_for_file_processing()).await;
                match result {
                    Ok(response) => {
                        let (parts, body) = response.into_parts();
                        Ok(hyper::Response::from_parts(parts, Either::Right(body)))
                    }
                    Err(err) => {
                        let error = Error::internal_server_error_message(format!("cannot read file: {:?}", err));
                        Err(error)
                    }
                }
            }
        }
    }?;
    response.headers().extend_to(hyper_response.headers_mut());
    for cookie in response.cookies() {
        hyper_response.headers_mut().append("Set-Cookie", HeaderValue::try_from(cookie.encoded())?);
    }
    Ok(hyper_response)
}
