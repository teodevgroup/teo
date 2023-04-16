use crate::parser::parser::parser::Parser;

pub struct AppCtx {
    parser: Option<Box<Parser>>,
}

impl AppCtx {

    fn new() -> Self {
        Self { parser: None }
    }

    pub(super) fn drop() {
        unsafe {
            let reference = CURRENT.unwrap();
            let ptr = reference as *const AppCtx as *mut AppCtx;
            let _app_ctx = Box::from_raw(ptr);
            CURRENT = None;
        }
    }

    pub(super) fn create() -> bool {
        unsafe {
            if CURRENT.is_some() {
                return false;
            }
            let ptr = Box::into_raw(Box::new(AppCtx::new()));
            let reference = &mut *ptr;
            CURRENT = Some(reference);
            true
        }
    }
}

static mut CURRENT: Option<&'static AppCtx> = None;
