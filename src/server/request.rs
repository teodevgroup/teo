use std::sync::Arc;
use actix_http::header::HeaderMap as HTTPHeaderMap;
use actix_http::HttpMessage;
use actix_web::HttpRequest;
use teo_runtime::request::header::readonly::HeaderMap;
use teo_runtime::request::request::r#trait;

pub struct HeadersImpl {
    pub http_headers: HTTPHeaderMap,
}

impl teo_runtime::request::header::readonly::r#trait::HeaderMap for HeadersImpl {

    fn keys(&self) -> Vec<&str> {
        self.http_headers.keys().collect()
    }

    fn len(&self) -> usize {
        self.http_headers.len()
    }

    fn contains_key(&self, key: &str) -> bool {
        self.http_headers.contains_key(key)
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.http_headers.get(key).map(|v| v.to_str().unwrap())
    }
}

pub struct RequestImpl {
    pub http_request: HttpRequest,
    pub header_map: HeaderMap,
}

impl RequestImpl {

    pub fn new(http_request: HttpRequest) -> Self {
        Self {
            http_request,
            header_map: HeaderMap {
                inner: Arc::new(HeadersImpl {
                    http_headers: http_request.headers().clone()
                })
            }
        }
    }
}

impl r#trait::Request for RequestImpl {

    fn method(&self) -> &str {
        self.http_request.method().as_str()
    }

    fn path(&self) -> &str {
        self.http_request.path()
    }

    fn query_string(&self) -> &str {
        self.http_request.query_string()
    }

    fn content_type(&self) -> &str {
        self.http_request.content_type()
    }

    fn headers(&self) -> &HeaderMap {
        &self.header_map
    }
}