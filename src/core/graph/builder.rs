use std::collections::HashMap;
use std::sync::Arc;
use crate::core::connector::Connector;
use crate::core::r#enum::builder::EnumBuilder;
use crate::core::graph::GraphInner;
use crate::core::model::builder::ModelBuilder;
use crate::core::model::Model;
use crate::core::r#enum::Enum;
use crate::prelude::Graph;

pub struct GraphBuilder {
    pub(crate) enum_builders: HashMap<String, EnumBuilder>,
    pub(crate) model_builders: Vec<ModelBuilder>,
    pub(crate) reset_database: bool,
}

impl GraphBuilder {

    pub(crate) fn new() -> Self {
        GraphBuilder {
            enum_builders: HashMap::new(),
            model_builders: Vec::new(),
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
        let mut model: ModelBuilder = ModelBuilder::new(name);
        build(&mut model);
        self.model_builders.push(model);
        self
    }

    pub fn reset_database(&mut self) -> &mut Self {
        self.reset_database = true;
        self
    }

    pub(crate) fn build_enums(&self) -> HashMap<String, Enum> {
        let mut retval: HashMap<String, Enum> = HashMap::new();
        for (k, v) in &self.enum_builders {
            retval.insert(k.clone(), v.into());
        }
        retval
    }

    pub(crate) async fn build(&self, connector: Arc<dyn Connector>) -> Graph {
        let mut graph = GraphInner {
            enums: self.build_enums(),
            models_vec: Vec::new(),
            models_map: HashMap::new(),
            url_segment_name_map: HashMap::new(),
            connector: None,
        };
        graph.models_vec = self.model_builders.iter().map(|mb| { mb.build(connector.clone()) }).collect();
        let mut models_map: HashMap<String, Model> = HashMap::new();
        let mut url_segment_name_map: HashMap<String, String> = HashMap::new();
        for model in graph.models_vec.iter() {
            models_map.insert(model.name().to_owned(), model.clone());
            url_segment_name_map.insert(model.url_segment_name().to_owned(), model.name().to_owned());
        }
        graph.models_map = models_map;
        graph.url_segment_name_map = url_segment_name_map;
        // TODO: disabled for Node.js, enable for MongoDB in the future
        // let c = connector.as_ref();
        // let connector_mut = unsafe {
        //     let d: * const dyn Connector = c;
        //     let e: * mut dyn Connector = d as *mut dyn Connector;
        //     &mut *e
        // };
        // let _ = connector_mut.load(&graph.models_vec).await;
        graph.connector = Some(connector.clone());
        Graph { inner: Arc::new(graph) }
    }
}
