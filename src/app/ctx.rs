use std::sync::Arc;
use crate::app::entrance::Entrance;
use crate::app::program::Program;
use crate::core::callbacks::lookup::CallbackLookup;
use crate::core::callbacks::types::callback_without_args::AsyncCallbackWithoutArgs;
use crate::core::conf::debug::DebugConf;
use crate::core::conf::test::TestConf;
use crate::core::connector::{Connector, ConnectorConf};
use crate::gen::interface::client::conf::Conf as ClientConf;
use crate::gen::interface::server::conf::EntityGeneratorConf;
use crate::parser::parser::parser::ASTParser;
use crate::prelude::Graph;
use crate::seeder::data_set::DataSet;
use crate::server::conf::ServerConf;
use super::new_result::Result;
use super::new_error::Error;

#[derive(Debug)]
pub struct AppCtx {
    callbacks: CallbackLookup,
    entrance: Entrance,
    program: Program,
    parser: Option<Box<ASTParser>>,
    connector: Option<Box<dyn Connector>>,
    graph: Option<Box<Graph>>,
    connector_conf: Option<Box<ConnectorConf>>,
    server_conf: Option<Box<ServerConf>>,
    debug_conf: Option<Box<DebugConf>>,
    test_conf: Option<Box<TestConf>>,
    clients: Vec<ClientConf>,
    entities: Vec<EntityGeneratorConf>,
    datasets: Vec<DataSet>,
    setup: Option<Arc<dyn AsyncCallbackWithoutArgs>>,
}

impl AppCtx {

    fn new() -> Self {
        Self {
            callbacks: CallbackLookup::new(),
            parser: None,
            entrance: Entrance::APP,
            program: Program::Rust(env!("TEO_RUSTC_VERSION")),
            connector: None,
            graph: None,
            server_conf: None,
            clients: vec![],
            entities: vec![],
            datasets: vec![],
            debug_conf: None,
            test_conf: None,
            setup: None,
        }
    }

    pub(in crate::app) fn create() -> bool {
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

    pub(in crate::app) fn drop() {
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

    pub fn set_entrance(entrance: Entrance) -> Result<()> {
        Self::get_mut()?.entrance = entrance;
        Ok(())
    }

    pub fn set_program(program: Program) -> Result<()> {
        Self::get_mut()?.program = program;
        Ok(())
    }

    pub(crate) fn callbacks(&self) -> &CallbackLookup {
        &self.callbacks
    }

    pub(crate) fn callbacks_mut(&mut self) -> &mut CallbackLookup {
        &mut self.callbacks
    }

    pub(crate) fn set_parser(&mut self, parser: Box<ASTParser>) {
        self.parser = Some(parser)
    }

    pub(crate) fn parser(&self) -> Result<&ASTParser> {
        match &self.parser {
            Some(parser) => Ok(parser.as_ref()),
            None => Err(Error::fatal("Parser is accessed while it's not set.")),
        }
    }

    pub(crate) fn parser_mut(&mut self) -> Result<&mut ASTParser> {
        match &mut self.parser {
            Some(parser) => Ok(parser.as_mut()),
            None => Err(Error::fatal("Parser is accessed mutably while it's not set.")),
        }
    }

    pub(crate) fn datasets(&self) -> &Vec<DataSet> {
        &self.datasets
    }

    pub(crate) fn datasets_mut(&mut self) -> &mut Vec<DataSet> {
        &mut self.datasets
    }

    pub(crate) fn set_setup(&mut self, setup: Arc<dyn AsyncCallbackWithoutArgs>) {
        self.setup = Some(setup);
    }

    pub(crate) fn setup(&self) -> Option<Arc<dyn AsyncCallbackWithoutArgs>> {
        self.setup.clone()
    }

    pub(crate) fn clients(&self) -> &Vec<ClientConf> {
        &self.clients
    }

    pub(crate) fn clients_mut(&mut self) -> &mut Vec<ClientConf> {
        &mut self.clients
    }

    pub(crate) fn entities(&self) -> &Vec<EntityGeneratorConf> {
        &self.entities
    }

    pub(crate) fn entities_mut(&mut self) -> &mut Vec<EntityGeneratorConf> {
        &mut self.entities
    }

    pub(crate) fn set_server_conf(&mut self, server_conf: Box<ServerConf>) {
        self.server_conf = Some(server_conf);
    }

    pub(crate) fn server_conf(&self) -> Result<&ServerConf> {
        match &self.server_conf {
            Some(server_conf) => Ok(server_conf.as_ref()),
            None => Err(Error::fatal("Server conf is accessed while it's not set."))
        }
    }

    pub(crate) fn set_graph(&mut self, graph: Box<Graph>) {
        self.graph = Some(graph);
    }

    pub(crate) fn graph(&self) -> Result<&Graph> {
        match &self.graph {
            Some(graph) => Ok(graph.as_ref()),
            None => Err(Error::fatal("Graph is accessed while it's not set.")),
        }
    }

    pub(crate) fn graph_mut(&mut self) -> Result<&mut Graph> {
        match &mut self.graph {
            Some(graph) => Ok(graph.as_mut()),
            None => Err(Error::fatal("Graph is accessed mutably while it's not set.")),
        }
    }

    pub(crate) fn set_connector(&mut self, connector: Box<dyn Connector>) {
        self.connector = Some(connector);
    }

    pub(crate) fn connector(&self) -> Result<&dyn Connector> {
        match &self.connector {
            Some(connector) => Ok(connector.as_ref()),
            None => Err(Error::fatal("Connector is accessed while it's not set."))
        }
    }

    pub(crate) fn set_debug_conf(&mut self, debug_conf: Box<DebugConf>) {
        self.debug_conf = Some(debug_conf);
    }

    pub(crate) fn debug_conf(&self) -> Option<&DebugConf> {
        self.debug_conf.map(|c| c.as_ref())
    }

    pub(crate) fn set_test_conf(&mut self, test_conf: Box<TestConf>) {
        self.test_conf = Some(test_conf);
    }

    pub(crate) fn test_conf(&self) -> Option<&TestConf> {
        self.test_conf.map(|c| c.as_ref())
    }

    pub(crate) fn program(&self) -> &Program {
        &self.program
    }

    pub(crate) fn entrance(&self) -> &Entrance {
        &self.entrance
    }

    pub(crate) fn set_connector_conf(&mut self, connector_conf: Box<ConnectorConf>) {
        self.connector_conf = Some(connector_conf);
    }

    pub(crate) fn connector_conf(&self) -> Result<&ConnectorConf> {
        match &self.connector_conf {
            Some(connector_conf) => Ok(connector_conf.as_ref()),
            None => Err(Error::fatal("Connector conf is accessed while it's not set."))
        }
    }
}

static mut CURRENT: Option<&'static AppCtx> = None;
