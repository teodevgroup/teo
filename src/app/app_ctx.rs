use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use indexmap::IndexMap;
use maplit::hashmap;
use once_cell::sync::OnceCell;
use crate::app::entrance::Entrance;
use crate::app::program::Program;
use crate::app::routes::action_ctx::{ActionCtxArgument, ActionHandlerDef, ActionHandlerDefTrait};
use crate::app::routes::middleware_ctx::Middleware;
use crate::core::callbacks::lookup::CallbackLookup;
use crate::core::callbacks::types::callback_with_user_ctx::AsyncCallbackWithUserCtx;
use crate::core::conf::debug::DebugConf;
use crate::core::conf::test::TestConf;
use crate::core::connector::connector::Connector;
use crate::core::connector::conf::ConnectorConf;
use crate::gen::interface::client::conf::Conf as ClientConf;
use crate::gen::interface::server::conf::Conf;
use crate::parser::parser::parser::ASTParser;
use crate::prelude::{Graph};
use crate::seeder::data_set::DataSet;
use crate::server::conf::ServerConf;
use crate::core::result::Result;
use crate::core::error::Error;
use crate::core::interface::CustomActionDefinition;
use crate::core::model::model::Model;
use crate::core::r#enum::Enum;
use crate::server::test_context::TestContext;

pub struct AppCtx {
    callbacks: CallbackLookup,
    entrance: Entrance,
    program: Program,
    parser: Option<Box<ASTParser>>,
    connector: Option<Box<dyn Connector>>,
    graph: Graph,
    enums: HashMap<&'static str, Enum>,
    models: HashMap<&'static str, Model>,
    connector_conf: Option<Box<ConnectorConf>>,
    server_conf: Option<Box<ServerConf>>,
    debug_conf: Option<Box<DebugConf>>,
    test_conf: Option<Box<TestConf>>,
    clients: Vec<ClientConf>,
    entities: Vec<Conf>,
    datasets: Vec<DataSet>,
    setup: Option<Arc<dyn AsyncCallbackWithUserCtx>>,
    ignore_callbacks: bool,
    middlewares: IndexMap<&'static str, &'static dyn Middleware>,
    action_handlers: Vec<&'static dyn ActionHandlerDefTrait>,
    action_map: HashMap<&'static str, HashMap<&'static str, &'static dyn ActionHandlerDefTrait>>,
    test_context: Option<&'static TestContext>,
    custom_action_declarations: HashMap<&'static str, HashMap<&'static str, CustomActionDefinition>>,
    static_files: HashMap<&'static str, &'static str>,
}

impl Debug for AppCtx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("AppCtx")
    }
}

impl AppCtx {

    fn new() -> Self {
        Self {
            callbacks: CallbackLookup::new(),
            parser: None,
            entrance: Entrance::APP,
            program: Program::Rust(env!("TEO_RUSTC_VERSION")),
            connector: None,
            graph: Graph::new(),
            enums: HashMap::new(),
            models: HashMap::new(),
            server_conf: None,
            connector_conf: None,
            clients: vec![],
            entities: vec![],
            datasets: vec![],
            debug_conf: None,
            test_conf: None,
            setup: None,
            ignore_callbacks: false,
            middlewares: IndexMap::new(),
            action_handlers: vec![],
            action_map: hashmap!{},
            test_context: None,
            custom_action_declarations: hashmap!{},
            static_files: hashmap!{},
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

    pub(in crate::app) fn drop() {
        // do nothing yet
    }

    pub fn get() -> Result<&'static AppCtx> {
        unsafe {
            match CURRENT.get() {
                Some(ctx) => Ok({
                    let retval = ctx.lock().unwrap();
                    unsafe {
                        &*(retval.deref() as * const AppCtx)
                    }
                }),
                None => Err(Error::fatal("App ctx is accessed while there is none.")),
            }
        }
    }

    pub(crate) fn graph(&self) -> &Graph {
        &self.graph
    }

    pub(crate) fn get_mut() -> Result<&'static mut AppCtx> {
        unsafe {
            match CURRENT.get() {
                Some(ctx) => Ok({
                    let mut retval = ctx.lock().unwrap();
                    unsafe {
                        &mut *(retval.deref_mut() as * mut AppCtx)
                    }
                }),
                None => Err(Error::fatal("App ctx is accessed mutably while there is none.")),
            }
        }
    }

