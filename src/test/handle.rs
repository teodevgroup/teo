use std::ptr::null;
use actix_http::body::MessageBody;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::test;
use crate::app::App;
use crate::test::server::make_actix_app;

pub struct Handle {
    app: * const App,
}

impl Handle {

    pub fn new() -> Self {
        Self { app: null() }
    }

    pub fn load<F>(&mut self, f: F) where F: FnOnce() -> App {
        self.app = Box::into_raw(Box::new(f()));
    }

    pub fn unload(&mut self) {
        if !self.app.is_null() {
            unsafe { let _ = Box::from_raw(self.app as *mut App); }
            self.app = null();
        }
    }

    pub fn teo_app(&self) -> &App {
        unsafe { &*self.app }
    }

    pub async fn actix_app(&self) -> impl Service<
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