use std::cell::Cell;
use std::collections::BTreeMap;
use std::process::exit;
use std::env::current_dir;
use std::ptr::{null, null_mut};
use std::sync::{Arc, Mutex};
use teo_result::{Error, Result};
use teo_runtime::namespace::Namespace;
use teo_runtime::namespace;
use teo_runtime::utils::find_main_schema_file;
use teo_parser::{parse as schema_parse};
use teo_parser::diagnostics::printer::print_diagnostics;
use teo_runtime::stdlib::load::{load as load_std};
use teo_runtime::schema::load::load_schema::load_schema;
use dotenvy::dotenv;
use educe::Educe;
use maplit::btreemap;
use teo_parser::ast::schema::Schema;
use teo_runtime::connection::transaction;
use teo_runtime::connection;
use teo_runtime::app::cleanup::Cleanup;
use teo_runtime::app::data::AppData;
use teo_runtime::app::entrance::Entrance;
use teo_runtime::app::runtime_version::RuntimeVersion;
use crate::app::callbacks::{AsyncCallback, AsyncCallbackArgument};
use crate::app::program::Program;
use crate::cli::cli_parse::cli_parse;
use crate::cli::command::CLI;
use crate::cli::run::run;

#[derive(Clone, Debug)]
pub struct App {
    inner: Arc<Inner>,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct Inner {
    argv: Option<Vec<String>>,
    main_namespace: namespace::Builder,
    compiled_main_namespace: Arc<Mutex<&'static Namespace>>,
    cli: CLI,
    #[educe(Debug(ignore))]
    schema: Schema,
    #[educe(Debug(ignore))]
    setup: Arc<Mutex<Option<Arc<dyn AsyncCallback>>>>,
    #[educe(Debug(ignore))]
    programs: Arc<Mutex<BTreeMap<String, Program>>>,
    #[educe(Debug(ignore))]
    conn_ctx: Arc<Mutex<Option<connection::Ctx>>>,
    app_data: AppData,
}

impl App {

    pub fn new() -> Result<Self> {
        Self::new_with_entrance_and_runtime_version(None, None, None)
    }

    pub fn new_with_argv(argv: Vec<String>) -> Result<Self> {
        Self::new_with_entrance_and_runtime_version(None, None, Some(argv))
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
            inner: Arc::new(Inner {
                argv,
                cli,
                schema,
                main_namespace: namespace_builder,
                compiled_main_namespace: Arc::new(Mutex::new(unsafe { &*null() })),
                setup: Arc::new(Mutex::new(None)),
                programs: Arc::new(Mutex::new(btreemap!{})),
                conn_ctx: Arc::new(Mutex::new(None)),
                app_data: AppData::new(entrance, runtime_version),
            })
        })
    }

    pub fn setup<A, F>(&self, f: F) where F: AsyncCallbackArgument<A> + 'static {
        let wrap_call = Box::leak(Box::new(f));
        *self.inner.setup.lock().unwrap() = Some(Arc::new(|ctx: transaction::Ctx| async {
            wrap_call.call(ctx).await
        }));
    }

    pub fn get_setup(&self) -> Option<Arc<dyn AsyncCallback>> {
        self.inner.setup.lock().unwrap().clone()
    }

    pub fn program<A, T, F>(&self, name: &str, desc: Option<T>, f: F) where T: Into<String>, F: AsyncCallbackArgument<A> + 'static {
        let wrap_call = Box::leak(Box::new(f));
        self.inner.programs.lock().unwrap().insert(name.to_owned(), Program::new(desc.map(|desc| desc.into()), Arc::new(|ctx: transaction::Ctx| async {
            wrap_call.call(ctx).await
        })));
    }

    pub fn compiled_main_namespace(&self) -> &'static Namespace {
        *self.inner.compiled_main_namespace.lock().unwrap()
    }

    pub fn main_namespace(&self) -> &namespace::Builder {
        &self.inner.main_namespace
    }

    pub fn conn_ctx(&self) -> connection::Ctx {
        self.inner.conn_ctx.lock().unwrap().clone().unwrap()
    }

    pub fn replace_conn_ctx(&self, ctx: connection::Ctx) {
        *self.inner.conn_ctx.lock().unwrap() = Some(ctx);
    }

    pub fn runtime_version(&self) -> RuntimeVersion {
        self.inner.app_data.runtime_version().clone()
    }

    pub fn entrance(&self) -> Entrance {
        self.inner.app_data.entrance().clone()
    }

    pub fn schema(&self) -> &Schema {
        &self.inner.schema
    }

    pub fn cli(&self) -> &CLI {
        &self.inner.cli
    }

    pub fn programs(&self) -> BTreeMap<String, Program> {
        self.inner.programs.lock().unwrap().clone()
    }

    pub fn set_compiled_main_namespace(&self, main_namespace: &'static Namespace) {
        *self.inner.compiled_main_namespace.lock().unwrap() = main_namespace;
    }

    pub fn app_data(&self) -> AppData {
        self.inner.app_data.clone()
    }

    pub async fn run(&self) -> Result<()> {
        self.prepare_for_run().await?;
        self.run_without_prepare().await
    }

    pub async fn prepare_for_run(&self) -> Result<()> {
        load_schema(self.main_namespace(), self.schema(), self.cli().command.ignores_loading()).await?;
        let namespace = Box::into_raw(Box::new(self.main_namespace().build()));
        self.set_compiled_main_namespace(unsafe { &*namespace });
        Ok(())
    }

    pub async fn run_without_prepare(&self) -> Result<()> {
        run(self).await
    }
}

// impl Drop for App {
//     fn drop(&mut self) {
//         // drop dynamic classes
//         if let Some(clean_up) = self.dynamic_classes_clean_up() {
//             clean_up.call(self);
//         }
//         // drop namespace
//         let p = unsafe { self.compiled_main_namespace() as *const Namespace as *mut Namespace };
//         if !p.is_null() {
//             let _ = unsafe { Box::from_raw(p) };
//         }
//     }
// }

unsafe impl Send for App { }
unsafe impl Sync for App { }