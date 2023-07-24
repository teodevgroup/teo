use std::sync::Arc;
use actix_web::HttpRequest;
use actix_web::web::Payload;

#[derive(Clone)]
pub struct Req {
    request: HttpRequest,
    payload: Arc<Payload>,
}

impl Req {

    pub(crate) fn new(request: HttpRequest, payload: Payload) -> Self {
        Self { request, payload: Arc::new(payload) }
    }

}

unsafe impl Send for Req { }
unsafe impl Sync for Req { }
