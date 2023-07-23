use actix_web::HttpRequest;
use actix_web::web::Payload;

pub struct Req {
    request: HttpRequest,
    payload: Payload,
}

impl Req {

    pub(crate) fn new(request: HttpRequest, payload: Payload) -> Self {
        Self { request, payload }
    }


}