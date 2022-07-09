use std::collections::HashMap;
use std::sync::Arc;
use serde_json::{Value as JsonValue};
use crate::core::builders::graph_builder::GraphBuilder;

use crate::core::connector::Connector;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::r#enum::Enum;
use crate::error::ActionError;

#[derive(Clone)]
pub struct Graph {
    inner: Arc<GraphInner>
}

struct GraphInner {
    enums: HashMap<String, Enum>,
    models_vec: Vec<Model>,
    models_map: HashMap<String, Model>,
    url_segment_name_map: HashMap<String, String>,
    connector: Option<Box<dyn Connector>>,
}

impl Graph {

    pub async fn new<'a, F: Fn(&mut GraphBuilder)>(build: F) -> Graph {
        let mut builder = GraphBuilder::new();
        build(&mut builder);
        let mut graph = GraphInner {
            enums: builder.build_enums(),
            models_vec: Vec::new(),
            models_map: HashMap::new(),
            url_segment_name_map: HashMap::new(),
            connector: None,
        };
        graph.models_vec = builder.models.iter().map(|mb| { mb.build(&builder.connector_builder()) }).collect();
        let mut models_map: HashMap<String, Model> = HashMap::new();
        let mut url_segment_name_map: HashMap<String, String> = HashMap::new();
        for model in graph.models_vec.iter() {
            models_map.insert(model.name().to_owned(), model.clone());
            url_segment_name_map.insert(model.url_segment_name().to_owned(), model.name().to_owned());
        }
        graph.models_map = models_map;
        graph.url_segment_name_map = url_segment_name_map;
        graph.connector = Some(builder.connector_builder().build_connector(&graph.models_vec, builder.reset_database).await);
        Graph { inner: Arc::new(graph) }
    }

    pub async fn find_unique(&self, model: &str, finder: &JsonValue, mutation_mode: bool) -> Result<Object, ActionError> {
        let model = self.model(model)?;
        self.connector().find_unique(self, model, finder, mutation_mode).await
    }

    pub async fn find_first(&self, model: &str, finder: &JsonValue, mutation_mode: bool) -> Result<Object, ActionError> {
        let model = self.model(model)?;
        self.connector().find_first(self, model, finder, mutation_mode).await
    }

    pub async fn find_many(&self, model: &str, finder: &JsonValue, mutation_mode: bool) -> Result<Vec<Object>, ActionError> {
        let model = self.model(model)?;
        self.connector().find_many(self, model, finder, mutation_mode).await
    }

    pub async fn count(&self, model: &str, finder: &JsonValue) -> Result<usize, ActionError> {
        let model = self.model(model)?;
        self.connector().count(self, model, finder).await
    }

    pub fn create_object(&self, model: &str) -> Result<Object, ActionError> {
        match self.model(model) {
            Ok(model) => Ok(Object::new(self, model)),
            Err(err) => Err(err)
        }
    }

    pub(crate) fn connector(&self) -> &dyn Connector {
        match &self.inner.connector {
            Some(c) => { c.as_ref() }
            None => { panic!() }
        }
    }

    pub(crate) fn model(&self, name: &str) -> Result<&Model, ActionError> {
        match self.inner.models_map.get(name) {
            Some(model) => Ok(model),
            None => Err(ActionError::model_not_found(name))
        }
    }

    pub(crate) fn r#enum(&self, name: impl Into<String>) -> &Vec<String> {
        &self.inner.enums.get(&name.into()).unwrap().values
    }

    pub(crate) fn models(&self) -> &Vec<Model> { &self.inner.models_vec }

    pub(crate) fn enums(&self) -> &HashMap<String, Enum> { &self.inner.enums }

    pub(crate) fn model_name_for_url_segment_name(&self, segment_name: &str) -> Option<&String> {
        match self.inner.url_segment_name_map.get(segment_name) {
            Some(val) => Some(val),
            None => None
        }
    }
}

unsafe impl Send for Graph { }
unsafe impl Sync for Graph { }
