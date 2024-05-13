use std::sync::Arc;
use actix_http::header::HeaderMap as HTTPHeaderMap;
use actix_http::HttpMessage;
use actix_web::HttpRequest;
use chrono::{DateTime, TimeZone, Utc};
use teo_runtime::request::header::readonly::HeaderMap;
use teo_runtime::request::request::r#trait;
use teo_result::{Error, Result};
use teo_runtime::request::cookie::readonly::Cookie;

pub struct CookieImpl {
    name: String,
    path: Option<String>,
    value: String,
    expires_datetime: Option<DateTime<Utc>>,
    expires_session: bool,
    secure: Option<bool>,
    max_age: Option<f64>,
}

impl CookieImpl {
    pub fn from_actix_cookie<'a>(cookie: &'a actix_web::cookie::Cookie<'a>) -> Self {
        Self {
            name: cookie.name().to_owned(),
            path: cookie.path().map(|s| s.to_owned()),
            value: cookie.value().to_owned(),
            expires_datetime: if let Some(expires) = cookie.expires() {
                if let Some(datetime) = expires.datetime() {
                    Some(Utc.timestamp_opt(datetime.unix_timestamp(), 0).unwrap())
                } else {
                    None
                }
            } else {
                None
            },
            expires_session: cookie.expires().is_some() && cookie.expires().unwrap().is_session(),
            secure: cookie.secure(),
            max_age: cookie.max_age().map(|v| v.as_seconds_f64()),
        }
    }

}

impl teo_runtime::request::cookie::readonly::r#trait::Cookie for CookieImpl {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn path(&self) -> Option<&str> {
        self.path.as_ref().map(|p| p.as_str())
    }

    fn value(&self) -> &str {
        self.value.as_str()
    }

    fn expires_datetime(&self) -> Option<&DateTime<Utc>> {
        self.expires_datetime.as_ref()
    }

    fn expires_session(&self) -> bool {
        self.expires_session
    }

    fn secure(&self) -> Option<bool> {
        self.secure
    }

    fn max_age(&self) -> Option<f64> {
        self.max_age
    }
}

pub struct HeadersImpl {
    pub http_headers: HTTPHeaderMap,
}

impl teo_runtime::request::header::readonly::r#trait::HeaderMap for HeadersImpl {

    fn keys(&self) -> Vec<&str> {
        self.http_headers.keys().map(|k| k.as_str()).collect()
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
            http_request: http_request.clone(),
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

    fn cookies(&self) -> Result<Vec<Cookie>> {
        if let Ok(cookies) = self.http_request.cookies() {
            Ok(cookies.iter().map(|c| Cookie {
                inner: Arc::new(CookieImpl::from_actix_cookie(c))
            }).collect())
        } else {
            Err(Error::new("cookie parsing error"))
        }
    }
}