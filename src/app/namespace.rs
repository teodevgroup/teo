use std::collections::HashMap;
use std::sync::Arc;
use indexmap::{IndexMap, indexmap};
use maplit::hashmap;
use crate::core::conf::debug::DebugConf;
use crate::core::conf::test::TestConf;
use crate::core::connector::conf::ConnectorConf;
use crate::core::interface::CustomActionDefinition;
use crate::core::model::model::Model;
use crate::core::r#enum::Enum;
use crate::server::conf::ServerConf;
use crate::gen::interface::client::conf::ClientConf as ClientConf;
use crate::gen::interface::server::conf::EntityConf as EntityConf;
use crate::parser::ast::entity::Entity;
use crate::prelude::{ActionCtxArgument, ActionHandlerDef, ActionHandlerDefTrait, Middleware};
use crate::seeder::data_set::DataSet;
use crate::core::result::Result;

pub struct Namespace {
    pub(crate) is_main: bool,
    pub(crate) namespaces: HashMap<&'static str, Namespace>,
    // configuration and declarations
    pub(crate) connector_conf: Option<Box<ConnectorConf>>,
    pub(crate) server_conf: Option<Box<ServerConf>>,
    pub(crate) debug_conf: Option<Box<DebugConf>>,
    pub(crate) test_conf: Option<Box<TestConf>>,
    pub(crate) clients: Vec<ClientConf>,
    pub(crate) entities: Vec<EntityConf>,
    pub(crate) datasets: Vec<DataSet>,
    pub(crate) enums: HashMap<&'static str, Enum>,
    pub(crate) models: HashMap<&'static str, Model>,
    pub(crate) middlewares: IndexMap<&'static str, &'static dyn Middleware>,
    pub(crate) action_handlers: Vec<&'static dyn ActionHandlerDefTrait>,
    pub(crate) action_map: HashMap<&'static str, HashMap<&'static str, &'static dyn ActionHandlerDefTrait>>,
    pub(crate) custom_action_declarations: HashMap<&'static str, HashMap<&'static str, CustomActionDefinition>>,
    pub(crate) constants: HashMap<&'static str, Entity>,
}

impl Namespace {

    pub fn new() -> Self {
        Self::new_internal(false)
    }

    pub(crate) fn main() -> Self {
        Self::new_internal(true)
    }

    fn new_internal(is_main: bool) -> Self {
        Self {
            is_main,
            namespaces: hashmap!{},
            connector_conf: None,
            server_conf: None,
            debug_conf: None,
            test_conf: None,
            clients: vec![],
            entities: vec![],
            datasets: vec![],
            enums: hashmap!{},
            models: hashmap!{},
            middlewares: indexmap!{},
            action_handlers: vec![],
            action_map: hashmap!{},
            custom_action_declarations: hashmap!{},
            constants: hashmap!{},
        }
    }

    pub fn middleware<F>(&mut self, name: &'static str, f: F) -> Result<()> where
        F: Middleware + 'static,
    {
        self.add_middleware(name, f)
    }

    pub fn action<T, F>(&mut self, group: &'static str, name: &'static str, f: F) -> Result<()> where
        T: 'static,
        F: ActionCtxArgument<T> + 'static,
    {
        self.add_action_handler(group, name, f)
    }

    pub(crate) fn datasets(&self) -> &Vec<DataSet> {
        &self.datasets
    }

    pub(crate) fn datasets_mut(&mut self) -> &mut Vec<DataSet> {
        &mut self.datasets
    }

    pub(crate) fn clients(&self) -> &Vec<ClientConf> {
        &self.clients
    }

    pub(crate) fn clients_mut(&mut self) -> &mut Vec<ClientConf> {
        &mut self.clients
    }

    pub(crate) fn entities(&self) -> &Vec<EntityConf> {
        &self.entities
    }

    pub(crate) fn entities_mut(&mut self) -> &mut Vec<EntityConf> {
        &mut self.entities
    }

    pub(crate) fn set_server_conf(&mut self, server_conf: Box<ServerConf>) {
        self.server_conf = Some(server_conf);
    }

    pub(crate) fn server_conf(&self) -> Option<&ServerConf> {
        self.server_conf.map(|c| c.as_ref())
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

    pub(crate) fn set_debug_conf(&mut self, debug_conf: Box<DebugConf>) {
        self.debug_conf = Some(debug_conf);
    }

    pub(crate) fn debug_conf(&self) -> Option<&DebugConf> {
        self.debug_conf.as_ref().map(|c| c.as_ref())
    }

    pub(crate) fn set_test_conf(&mut self, test_conf: Box<TestConf>) {
        self.test_conf = Some(test_conf);
    }

    pub(crate) fn test_conf(&self) -> Option<&TestConf> {
        self.test_conf.as_ref().map(|c| c.as_ref())
    }

    pub(crate) fn set_connector_conf(&mut self, connector_conf: Box<ConnectorConf>) {
        self.connector_conf = Some(connector_conf);
    }

    pub(crate) fn connector_conf(&self) -> Option<&ConnectorConf> {
        self.connector_conf.map(|c| c.as_ref())
    }

    pub(crate) fn add_middleware<F>(&mut self, name: &'static str, f: F) -> Result<()> where
        F: Middleware + 'static,
    {
        self.middlewares.insert(name, Box::leak(Box::new(f)));
        Ok(())
    }

    pub(crate) fn add_custom_action_declaration(&mut self, group: &'static str, name: &'static str, dec: CustomActionDefinition) -> Result<()> {
        if !self.custom_action_declarations.contains_key(group) {
            self.custom_action_declarations.insert(group, HashMap::new());
        }
        let name_map = self.custom_action_declarations.get_mut(group).unwrap();
        name_map.insert(name, dec);
        Ok(())
    }

    pub(crate) fn add_action_handler<A: 'static, F>(&mut self, group: &'static str, name: &'static str, f: F) -> Result<()> where
        F: ActionCtxArgument<A> + 'static,
    {
        let handler_def = Box::leak(Box::new(ActionHandlerDef {
            group, name, f: Arc::new(f),
        }));
        self.action_handlers.push(handler_def);
        if !self.action_map.contains_key(group) {
            self.action_map.insert(group, HashMap::new());
        }
        let name_map = self.action_map.get_mut(group).unwrap();
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

    pub(crate) fn has_child_namespace(&self, name: &'static str) -> bool {
        self.namespaces.contains_key(name)
    }

    pub(crate) fn child_namespace(&mut self, name: &'static str) -> &mut Namespace {
        if !self.has_child_namespace(name) {
            self.namespaces.insert(name, Namespace::new());
        }
        self.namespaces.get_mut(name).unwrap()
    }

    pub(crate) fn add_enum(&mut self, e: Enum) -> Result<()> {
        self.enums.insert(e.name, e);
        Ok(())
    }

    pub(crate) fn add_model(&mut self, m: Model, name: &'static str) -> Result<()> {
        self.models.insert(name, m);
        Ok(())
    }

    pub(crate) fn r#enum(&self, name: &'static str) -> Option<&Enum> {
        self.enums().get(name)
    }

    pub(crate) fn model(&self, name: &'static str) -> Option<&'static Model> {
        self.models.get(name)
    }

    pub(crate) fn enum_mut(&mut self, name: &'static str) -> Option<&mut Enum> {
        self.enums_mut().get_mut(name)
    }

    pub(crate) fn model_mut(&mut self, name: &'static str) -> Option<&mut Model> {
        self.models_mut().get_mut(name)
    }
}