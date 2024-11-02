use bytes::Bytes;
use http_body_util::{BodyExt, Either, Full};
use hyper::{HeaderMap, StatusCode, Version};
use tower_http::services::fs::ServeFileSystemResponseBody;
use teo_result::{Result, Error};

#[derive(Clone)]
pub struct TestResponse {
    headers: HeaderMap,
    status: StatusCode,
    version: Version,
    body: Bytes,
}

impl TestResponse {
    pub(crate) async fn new(hyper_response: hyper::Response<Either<Full<Bytes>, ServeFileSystemResponseBody>>) -> Result<Self> {
        let (parts, body) = hyper_response.into_parts();
        let body = match body.collect().await {
            Ok(body) => body.to_bytes(),
            Err(_) => return Err(Error::internal_server_error_message("cannot read test response body")),
        };
        Ok(Self {
            status: parts.status,
            version: parts.version,
            headers: parts.headers,
            body,
        })
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn body(&self) -> &Bytes {
        &self.body
    }

    pub fn body_as_json(&self) -> Result<serde_json::Value> {
        let parsed_json_body_result: std::result::Result<serde_json::Value, serde_json::Error> = serde_json::from_slice(&self.body);
        match parsed_json_body_result {
            Ok(b) => Ok(b),
            Err(_) => Err(Error::internal_server_error_message("incorrect json format")),
        }
    }

    pub fn body_as_string(&self) -> String {
        unsafe { String::from_utf8_unchecked(self.body.to_vec()) }
    }
}