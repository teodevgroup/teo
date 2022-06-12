use std::collections::HashMap;
use std::sync::Arc;
use crate::core::builders::client_builder::ClientBuilder;
use crate::core::builders::data_source_builder::DataSourceBuilder;
use crate::core::connector::{ConnectorBuilder};
use crate::core::builders::model_builder::ModelBuilder;
use crate::core::client::Client;


pub struct GraphBuilder {
    pub(crate) enums: HashMap<String, Vec<String>>,
    pub(crate) models: Vec<ModelBuilder>,
    pub(crate) connector_builder: Option<Box<dyn ConnectorBuilder>>,
    pub(crate) reset_database: bool,
    pub(crate) jwt_secret: &'static str,
    pub(crate) clients: Vec<Arc<dyn Client>>,
    pub(crate) host_url: Option<&'static str>,
}

impl GraphBuilder {

    pub(crate) fn new() -> GraphBuilder {
        GraphBuilder {
            enums: HashMap::new(),
            models: Vec::new(),
            connector_builder: None,
            reset_database: false,
            jwt_secret: "",
            clients: Vec::new(),
            host_url: None
        }
    }

    pub(crate) fn connector_builder(&self) -> &Box<dyn ConnectorBuilder> {
        match &self.connector_builder {
            Some(connector_builder) => connector_builder,
            None => panic!("Graph doesn't have a database connector.")
        }
    }

    pub fn r#enum<I, S, N>(&mut self, name: N, values: I) where I: IntoIterator<Item = S>, S: Into<String>, N: Into<String> {
        self.enums.insert(name.into(), values.into_iter().map(Into::into).collect());
    }

    pub fn model<F: Fn(&mut ModelBuilder)>(&mut self, name: &'static str, build: F) {
        let mut model: ModelBuilder = ModelBuilder::new(name, self.connector_builder());
        build(&mut model);
        self.models.push(model);
    }

    pub fn reset_database(&mut self) {
        self.reset_database = true;
    }

    pub fn jwt_secret(&mut self, secret: &'static str) {
        self.jwt_secret = secret;
    }

    pub fn data_source(&mut self) -> DataSourceBuilder {
        DataSourceBuilder { graph_builder: self }
    }

    pub fn client(&mut self) -> ClientBuilder {
        ClientBuilder { graph_builder: self }
    }

    pub fn host_url(&mut self, url: &'static str) -> &mut Self {
        self.host_url = Some(url);
        self
    }
}
