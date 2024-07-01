use std::collections::BTreeMap;
use std::process::exit;
use std::env::current_dir;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use teo_result::{Error, Result};
use teo_runtime::namespace::Namespace;
use teo_runtime::namespace;
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
    pub(crate) namespace_builder: namespace::Builder,
    pub(crate) main_namespace: Arc<Mutex<&'static Namespace>>,
    pub(crate) cli: CLI,
    #[educe(Debug(ignore))]
    pub(crate) schema: Schema,
    #[educe(Debug(ignore))]
    pub(crate) setup: Arc<Mutex<Option<Arc<dyn AsyncCallback>>>>,
    #[educe(Debug(ignore))]
    pub(crate) programs: Arc<Mutex<BTreeMap<String, Program>>>,
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
        let namespace_builder = namespace::Builder::main();
        load_std(&namespace_builder);
        Ok(Self {
            argv,
            runtime_version,
            entrance,
            cli,
            schema,
            namespace_builder,
            main_namespace: Arc::new(Mutex::new(unsafe { &*null() })),
            setup: Arc::new(Mutex::new(None)),
            programs: Arc::new(Mutex::new(btreemap!{})),
            conn_ctx: Arc::new(Mutex::new(None)),
        })
    }

    pub fn setup<A, F>(&self, f: F) where F: AsyncCallbackArgument<A> + 'static {
        let wrap_call = Box::leak(Box::new(f));
        *self.setup.lock().unwrap() = Some(Arc::new(|ctx: transaction::Ctx| async {
            wrap_call.call(ctx).await
        }));
    }

    pub fn program<A, T, F>(&self, name: &str, desc: Option<T>, f: F) where T: Into<String>, F: AsyncCallbackArgument<A> + 'static {
        let wrap_call = Box::leak(Box::new(f));
        self.programs.lock().unwrap().insert(name.to_owned(), Program::new(desc.map(|desc| desc.into()), Arc::new(|ctx: transaction::Ctx| async {
            wrap_call.call(ctx).await
        })));
    }

    pub fn main_namespace(&self) -> &'static Namespace {
        *self.main_namespace.lock().unwrap()
    }

    pub fn namespace_builder(&self) -> &namespace::Builder {
        &self.namespace_builder
    }

    pub async fn run(&self) -> Result<()> {
        self.prepare_for_run().await?;
        self.run_without_prepare().await
    }

    pub async fn prepare_for_run(&self) -> Result<()> {
        load_schema(self.namespace_builder(), self.schema(), self.cli().command.ignores_loading()).await?;
        let namespace = Box::into_raw(Box::new(self.namespace_builder().build()));
        *self.main_namespace.lock().unwrap() = unsafe { &*namespace };
        Ok(())
    }

    pub async fn run_without_prepare(&self) -> Result<()> {
        run(self).await
    }

    pub fn conn_ctx(&self) -> connection::Ctx {
        self.conn_ctx.lock().unwrap().clone().unwrap()
    }

    pub fn runtime_version(&self) -> RuntimeVersion {
        self.runtime_version.clone()
    }

    pub fn entrance(&self) -> Entrance {
        self.entrance.clone()
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn cli(&self) -> &CLI {
        &self.cli
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let p = unsafe { self.main_namespace() as *const Namespace as *mut Namespace };
        if (!p.is_null()) {
            let _ = unsafe { Box::from_raw(p) };
        }
    }
}

unsafe impl Send for App {}
unsafe impl Sync for App {}