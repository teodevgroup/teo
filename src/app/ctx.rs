use educe::Educe;
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use maplit::btreemap;
use once_cell::sync::OnceCell;
use teo_result::{Error, Result};
use teo_runtime::namespace::Namespace;
use crate::app::callbacks::callback::AsyncCallback;
use crate::cli::entrance::Entrance;
use crate::cli::runtime_version::RuntimeVersion;

#[derive(Educe)]
#[educe(Debug)]
pub struct Ctx {
    loaded: bool,
    pub(crate) runtime_version: RuntimeVersion,
    pub(crate) entrance: Entrance,
    pub(crate) main_namespace: Namespace,
    #[educe(Debug(ignore))]
    pub(crate) setup: Option<Arc<dyn AsyncCallback>>,
    #[educe(Debug(ignore))]
    pub(crate) programs: BTreeMap<String, Arc<dyn AsyncCallback>>,
}

impl Ctx {

    fn new() -> Self {
        Self {
            loaded: true,
            runtime_version: RuntimeVersion::Rust(env!("TEO_RUSTC_VERSION")),
            entrance: Entrance::APP,
            main_namespace: Namespace::main(),
            setup: None,
            programs: btreemap!{}
        }
    }

    pub(in crate::app) fn create() -> bool {
        if CURRENT.get().is_none() {
            CURRENT.set(Arc::new(Mutex::new(Self::new()))).unwrap();
            true
        } else {
            false
        }
    }

    pub(in crate::app) fn drop() -> Result<()> {
        Ok(Self::get_mut().reset())
    }

    pub fn get() -> &'static Ctx {
        match CURRENT.get() {
            Some(ctx) => {
                let retval = ctx.lock().unwrap();
                unsafe {
                    &*(retval.deref() as * const Ctx)
                }
            },
            None => panic!("app ctx is accessed when it's not created"),
        }
    }

    pub fn get_mut() -> &'static mut Ctx {
        match CURRENT.get() {
            Some(ctx) => {
                let mut retval = ctx.lock().unwrap();
                unsafe {
                    &mut *(retval.deref_mut() as * mut Ctx)
                }
            },
            None => panic!("app ctx is accessed mutably when it's not created"),
        }
    }

    fn reset(&mut self) {
        self.loaded = false;
    }

    fn reload(&mut self) {
        self.main_namespace = Namespace::main();
        self.loaded = true;
    }
}

static CURRENT: OnceCell<Arc<Mutex<Ctx>>> = OnceCell::new();