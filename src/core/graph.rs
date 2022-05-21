use std::collections::HashMap;
use std::ptr::addr_of;
use std::sync::{Arc};
use serde_json::{Value as JsonValue};
use crate::core::builders::GraphBuilder;
use crate::core::connector::Connector;
use crate::core::model::Model;
use crate::core::object::Object;


#[derive(Debug)]
pub struct Graph {
    enums: HashMap<&'static str, Vec<&'static str>>,
    models_vec: Vec<Model>,
    models_map: HashMap<&'static str, * const Model>,
    connector: Arc<dyn Connector>,
}

impl Graph {

    pub fn new<'a, F: Fn(&mut GraphBuilder)>(build: F) -> Graph {
        let mut builder = GraphBuilder::new();
        build(&mut builder);
        let mut graph = Graph {
            enums: HashMap::new(),
            models_vec: Vec::new(),
            models_map: HashMap::new(),
            connector: builder.connector().clone()
        };
        let models_vec: Vec<Model> = builder.models.iter().map(move |mb| Model::new(mb)).collect();
        graph.enums = builder.enums.clone();
        graph.models_vec = models_vec;
        let mut map: HashMap<&'static str, * const Model> = HashMap::new();
        for model in graph.models_vec.iter() {
            map.insert(model.name(), addr_of!(*model));
        }
        graph.models_map = map.clone();
        graph
    }

    pub(crate) fn model(&'static self, name: &str) -> &'static Model {
        unsafe {
            &(**self.models_map.get(name).unwrap())
        }
    }

    pub(crate) fn r#enum(&self, name: &str) -> &Vec<&'static str> {
        &self.enums.get(name).unwrap()
    }

    pub(crate) fn models(&'static self) -> &'static Vec<Model> {
        &self.models_vec
    }

    pub(crate) fn enums(&'static self) -> &'static HashMap<&'static str, Vec<&'static str>> {
        &self.enums
    }

    pub fn new_object(&'static self, model: &'static str) -> Object {
        Object::new(self, self.model(model))
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

    pub async fn drop_database(&self) {
        self.connector.drop_database().await;
    }

    pub(crate) async fn connect(&self) {
        self.connector.sync_graph(self).await
    }

    pub(crate) fn connector(&self) -> &Arc<dyn Connector> {
        &self.connector
    }
}

unsafe impl Send for Graph {}
unsafe impl Sync for Graph {}
