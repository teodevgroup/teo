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

    pub fn new_object(&self, model: &'static str) -> Object {
        self.inner.new_object(model)
    }

    pub async fn find_unique(&self, model_name: &'static str, finder: JsonValue) -> Object {
        self.inner.find_unique(model_name, finder).await
    }

    pub async fn find_one(&self, model_name: &'static str, finder: JsonValue) -> Object {
        self.inner.find_one(model_name, finder).await
    }

    pub async fn find_many(&self, model_name: &'static str, finder: JsonValue) -> Vec<Object> {
        self.inner.find_many(model_name, finder).await
    }

    pub(crate) fn model(&self, name: &str) -> Arc<Model> {
        self.inner.model(name)
    }

    pub(crate) fn r#enum(&self, name: &str) -> Vec<&'static str> {
        self.inner.r#enum(name)
    }

    pub(crate) async fn connect(&self) {
        self.inner.clone().connect().await
    }
}

#[derive(Debug)]
pub(crate) struct GraphInner {
    pub(crate) enums: RwLock<HashMap<&'static str, Vec<&'static str>>>,
    pub(crate) models: RwLock<Vec<Arc<Model>>>,
    pub(crate) models_map: RwLock<HashMap<&'static str, Arc<Model>>>,
    pub(crate) connector: Option<RwLock<Arc<dyn Connector>>>,
}

impl GraphInner {

    pub(crate) fn new<F: Fn(&mut GraphBuilder)>(build: F) -> Arc<GraphInner> {
        let mut builder = GraphBuilder::new();
        build(&mut builder);
        let mut GraphInner = Arc::new(GraphInner {
            enums: RwLock::new(HashMap::new()),
            models: RwLock::new(Vec::new()),
            models_map: RwLock::new(HashMap::new()),
            connector: None
        });
        let addr = addr_of!(*GraphInner);
        let models: Vec<Arc<Model>> = builder.models.iter().map(move |mb| Arc::new(Model::new(mb, addr))).collect();
        let mut mut_graph = Arc::get_mut(&mut GraphInner).unwrap();
        mut_graph.enums = RwLock::new(builder.enums.clone());
        mut_graph.models = RwLock::new(models);
        mut_graph.connector = Some(RwLock::new(builder.connector().clone()));
        let mut map: HashMap<&'static str, Arc<Model>> = HashMap::new();
        for model in mut_graph.models.read().unwrap().iter() {
            map.insert(model.name, model.clone());
        }
        mut_graph.models_map = RwLock::new(map);
        return GraphInner;
    }

    pub(crate) fn model(&self, name: &str) -> Arc<Model> {
        self.models_map.read().unwrap().get(name).unwrap().clone()
    }

    pub(crate) fn r#enum(&self, name: &str) -> Vec<&'static str> {
        self.enums.read().unwrap().get(name).unwrap().clone()
    }

    pub(crate) async fn connect(self: Arc<Self>) {
        match &self.connector {
            Some(connector) => {
                connector.read().unwrap().clone().connect().await;
                connector.read().unwrap().clone().sync_graph(self.clone()).await;
            }
            None => {
                panic!();
            }
        }
    }

    pub(crate) fn new_object(&self, model: &'static str) -> Object {
        Object::new(self.model(model))
    }

    pub(crate) async fn find_unique(&self, model_name: &'static str, finder: JsonValue) -> Object {
        let model = &self.model(model_name);
        match &self.connector {
            Some(connector) => {
                connector.read().unwrap().clone().find_unique(model, finder).await
            }
            None => {
                panic!()
            }
        }
    }

    pub(crate) async fn find_one(&self, model_name: &'static str, finder: JsonValue) -> Object {
        let model = &self.model(model_name);
        match &self.connector {
            Some(connector) => {
                connector.read().unwrap().clone().find_one(model, finder).await
            }
            None => {
                panic!()
            }
        }
    }

    pub(crate) async fn find_many(&self, model_name: &'static str, finder: JsonValue) -> Vec<Object> {
        let model = &self.model(model_name);
        match &self.connector {
            Some(connector) => {
                connector.read().unwrap().clone().find_many(model, finder).await
            }
            None => {
                panic!()
            }
        }
    }
}

unsafe impl Send for GraphInner {}
unsafe impl Sync for GraphInner {}
