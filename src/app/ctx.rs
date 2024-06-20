use std::cell::UnsafeCell;
use educe::Educe;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use maplit::btreemap;
use teo_parser::ast::schema::Schema;
use teo_runtime::connection;
use teo_runtime::namespace::Namespace;
use crate::app::callbacks::callback::AsyncCallback;
use crate::app::program::Program;
use crate::cli::command::CLI;
use crate::cli::entrance::Entrance;
use crate::cli::runtime_version::RuntimeVersion;

#[derive(Educe)]
#[educe(Debug)]
pub struct Ctx {
    pub(crate) argv: Option<Vec<String>>,
    pub(crate) runtime_version: RuntimeVersion,
    pub(crate) entrance: Entrance,
    pub(crate) main_namespace: UnsafeCell<Namespace>,
    pub(crate) cli: CLI,
    #[educe(Debug(ignore))]
    pub(crate) schema: Schema,
    #[educe(Debug(ignore))]
    pub(crate) setup: Option<Arc<dyn AsyncCallback>>,
    #[educe(Debug(ignore))]
    pub(crate) programs: BTreeMap<String, Program>,
    #[educe(Debug(ignore))]
    pub(crate) conn_ctx: Arc<Mutex<Option<connection::Ctx>>>,
}

impl Ctx {

    pub(super) fn new(entrance: Entrance, runtime_version: RuntimeVersion, argv: Option<Vec<String>>, schema: Schema, cli: CLI, main_namespace: Namespace) -> Self {
        Self {
            argv,
            runtime_version,
            entrance,
            main_namespace: UnsafeCell::new(main_namespace),
            cli,
            schema,
            setup: None,
            programs: btreemap!{},
            conn_ctx: Arc::new(Mutex::new(None)),
        }
    }
}