    pub fn set_entrance(&self, entrance: Entrance) -> Result<()> {
        Self::get_mut()?.entrance = entrance;
        Ok(())
    }

    pub fn set_program(&self, program: Program) -> Result<()> {
        Self::get_mut()?.program = program;
        Ok(())
    }

    pub(crate) fn callbacks(&self) -> &CallbackLookup {
        &self.callbacks
    }

    pub(crate) fn callbacks_mut(&self) -> &mut CallbackLookup {
        &mut Self::get_mut().unwrap().callbacks
    }

    pub(crate) fn set_parser(&self, parser: Box<ASTParser>) {
        Self::get_mut().unwrap().parser = Some(parser);
    }

    pub(crate) fn parser(&self) -> Result<&ASTParser> {
        match &self.parser {
            Some(parser) => Ok(parser.as_ref()),
            None => Err(Error::fatal("Parser is accessed while it's not set.")),
        }
    }

    pub(crate) fn parser_mut(&self) -> Result<&mut ASTParser> {
        match &mut AppCtx::get_mut()?.parser {
            Some(parser) => Ok(parser.as_mut()),
            None => Err(Error::fatal("Parser is accessed mutably while it's not set.")),
        }
    }

    pub(crate) fn datasets(&self) -> &Vec<DataSet> {
        &self.datasets
    }

    pub(crate) fn datasets_mut(&self) -> &mut Vec<DataSet> {
        &mut AppCtx::get_mut().unwrap().datasets
    }

    pub(crate) fn set_setup(&self, setup: Arc<dyn AsyncCallbackWithUserCtx>) {
        AppCtx::get_mut().unwrap().setup = Some(setup);
    }

    pub(crate) fn setup(&self) -> Option<Arc<dyn AsyncCallbackWithUserCtx>> {
        self.setup.clone()
    }

    pub(crate) fn clients(&self) -> &Vec<ClientConf> {
        &self.clients
    }

    pub(crate) fn clients_mut(&self) -> &mut Vec<ClientConf> {
        &mut AppCtx::get_mut().unwrap().clients
    }

    pub(crate) fn entities(&self) -> &Vec<Conf> {
        &self.entities
    }

    pub(crate) fn entities_mut(&self) -> &mut Vec<Conf> {
        &mut AppCtx::get_mut().unwrap().entities
    }

    pub(crate) fn set_server_conf(&self, server_conf: Box<ServerConf>) {
        AppCtx::get_mut().unwrap().server_conf = Some(server_conf);
    }

    pub(crate) fn server_conf(&self) -> Result<&ServerConf> {
        match &self.server_conf {
            Some(server_conf) => Ok(server_conf.as_ref()),
            None => Err(Error::fatal("Server conf is accessed while it's not set."))
        }
    }

