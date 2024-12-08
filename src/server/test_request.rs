use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::Method;
use hyper::body::Body;
use hyper::header::{HeaderValue, IntoHeaderName};
use teo_result::{Error, Result};
use teo_runtime::cookies::Cookies;
use teo_runtime::headers::Headers;

#[derive(Clone)]
pub struct TestRequest {
    method: Method,
    uri: String,
    headers: Headers,
    cookies: Cookies,
    body: Full<Bytes>,
}

impl TestRequest {
    pub fn new(method: Method, uri: &str) -> Self {
        Self {
            method,
            uri: uri.to_string(),
            headers: Headers::new(),
            cookies: Cookies::new(),
            body: Full::new(Bytes::new()),
        }
    }

    pub async fn json_body(mut self, json: serde_json::Value) -> Result<Self> {
        self.headers().insert("content-type", "application/json")?;
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

    pub fn insert_header<K, V>(mut self, key: K, value: V) -> Result<Self> where K: Into<String>, V: Into<String> {
        self.headers.insert(key, value)?;
        Ok(self)
    }

    pub fn append_header<K, V>(mut self, key: K, value: V) -> Result<Self> where K: Into<String>, V: Into<String> {
        self.headers.append(key, value)?;
        Ok(self)
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn set_headers(&mut self, headers: Headers) {
        self.headers = headers
    }

    pub fn cookies(&self) -> &Cookies {
        &self.cookies
    }

    pub fn set_cookies(&mut self, cookies: Cookies) {
        self.cookies = cookies
    }

    pub(crate) fn to_hyper_request(self) -> Result<hyper::Request<Full<Bytes>>> {
        let headers = self.headers().clone();
        let cookies = self.cookies().clone();
        let mut request = hyper::Request::builder()
            .method(self.method)
            .uri(self.uri);
        let mut request = request.body(self.body).unwrap();
        headers.extend_to(request.headers_mut());
        for cookie in cookies {
            request.headers_mut().append("Cookie", HeaderValue::try_from(cookie.encoded())?);
        }
        Ok(request)
    }
}