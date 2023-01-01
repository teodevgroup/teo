use std::collections::HashMap;
use std::sync::Arc;

use crate::core::data_source::builder::DataSourceBuilder;
use crate::core::r#enum::builder::EnumBuilder;
use crate::core::connector::{ConnectorBuilder};
use crate::core::graph::GraphInner;
use crate::core::model::builder::ModelBuilder;
use crate::core::model::Model;

use crate::core::r#enum::Enum;
use crate::prelude::Graph;

pub struct GraphBuilder {
    pub(crate) enum_builders: HashMap<String, EnumBuilder>,
    pub(crate) model_builders: Vec<ModelBuilder>,
    pub(crate) connector_builder: Option<Box<dyn ConnectorBuilder>>,
    pub(crate) reset_database: bool,
}

impl GraphBuilder {

    pub(crate) fn new() -> Self {
        GraphBuilder {
            enum_builders: HashMap::new(),
            model_builders: Vec::new(),
            connector_builder: None,
            reset_database: false,
        }
    }

    pub fn r#enum<F: Fn(&mut EnumBuilder)>(&mut self, name: impl Into<String>, build: F) -> &mut Self {
        let name = name.into();
        let mut enum_builder = EnumBuilder::new(name.clone());
        build(&mut enum_builder);
        self.enum_builders.insert(name.clone(), enum_builder);
        self
    }

    pub fn model<F: Fn(&mut ModelBuilder)>(&mut self, name: impl Into<String>, build: F) -> &mut Self {
        let mut model: ModelBuilder = ModelBuilder::new(name, self.connector_builder());
        build(&mut model);
        self.model_builders.push(model);
        self
    }

    pub fn reset_database(&mut self) -> &mut Self {
        self.reset_database = true;
        self
    }

    pub fn data_source(&mut self) -> DataSourceBuilder {
        DataSourceBuilder { graph_builder: self }
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

    pub(crate) async fn build(&self) -> Graph {
        let mut graph = GraphInner {
            enums: self.build_enums(),
            models_vec: Vec::new(),
            models_map: HashMap::new(),
            url_segment_name_map: HashMap::new(),
            connector: None,
        };
        graph.models_vec = self.model_builders.iter().map(|mb| { mb.build(&self.connector_builder()) }).collect();
        let mut models_map: HashMap<String, Model> = HashMap::new();
        let mut url_segment_name_map: HashMap<String, String> = HashMap::new();
        for model in graph.models_vec.iter() {
            models_map.insert(model.name().to_owned(), model.clone());
            url_segment_name_map.insert(model.url_segment_name().to_owned(), model.name().to_owned());
        }
        graph.models_map = models_map;
        graph.url_segment_name_map = url_segment_name_map;
        graph.connector = Some(self.connector_builder().build_connector(&graph.models_vec, self.reset_database).await);
        Graph { inner: Arc::new(graph) }
    }
}
