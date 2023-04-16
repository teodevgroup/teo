use super::new_error::Error;
use super::ctx::AppCtx;
use super::new_result::Result;

pub struct App(usize);

impl App {
    pub fn new() -> Result<Self> {
        if AppCtx::create() {
            Ok(Self(0))
        } else {
            Err(Error::fatal("A running Teo application cannot have more than 1 app instance."))
        }
    }

    pub fn setup() { }

    pub fn transform() { }

    pub fn validate() { }


}

impl Drop for App {
    fn drop(&mut self) {
        AppCtx::drop()
    }
}