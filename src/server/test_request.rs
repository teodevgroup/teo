use http_body_util::BodyExt;
use hyper::{HeaderMap, Method};
use hyper::body::Body;
use hyper::header::{HeaderValue, IntoHeaderName};
use teo_result::{Error, Result};

#[derive(Clone)]
pub struct TestRequest {
    method: Method,
    uri: String,
    headers: HeaderMap,
    body: String,
}

impl TestRequest {
    pub fn new(method: Method, uri: &str) -> Self {
        Self {
            method,
            uri: uri.to_string(),
            headers: HeaderMap::new(),
            body: String::new(),
        }
    }

    pub fn insert_header<K>(mut self, key: K, value: &str) -> Self where K: IntoHeaderName {
        self.headers.insert::<K>(key.into(), HeaderValue::from_str(value).unwrap());
        self
    }

    pub fn append_header<K>(mut self, key: K, value: &str) -> Self where K: IntoHeaderName {
        self.headers.append::<K>(key.into(), HeaderValue::from_str(value).unwrap());
        self
    }

    pub async fn json_body(mut self, json: serde_json::Value) -> Result<Self> {
        let body = match serde_json::to_string(&json) {
            Ok(body) => body,
            Err(_) => return Err(Error::internal_server_error_message("cannot serialize json value"))
        };
        self.set_body(body).await
    }

    pub async fn set_body<T: Body>(mut self, body: T) -> Result<Self> {
        let body = match body.collect().await {
            Ok(body) => body,
            Err(_) => return Err(Error::internal_server_error_message("cannot set body data, please fire a bug at https://github.com/teodevgroup/teo/issues")),
        }.to_bytes();
        let body = body.to_vec();
        let body = unsafe { String::from_utf8_unchecked(body) };
        self.body = body;
        Ok(self)
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub(crate) fn to_hyper_request(self) -> hyper::Request<String> {
        let mut request = hyper::Request::builder()
            .method(self.method)
            .uri(self.uri);
        for (key, value) in self.headers.into_iter() {
            request = request.header(key.unwrap(), value.clone());
        }
        request.body(self.body).unwrap()
    }
}