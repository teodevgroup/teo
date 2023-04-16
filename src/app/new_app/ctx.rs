use crate::app::entrance::Entrance;
use crate::app::program::Program;
use crate::core::callbacks::lookup::CallbackLookup;
use crate::parser::parser::parser::Parser;
use super::new_result::Result;
use super::new_error::Error;

pub struct AppCtx {
    callbacks: CallbackLookup,
    parser: Option<Box<Parser>>,
    entrance: Entrance,
    program: Program,
}

impl AppCtx {

    fn new() -> Self {
        Self {
            callbacks: CallbackLookup::new(),
            parser: None,
            entrance: Entrance::APP,
            program: Program::Rust(env!("TEO_RUSTC_VERSION")),
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

    pub(super) fn drop() {
        unsafe {
            let reference = CURRENT.unwrap();
            let ptr = reference as *const AppCtx as *mut AppCtx;
            let _app_ctx = Box::from_raw(ptr);
            CURRENT = None;
        }
    }

    pub(crate) fn get() -> Result<&'static AppCtx> {
        unsafe {
            match CURRENT {
                Some(ctx) => Ok(ctx),
                None => Err(Error::fatal("App ctx is accessed while there is none.")),
            }
        }
    }

    pub(crate) fn get_mut() -> Result<&'static mut AppCtx> {
        unsafe {
            match CURRENT {
                Some(ctx) => Ok({
                    let ptr = ctx as *const AppCtx as *mut AppCtx;
                    &mut *ptr
                }),
                None => Err(Error::fatal("App ctx is accessed mutably while there is none.")),
            }
        }
    }

    pub(crate) fn callbacks(&self) -> &CallbackLookup {
        &self.callbacks
    }

    pub(crate) fn callbacks_mut(&mut self) -> &mut CallbackLookup {
        &mut self.callbacks
    }

    pub(crate) fn set_parser(&mut self, parser: Box<Parser>) {
        self.parser = Some(parser)
    }

    pub(crate) fn parser(&self) -> Result<&Parser> {
        match &self.parser {
            Some(parser) => Ok(parser.as_ref()),
            None => Err(Error::fatal("Parser is accessed while it's not set.")),
        }
    }

    pub fn set_entrance(entrance: Entrance) -> Result<()> {
        Self::get_mut()?.entrance = entrance;
        Ok(())
    }
}

static mut CURRENT: Option<&'static AppCtx> = None;
