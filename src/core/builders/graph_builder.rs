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
    pub(crate) jwt_secret: String,
    pub(crate) clients: Vec<Arc<dyn Client>>,
    pub(crate) host_url: Option<String>,
}

impl GraphBuilder {

    pub(crate) fn new() -> Self {
        GraphBuilder {
            enums: HashMap::new(),
            models: Vec::new(),
            connector_builder: None,
            reset_database: false,
            jwt_secret: "".to_string(),
            clients: Vec::new(),
            host_url: None
        }
    }

    pub fn r#enum<I, S, N>(&mut self, name: N, values: I) -> &mut Self where I: IntoIterator<Item = S>, S: Into<String>, N: Into<String> {
        self.enums.insert(name.into(), values.into_iter().map(Into::into).collect());
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
}
