use std::cell::UnsafeCell;
use std::collections::BTreeMap;
use std::process::exit;
use std::env::current_dir;
use std::sync::{Arc, Mutex};
use teo_result::{Error, Result};
use teo_runtime::namespace::Namespace;
use teo_runtime::utils::find_main_schema_file;
use crate::cli::parse::{parse as cli_parse};
use teo_parser::{parse as schema_parse};
use teo_parser::diagnostics::printer::print_diagnostics;
use teo_runtime::stdlib::load::{load as load_std};
use teo_runtime::schema::load::load_schema::load_schema;
use crate::cli::run::run;
use dotenvy::dotenv;
use educe::Educe;
use maplit::btreemap;
use teo_parser::ast::schema::Schema;
use teo_runtime::connection::transaction;
use teo_runtime::connection;
use crate::app::callbacks::callback::{AsyncCallback, AsyncCallbackArgument};
use crate::app::program::Program;
use crate::cli::command::CLI;
use crate::prelude::{Entrance, RuntimeVersion};

#[derive(Educe)]
#[educe(Debug)]
pub struct App {
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

impl App {

    pub fn new() -> Result<Self> {
        Self::new_with_entrance_and_runtime_version(None, None, None)
    }

    pub fn new_with_entrance_and_runtime_version(entrance: Option<Entrance>, runtime_version: Option<RuntimeVersion>, argv: Option<Vec<String>>) -> Result<Self> {
        // load env first
        let _ = dotenv();
        let entrance = entrance.unwrap_or(Entrance::APP);
        let runtime_version = runtime_version.unwrap_or(RuntimeVersion::Rust(env!("TEO_RUSTC_VERSION")));
        let cli = cli_parse(&runtime_version, &entrance, argv.clone());
        let current_dir = match current_dir() {
            Ok(current_dir) => current_dir,
            Err(e) => Err(Error::new(format!("{}", e)))?,
        };
        let main_schema_file = find_main_schema_file(cli.schema.as_ref().map(AsRef::as_ref), &current_dir)?;
        let (schema, diagnostics) = schema_parse(main_schema_file.as_path().to_str().unwrap(), None, None);
        print_diagnostics(&diagnostics, true);
        if diagnostics.has_errors() {
            exit(1);
        }
        let mut namespace = Namespace::main();
        load_std(&mut namespace);
        Ok(Self {
            argv,
            runtime_version,
            entrance,
            main_namespace: UnsafeCell::new(namespace),
            cli,
            schema,
            setup: None,
            programs: btreemap!{},
            conn_ctx: Arc::new(Mutex::new(None)),
        })
    }

    pub fn setup<A, F>(&mut self, f: F) where F: AsyncCallbackArgument<A> + 'static {
        let wrap_call = Box::leak(Box::new(f));
        self.setup = Some(Arc::new(|ctx: transaction::Ctx| async {
            wrap_call.call(ctx).await
        }));
    }

    pub fn program<A, T, F>(&mut self, name: &str, desc: Option<T>, f: F) where T: Into<String>, F: AsyncCallbackArgument<A> + 'static {
        let wrap_call = Box::leak(Box::new(f));
        self.programs.insert(name.to_owned(), Program::new(desc.map(|desc| desc.into()), Arc::new(|ctx: transaction::Ctx| async {
            wrap_call.call(ctx).await
        })));
    }

    pub fn main_namespace(&self) -> &'static Namespace {
        let r = &self.main_namespace;
        unsafe { &*(r.get() as *const Namespace) }
    }

    pub fn main_namespace_mut(&self) -> &'static mut Namespace {
        unsafe { &mut *(self.main_namespace.get() as *const Namespace as *mut Namespace) }
    }

    pub async fn run(&self) -> Result<()> {
        self.prepare_for_run().await?;
        self.run_without_prepare().await
    }

    pub async fn prepare_for_run(&self) -> Result<()> {
        load_schema(self.main_namespace_mut(), self.schema(), self.cli().command.ignores_loading()).await
    }

    pub async fn run_without_prepare(&self) -> Result<()> {
        run(self).await
    }

    pub fn conn_ctx(&self) -> connection::Ctx {
        self.conn_ctx.lock().unwrap().clone().unwrap()
    }

    pub fn runtime_version(&self) -> &'static RuntimeVersion {
        let r = &self.runtime_version;
        unsafe { &*(r as *const RuntimeVersion) }
    }

    pub fn entrance(&self) -> &'static Entrance {
        let r = &self.entrance;
        unsafe { &*(r as *const Entrance) }
    }

    pub fn schema(&self) -> &'static Schema {
        let r = &self.schema;
        unsafe { &*(r as *const Schema) }
    }

    pub fn cli(&self) -> &'static CLI {
        let r = &self.cli;
        unsafe { &*(r as *const CLI) }
    }

    pub fn programs(&self) -> &BTreeMap<String, Program> {
        &self.programs
    }
}

unsafe impl Send for App {}
unsafe impl Sync for App {}