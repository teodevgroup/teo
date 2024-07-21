use std::ptr::null;
use actix_http::body::MessageBody;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::test;
use teo::app::App;
use crate::lib::server::make_actix_app;

pub(crate) struct Handle {
    app: * const App,
}

impl Handle {

    pub(crate) fn new() -> Self {
        Self { app: null() }
    }

    pub(crate) fn load<F>(&mut self, f: F) where F: FnOnce() -> App {
        self.app = Box::into_raw(Box::new(f()));
    }

    pub(crate) fn unload(&mut self) {
        if !self.app.is_null() {
            unsafe { let _ = Box::from_raw(self.app as *mut App); }
            self.app = null();
        }
    }

    pub(crate) fn teo_app(&self) -> &App {
        unsafe { &*self.app }
    }

    pub(crate) async fn actix_app(&self) -> impl Service<
        actix_http::Request,
        Response = ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
    > {
        test::init_service(
            make_actix_app(
                self.teo_app()
            ).await.unwrap()
        ).await
    }
}

unsafe impl Send for Handle { }
unsafe impl Sync for Handle { }