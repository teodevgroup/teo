use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use indexmap::IndexMap;
use maplit::hashmap;
use once_cell::sync::OnceCell;
use crate::app::entrance::Entrance;
use crate::app::namespace::Namespace;
use crate::app::program::LanguagePlatform;
use crate::core::callbacks::lookup::CallbackLookup;
use crate::core::callbacks::types::callback_with_user_ctx::AsyncCallbackWithUserCtx;
use crate::core::conf::test::TestConf;
use crate::core::connector::conf::ConnectorConf;
use crate::core::connector::connector::Connector;
use crate::parser::parser::parser::ASTParser;
use crate::prelude::{Graph, Middleware};
use crate::core::result::Result;
use crate::core::error::Error;
use crate::core::model::model::Model;
use crate::core::r#enum::Enum;
use crate::gen::interface::client::conf::ClientConf;
use crate::gen::interface::server::conf::EntityConf;
use crate::seeder::data_set::DataSet;
use crate::server::conf::ServerConf;
use crate::server::test_context::TestContext;

pub struct AppCtx {
    entrance: Entrance,
    language_platform: LanguagePlatform,
    parser: Box<ASTParser>,
    callbacks: CallbackLookup,
    connector: Option<Box<dyn Connector>>,
    graph: Graph,
    setup: Option<Arc<dyn AsyncCallbackWithUserCtx>>,
    programs: HashMap<String, Arc<dyn AsyncCallbackWithUserCtx>>,
    ignore_callbacks: bool,
    test_context: Option<&'static TestContext>,
    static_files: HashMap<&'static str, &'static str>,
    main_namespace: Namespace,
}

impl Debug for AppCtx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("AppCtx")
    }
}

impl AppCtx {

