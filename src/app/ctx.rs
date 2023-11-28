use educe::Educe;
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use maplit::btreemap;
use once_cell::sync::OnceCell;
use teo_parser::ast::schema::Schema;
use teo_result:: Result;
use teo_runtime::connection;
use teo_runtime::namespace::Namespace;
use crate::app::callbacks::callback::AsyncCallback;
use crate::cli::command::CLI;
use crate::cli::entrance::Entrance;
use crate::cli::runtime_version::RuntimeVersion;

#[derive(Educe)]
#[educe(Debug)]
pub struct Ctx {
    loaded: bool,
    pub(crate) runtime_version: RuntimeVersion,
    pub(crate) entrance: Entrance,
    pub(crate) main_namespace: Namespace,
    pub(crate) cli: Option<CLI>,
    #[educe(Debug(ignore))]
    pub(crate) schema: Option<Schema>,
    #[educe(Debug(ignore))]
    pub(crate) setup: Option<Arc<dyn AsyncCallback>>,
    #[educe(Debug(ignore))]
    pub(crate) programs: BTreeMap<String, Arc<dyn AsyncCallback>>,
    #[educe(Debug(ignore))]
    pub(crate) conn_ctx: Option<connection::Ctx>,
}

impl Ctx {

    fn new() -> Self {
        Self {
            loaded: true,
            runtime_version: RuntimeVersion::Rust(env!("TEO_RUSTC_VERSION")),
            entrance: Entrance::APP,
            main_namespace: Namespace::main(),
            cli: None,
            schema: None,
            setup: None,
            programs: btreemap!{},
            conn_ctx: None,
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

    pub fn main_namespace() -> &'static Namespace {
        &Ctx::get().main_namespace
    }

    pub fn main_namespace_mut() -> &'static mut Namespace {
        &mut Ctx::get_mut().main_namespace
    }

    pub fn set_cli(cli: CLI) {
        Ctx::get_mut().cli = Some(cli)
    }

    pub fn cli() -> &'static CLI {
        Ctx::get().cli.as_ref().unwrap()
    }

    pub fn set_schema(schema: Schema) {
        Ctx::get_mut().schema = Some(schema)
    }

    pub fn schema() -> &'static Schema {
        Ctx::get().schema.as_ref().unwrap()
    }

    pub fn set_entrance(entrance: Entrance) {
        Ctx::get_mut().entrance = entrance;
    }

    pub fn conn_ctx() -> &'static connection::Ctx {
        Ctx::get().conn_ctx.as_ref().unwrap()
    }

    pub fn setup() -> Option<&'static Arc<dyn AsyncCallback>> {
        Ctx::get().setup.as_ref()
    }

    pub fn set_setup<F>(f: F) where F: AsyncCallback + 'static {
        Ctx::get_mut().setup = Some(Arc::new(f));
    }

    pub fn insert_program<F>(name: &str, f: F) where F: AsyncCallback + 'static {
        Ctx::get_mut().programs.insert(name.to_owned(), Arc::new(f));
    }
}

static CURRENT: OnceCell<Arc<Mutex<Ctx>>> = OnceCell::new();