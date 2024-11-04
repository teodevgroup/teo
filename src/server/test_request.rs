use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::{HeaderMap, Method};
use hyper::body::Body;
use hyper::header::{HeaderValue, IntoHeaderName};
use teo_result::{Error, Result};

#[derive(Clone)]
pub struct TestRequest {
    method: Method,
    uri: String,
    headers: HeaderMap,
    body: Full<Bytes>,
}

impl TestRequest {
    pub fn new(method: Method, uri: &str) -> Self {
        Self {
            method,
            uri: uri.to_string(),
            headers: HeaderMap::new(),
            body: Full::new(Bytes::new()),
        }
    }

    pub fn insert_header<K, V>(mut self, key: K, value: V) -> Result<Self> where K: IntoHeaderName, V: TryInto<HeaderValue>, V::Error: std::error::Error {
        self.headers.insert::<K>(key.into(), value.try_into().map_err(|_| Error::internal_server_error_message("cannot read header value"))?);
        Ok(self)
    }

    pub fn append_header<K, V>(mut self, key: K, value: V) -> Result<Self> where K: IntoHeaderName, V: TryInto<HeaderValue>, V::Error: std::error::Error {
        self.headers.append::<K>(key.into(), value.try_into().map_err(|_| Error::internal_server_error_message("cannot read header value"))?);
        Ok(self)
    }

    pub async fn json_body(mut self, json: serde_json::Value) -> Result<Self> {
        self = self.insert_header("content-type", "application/json")?;
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
        self.body = Full::new(body);
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

    pub(crate) fn to_hyper_request(self) -> hyper::Request<Full<Bytes>> {
        let mut request = hyper::Request::builder()
            .method(self.method)
            .uri(self.uri);
        for (key, value) in self.headers.into_iter() {
            request = request.header(key.unwrap(), value.clone());
        }
        request.body(self.body).unwrap()
    }
}