    pub fn models(&self) -> &HashMap<&'static str, Model> {
        &self.models
    }

    pub fn models_mut(&mut self) -> &mut HashMap<&'static str, Model> {
        &mut self.models
    }

    pub fn enums(&self) -> &HashMap<&'static str, Enum> {
        &self.enums
    }

    pub fn enums_mut(&mut self) -> &mut HashMap<&'static str, Enum> {
        &mut self.enums
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

    pub(crate) fn set_debug_conf(&self, debug_conf: Box<DebugConf>) {
        AppCtx::get_mut().unwrap().debug_conf = Some(debug_conf);
    }

    pub(crate) fn debug_conf(&self) -> Option<&DebugConf> {
        self.debug_conf.as_ref().map(|c| c.as_ref())
    }

    pub(crate) fn set_test_conf(&self, test_conf: Box<TestConf>) {
        AppCtx::get_mut().unwrap().test_conf = Some(test_conf);
    }

    pub(crate) fn test_conf(&self) -> Option<&TestConf> {
        self.test_conf.as_ref().map(|c| c.as_ref())
    }

    pub(crate) fn program(&self) -> &Program {
        &self.program
    }

    pub(crate) fn entrance(&self) -> &Entrance {
        &self.entrance
    }

    pub(crate) fn set_connector_conf(&self, connector_conf: Box<ConnectorConf>) {
        AppCtx::get_mut().unwrap().connector_conf = Some(connector_conf);
    }

    pub(crate) fn connector_conf(&self) -> Result<&ConnectorConf> {
        match &self.connector_conf {
            Some(connector_conf) => Ok(connector_conf.as_ref()),
            None => Err(Error::fatal("Connector conf is accessed while it's not set."))
        }
    }

    pub(crate) fn set_ignore_callbacks(&self, value: bool) {
        AppCtx::get_mut().unwrap().ignore_callbacks = value;
    }

    pub(crate) fn ignore_callbacks(&self) -> bool {
        self.ignore_callbacks
    }

    pub(crate) fn add_middleware<F>(&self, name: &'static str, f: F) -> Result<()> where
        F: Middleware + 'static,
    {
        AppCtx::get_mut()?.middlewares.insert(name, Box::leak(Box::new(f)));
        Ok(())
    }

    pub(crate) fn add_custom_action_declaration(&self, group: &'static str, name: &'static str, dec: CustomActionDefinition) -> Result<()> {
        let custom_action_declaration_mut = AppCtx::get_mut()?.custom_action_declaration_mut();
        if !custom_action_declaration_mut.contains_key(group) {
            custom_action_declaration_mut.insert(group, HashMap::new());
        }
        let name_map = custom_action_declaration_mut.get_mut(group).unwrap();
        name_map.insert(name, dec);
        Ok(())
    }

    pub(crate) fn add_action_handler<A: 'static, F>(&self, group: &'static str, name: &'static str, f: F) -> Result<()> where
        F: ActionCtxArgument<A> + 'static,
    {
        let handler_def = Box::leak(Box::new(ActionHandlerDef {
            group, name, f: Arc::new(f),
        }));
        AppCtx::get_mut()?.action_handlers.push(handler_def);
        let action_map_mut = AppCtx::get_mut()?.action_map_mut();
        if !action_map_mut.contains_key(group) {
            action_map_mut.insert(group, HashMap::new());
        }
        let name_map = action_map_mut.get_mut(group).unwrap();
        name_map.insert(name, handler_def);
        Ok(())
    }

    pub(crate) fn middlewares(&self) -> &IndexMap<&'static str, &'static dyn Middleware> {
        &self.middlewares
    }

    pub(crate) fn action_handlers(&self) -> &Vec<&'static dyn ActionHandlerDefTrait> {
        &self.action_handlers
    }

    pub(crate) fn action_map_mut(&mut self) -> &mut HashMap<&'static str, HashMap<&'static str, &'static dyn ActionHandlerDefTrait>> {
        &mut self.action_map
    }

    pub(crate) fn action_map(&self) -> &HashMap<&'static str, HashMap<&'static str, &'static dyn ActionHandlerDefTrait>> {
        &self.action_map
    }

    pub(crate) fn custom_action_declaration_mut(&mut self) -> &mut HashMap<&'static str, HashMap<&'static str, CustomActionDefinition>> {
        &mut self.custom_action_declarations
    }

    pub(crate) fn custom_action_declaration(&self) -> &HashMap<&'static str, HashMap<&'static str, CustomActionDefinition>> {
        &self.custom_action_declarations
    }

    pub(crate) fn has_action_handler_for(&self, group: &str, name: &str) -> bool {
        self.action_map().contains_key(group) && self.action_map().get(group).unwrap().contains_key(name)
    }

    pub(crate) fn get_action_handler(&self, group: &str, name: &str) -> &'static dyn ActionHandlerDefTrait {
        self.action_map().get(group).unwrap().get(name).cloned().unwrap()
    }

    pub(crate) fn has_custom_action_declaration_for(&self, group: &str, name: &str) -> bool {
        self.custom_action_declaration().contains_key(group) && self.custom_action_declaration().get(group).unwrap().contains_key(name)
    }

    pub(crate) fn get_custom_action_declaration_for(&self, group: &str, name: &str) -> &CustomActionDefinition {
        self.custom_action_declaration().get(group).unwrap().get(name).unwrap()
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
}

static CURRENT: OnceCell<Arc<Mutex<AppCtx>>> = OnceCell::new();