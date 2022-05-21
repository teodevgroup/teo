use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::ptr::addr_of;
use serde_json::{Value as JsonValue};
use crate::core::builders::GraphBuilder;
use crate::core::connector::Connector;
use crate::core::model::Model;
use crate::core::object::Object;


#[derive(Debug)]
pub struct Graph {
    inner: Arc<GraphInner>
}

impl Graph {

    pub fn new<F: Fn(&mut GraphBuilder)>(build: F) -> Graph {
        Graph { inner: GraphInner::new(build) }
    }

    pub fn new_object(&'static self, model: &'static str) -> Object {
        Object::new(self.model(model), self)
    }

    pub async fn find_unique(&'static self, model_name: &'static str, finder: JsonValue) -> Option<Object> {
        self.inner.find_unique(model_name, finder).await
    }

    pub async fn find_one(&'static self, model_name: &'static str, finder: JsonValue) -> Option<Object> {
        self.inner.find_one(model_name, finder).await
    }

    pub async fn find_many(&'static self, model_name: &'static str, finder: JsonValue) -> Vec<Object> {
        self.inner.find_many(model_name, finder).await
    }

    pub(crate) fn model(&'static self, name: &str) -> &'static Model {
        self.inner.model(name)
    }

    pub(crate) fn r#enum(&self, name: &str) -> Vec<&'static str> {
        self.inner.r#enum(name)
    }

    pub(crate) async fn connect(&self) {
        self.inner.connector.sync_graph(self).await
    }

    pub async fn drop_database(&self) {
        self.inner.drop_database().await;
    }

    pub(crate) fn connector(&self) -> &Arc<dyn Connector> {
        &self.inner.connector
    }

    pub(crate) fn enums(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &self.inner.enums
    }
}

#[derive(Debug)]
pub(crate) struct GraphInner {
    pub(crate) enums: HashMap<&'static str, Vec<&'static str>>,
    pub(crate) models: Vec<Model>,
    pub(crate) models_map: HashMap<&'static str, Model>,
    pub(crate) connector: Arc<dyn Connector>,
}

impl GraphInner {

    pub(crate) fn new<F: Fn(&mut GraphBuilder)>(build: F) -> Arc<GraphInner> {
        let mut builder = GraphBuilder::new();
        build(&mut builder);
        let mut graph_inner = Arc::new(GraphInner {
            enums: HashMap::new(),
            models: Vec::new(),
            models_map: HashMap::new(),
            connector: builder.connector().clone()
        });
        let addr = addr_of!(*graph_inner);
        let models: Vec<Model> = builder.models.iter().map(move |mb| Model::new(mb)).collect();
        let mut mut_graph = Arc::get_mut(&mut graph_inner).unwrap();
        mut_graph.enums = builder.enums.clone();
        mut_graph.models = models;
        let mut map: HashMap<&'static str, Model> = HashMap::new();
        for model in mut_graph.models.iter() {
            map.insert(model.name(), model.clone());
        }
        mut_graph.models_map = map;
        return graph_inner;
    }

    pub(crate) fn model(&'static self, name: &str) -> &'static Model {
        self.models_map.get(name).unwrap()
    }

    pub(crate) fn r#enum(&self, name: &str) -> Vec<&'static str> {
        self.enums.get(name).unwrap().clone()
    }

    pub(crate) async fn find_unique(&'static self, model_name: &'static str, finder: JsonValue) -> Option<Object> {
        self.connector.find_unique(self.model(model_name), finder).await
    }

    pub(crate) async fn find_one(&'static self, model_name: &'static str, finder: JsonValue) -> Option<Object> {
        self.connector.find_one(self.model(model_name), finder).await
    }

    pub(crate) async fn find_many(&'static self, model_name: &'static str, finder: JsonValue) -> Vec<Object> {
        self.connector.find_many(self.model(model_name), finder).await
    }

    pub(crate) async fn drop_database(&self) {
        self.connector.drop_database().await;
    }

    pub(crate) fn connector(&self) -> Arc<dyn Connector> {
        self.connector.clone()
    }
}

unsafe impl Send for GraphInner {}
unsafe impl Sync for GraphInner {}
