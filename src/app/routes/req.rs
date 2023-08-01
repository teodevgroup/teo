use std::sync::Arc;
use actix_http::HttpMessage;
use actix_web::HttpRequest;
use actix_web::dev::Payload;
use crate::app::routes::readonly_header_map::ReadOnlyHeaderMap;

#[derive(Clone)]
pub struct Req {
    request: HttpRequest,
    payload: Arc<Payload>,
}

impl Req {

    pub(crate) fn new(request: HttpRequest, payload: Payload) -> Self {
        Self { request, payload: Arc::new(payload) }
    }

    pub fn method(&self) -> &str {
        self.request.method().as_str()
    }

    pub fn path(&self) -> &str {
        self.request.path()
    }

    pub fn query_string(&self) -> &str {
        self.request.query_string()
    }

    pub fn content_type(&self) -> &str {
        self.request.content_type()
    }

    pub fn headers(&self) -> ReadOnlyHeaderMap {
        ReadOnlyHeaderMap::new(self.request.headers().clone())
    }

}

unsafe impl Send for Req { }
unsafe impl Sync for Req { }
