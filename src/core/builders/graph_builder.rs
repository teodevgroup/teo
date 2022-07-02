use std::collections::HashMap;
use std::sync::Arc;
use crate::core::builders::client_builder::ClientBuilder;
use crate::core::builders::data_source_builder::DataSourceBuilder;
use crate::core::builders::enum_builder::EnumBuilder;
use crate::core::connector::{ConnectorBuilder};
use crate::core::builders::model_builder::ModelBuilder;
use crate::core::client::Client;
use crate::core::r#enum::Enum;


pub struct GraphBuilder {
    pub(crate) enum_builders: HashMap<String, EnumBuilder>,
    pub(crate) models: Vec<ModelBuilder>,
    pub(crate) connector_builder: Option<Box<dyn ConnectorBuilder>>,
    pub(crate) reset_database: bool,
    pub(crate) jwt_secret: String,
    pub(crate) clients: Vec<Arc<dyn Client>>,
    pub(crate) host_url: Option<String>,
    pub(crate) url_prefix: Option<String>,
}

impl GraphBuilder {

    pub(crate) fn new() -> Self {
        GraphBuilder {
            enum_builders: HashMap::new(),
            models: Vec::new(),
            connector_builder: None,
            reset_database: false,
            jwt_secret: "".to_string(),
            clients: Vec::new(),
            host_url: None,
            url_prefix: None,
        }
    }

    pub fn r#enum<F: Fn(&mut EnumBuilder)>(&mut self, name: impl Into<String>, build: F) -> &mut Self {
        let name = name.into();
        let mut enum_builder = EnumBuilder::new(name.clone());
        build(&mut enum_builder);
        self.enum_builders.insert(name.clone(), enum_builder);
        self
    }

    pub fn model<F: Fn(&mut ModelBuilder)>(&mut self, name: &'static str, build: F) -> &mut Self {
        let mut model: ModelBuilder = ModelBuilder::new(name, self.connector_builder());
        build(&mut model);
        self.models.push(model);
        self
    }

    pub fn reset_database(&mut self) -> &mut Self {
        self.reset_database = true;
        self
    }

    pub fn jwt_secret(&mut self, secret: impl Into<String>) -> &mut Self {
        self.jwt_secret = secret.into();
        self
    }

    pub fn url_prefix(&mut self, prefix: impl Into<String>) -> &mut Self {
        self.url_prefix = Some(prefix.into());
        self
    }

    pub fn host_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.host_url = Some(url.into());
        self
    }

    pub fn data_source(&mut self) -> DataSourceBuilder {
        DataSourceBuilder { graph_builder: self }
    }

    pub fn client(&mut self) -> ClientBuilder {
        ClientBuilder { graph_builder: self }
    }

    pub(crate) fn connector_builder(&self) -> &Box<dyn ConnectorBuilder> {
        match &self.connector_builder {
            Some(connector_builder) => connector_builder,
            None => panic!("Graph doesn't have a database connector.")
        }
    }

    pub(crate) fn build_enums(&self) -> HashMap<String, Enum> {
        let mut retval: HashMap<String, Enum> = HashMap::new();
        for (k, v) in &self.enum_builders {
            retval.insert(k.clone(), v.into());
        }
        retval
    }
}