    pub fn get() -> Result<&'static AppCtx> {
        match CURRENT.get() {
            Some(ctx) => Ok({
                let retval = ctx.lock().unwrap();
                unsafe {
                    &*(retval.deref() as * const AppCtx)
                }
            }),
            None => Err(Error::fatal("App ctx is accessed when old_app is not created.")),
        }
    }

    pub(crate) fn get_mut() -> Result<&'static mut AppCtx> {
        match CURRENT.get() {
            Some(ctx) => Ok({
                let mut retval = ctx.lock().unwrap();
                unsafe {
                    &mut *(retval.deref_mut() as * mut AppCtx)
                }
            }),
            None => Err(Error::fatal("App ctx is mutably accessed when old_app is not created.")),
        }
    }

    fn new() -> Self {
        Self {
            entrance: Entrance::APP,
            language_platform: LanguagePlatform::Rust(env!("TEO_RUSTC_VERSION")),
            callbacks: CallbackLookup::new(),
            graph: Graph::new(),
            setup: None,
            ignore_callbacks: false,
            test_context: None,
            static_files: hashmap!{},
            programs: hashmap!{},
            main_namespace: Namespace::main(),
            parser: Box::new(ASTParser::new()),
            connector: None,
        }
    }

    fn reset(&mut self) {
        self.parser = Box::new(ASTParser::new());
        self.connector = None;
        self.static_files = hashmap!{};
        self.main_namespace = Namespace::main();
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
        Ok(Self::get_mut()?.reset())
    }

    pub(crate) fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn set_entrance(&self, entrance: Entrance) -> Result<()> {
        Self::get_mut()?.entrance = entrance;
        Ok(())
    }

    pub fn set_program(&self, program: LanguagePlatform) -> Result<()> {
        Self::get_mut()?.language_platform = program;
        Ok(())
    }

    pub(crate) fn callbacks(&self) -> &CallbackLookup {
        &self.callbacks
    }

    pub(crate) fn callbacks_mut(&self) -> &mut CallbackLookup {
        &mut Self::get_mut().unwrap().callbacks
    }

    pub(crate) fn parser(&self) -> &ASTParser {
        &self.parser
    }

    pub(crate) fn parser_mut(&self) -> &mut ASTParser {
        AppCtx::get_mut().unwrap().parser.as_mut()
    }

    pub(crate) fn main_namespace(&self) -> &Namespace {
        &self.main_namespace
    }

    pub(crate) fn main_namespace_mut(&self) -> &'static mut Namespace {
        &mut AppCtx::get_mut().unwrap().main_namespace
    }

    pub(crate) fn insert_program(&self, name: String, program: Arc<dyn AsyncCallbackWithUserCtx>) {
        AppCtx::get_mut().unwrap().programs.insert(name, program);
    }

    pub(crate) fn program(&self, name: &str) -> Option<Arc<dyn AsyncCallbackWithUserCtx>> {
        self.programs.get(name).cloned()
    }

    pub(crate) fn set_setup(&self, setup: Arc<dyn AsyncCallbackWithUserCtx>) {
        AppCtx::get_mut().unwrap().setup = Some(setup);
    }

    pub(crate) fn setup(&self) -> Option<Arc<dyn AsyncCallbackWithUserCtx>> {
        self.setup.clone()
    }

    pub(crate) fn set_connector(&self, connector: Box<dyn Connector>) {
        AppCtx::get_mut().unwrap().connector = Some(connector);
    }

    pub(crate) fn connector(&self) -> Result<&dyn Connector> {
        match &self.connector {
            Some(connector) => Ok(connector.as_ref()),
            None => Err(Error::fatal("Connector is accessed while it's not set."))
        }
    }

    pub(crate) fn langauge_platform(&self) -> &LanguagePlatform {
        &self.language_platform
    }

    pub(crate) fn entrance(&self) -> &Entrance {
        &self.entrance
    }

    pub(crate) fn set_ignore_callbacks(&self, value: bool) {
        AppCtx::get_mut().unwrap().ignore_callbacks = value;
    }

    pub(crate) fn ignore_callbacks(&self) -> bool {
        self.ignore_callbacks
    }

    pub(crate) fn set_test_context(&self, test_context: Option<&'static TestContext>) -> Result<()> {
        AppCtx::get_mut()?.test_context = test_context;
        Ok(())
    }

    pub(crate) fn test_context(&self) -> Option<&'static TestContext> {
        self.test_context
    }

    pub(crate) fn insert_static_files(&self, path: &'static str, map: &'static str) -> Result<()> {
        AppCtx::get_mut()?.static_files.insert(path, map);
        Ok(())
    }

    pub(crate) fn static_files(&self) -> &HashMap<&'static str, &'static str> {
        &self.static_files
    }

    pub(crate) fn connector_conf(&'static self) -> Result<&'static ConnectorConf> {
        match self.main_namespace().connector_conf() {
            Some(c) => Ok(c),
            None => Err(Error::fatal("Connector conf is accessed while it's not set.")),
        }
    }

    pub(crate) fn server_conf(&'static self) -> Result<&'static ServerConf> {
        match self.main_namespace().server_conf() {
            Some(s) => Ok(s),
            None => Err(Error::fatal("Server conf is accessed while it's not set.")),
        }
    }

    pub(crate) fn datasets(&'static self) -> Vec<&'static DataSet> {
        self.datasets_for_namespace(self.main_namespace())
    }

    fn datasets_for_namespace(&self, namespace: &'static Namespace) -> Vec<&'static DataSet> {
        let mut result = vec![];
        let datasets = namespace.datasets();
        for dataset in datasets {
            result.push(dataset);
        }
        for namespace in namespace.namespaces.values() {
            result.extend(self.datasets_for_namespace(namespace));
        }
        result
    }

    pub(crate) fn test_conf(&'static self) -> Option<&'static TestConf> {
        self.main_namespace().test_conf()
    }

    // TODO: get all middlewares
    pub(crate) fn middlewares(&'static self) -> &'static IndexMap<&'static str, &'static dyn Middleware> {
        self.main_namespace().middlewares()
    }

    pub(crate) fn entities(&'static self) -> &Vec<EntityConf> {
        self.main_namespace().entities()
    }

    pub(crate) fn clients(&'static self) -> &Vec<ClientConf> {
        self.main_namespace().clients()
    }

    pub fn models(&'static self) -> Vec<&'static Model> {
        self.models_for_namespace(self.main_namespace())
    }

    fn models_for_namespace(&'static self, namespace: &'static Namespace) -> Vec<&'static Model> {
        let mut result = vec![];
        let datasets: Vec<&Model> = namespace.models().values().collect();
        for dataset in datasets {
            result.push(dataset);
        }
        for namespace in namespace.namespaces.values() {
            result.extend(self.models_for_namespace(namespace));
        }
        result
    }

    pub fn enums(&'static self) -> Vec<&'static Enum> {
        self.enums_for_namespace(self.main_namespace())
    }

    fn enums_for_namespace(&'static self, namespace: &'static Namespace) -> Vec<&'static Enum> {
        let mut result = vec![];
        let datasets: Vec<&Enum> = namespace.enums().values().collect();
        for dataset in datasets {
            result.push(dataset);
        }
        for namespace in namespace.namespaces.values() {
            result.extend(self.enums_for_namespace(namespace));
        }
        result
    }

    pub fn model(&self, path: Vec<&str>) -> Result<Option<&Model>> {
        match path.len() {
            0 => Err(Error::fatal("Access model on AppCtx with 0 length path.")),
            1 => Ok(self.main_namespace().model(*path.get(0).unwrap())),
            _ => Ok({
                let mut ns = self.main_namespace();
                let mut retval = None;
                for (index, path_item) in path.iter().enumerate() {
                    if index == path.len() - 1 {
                        retval = ns.model(*path_item);
                        break
                    } else {
                        if ns.has_child_namespace(*path_item) {
                            ns = ns.child_namespace(*path_item).unwrap();
                        } else {
                            break
                        }
                    }
                }
                retval
            })
        }
    }

    pub fn r#enum(&self, path: Vec<&str>) -> Result<Option<&Enum>> {
        match path.len() {
            0 => Err(Error::fatal("Access enum on AppCtx with 0 length path.")),
            1 => Ok(self.main_namespace().r#enum(*path.get(0).unwrap())),
            _ => Ok({
                let mut ns = self.main_namespace();
                let mut retval = None;
                for (index, path_item) in path.iter().enumerate() {
                    if index == path.len() - 1 {
                        retval = ns.r#enum(*path_item);
                        break
                    } else {
                        if ns.has_child_namespace(*path_item) {
                            ns = ns.child_namespace(*path_item).unwrap();
                        } else {
                            break
                        }
                    }
                }
                retval
            })
        }
    }

    pub(crate) fn namespace_mut(&self, path: Vec<&str>) -> &mut Namespace {
        let mut ns = self.main_namespace_mut();
        for item in path {
            ns = ns.child_namespace_mut(item)
        }
        ns
    }
}

static CURRENT: OnceCell<Arc<Mutex<AppCtx>>> = OnceCell::new();