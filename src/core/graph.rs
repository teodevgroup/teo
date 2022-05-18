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
    pub(crate) enums: RwLock<HashMap<&'static str, Vec<&'static str>>>,
    pub(crate) models: RwLock<Vec<Arc<Model>>>,
    pub(crate) models_map: RwLock<HashMap<&'static str, Arc<Model>>>,
    pub(crate) connector: Option<RwLock<Arc<dyn Connector>>>,
}

impl Graph {

    pub fn new<F: Fn(&mut GraphBuilder)>(build: F) -> Arc<Graph> {
        let mut builder = GraphBuilder::new();
        build(&mut builder);
        let mut graph = Arc::new(Graph {
            enums: RwLock::new(HashMap::new()),
            models: RwLock::new(Vec::new()),
            models_map: RwLock::new(HashMap::new()),
            connector: None
        });
        let addr = addr_of!(*graph);
        let models: Vec<Arc<Model>> = builder.models.iter().map(move |mb| Arc::new(Model::new(mb, addr))).collect();
        let mut mut_graph = Arc::get_mut(&mut graph).unwrap();
        mut_graph.enums = RwLock::new(builder.enums.clone());
        mut_graph.models = RwLock::new(models);
        mut_graph.connector = Some(RwLock::new(builder.connector().clone()));
        let mut map: HashMap<&'static str, Arc<Model>> = HashMap::new();
        for model in mut_graph.models.read().unwrap().iter() {
            map.insert(model.name, model.clone());
        }
        mut_graph.models_map = RwLock::new(map);
        return graph;
    }

    pub(crate) fn model(&self, name: &str) -> Arc<Model> {
        self.models_map.read().unwrap().get(name).unwrap().clone()
    }

    pub(crate) fn r#enum(&self, name: &str) -> Vec<&'static str> {
        self.enums.read().unwrap().get(name).unwrap().clone()
    }

    pub(crate) async fn connect(&self) {
        match &self.connector {
            Some(connector) => {
                connector.read().unwrap().clone().connect().await;
                connector.read().unwrap().clone().sync_graph(self).await;
            }
            None => {
                panic!();
            }
        }
    }

    pub fn new_object(&self, model: &'static str) -> Arc<Object> {
        Object::new(self.model(model))
    }

    async fn find_unique(&self, model_name: &'static str, finder: JsonValue) -> Arc<Object> {
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

    async fn find_one(&self, model_name: &'static str, finder: JsonValue) -> Arc<Object> {
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

    async fn find_many(&self, model_name: &'static str, finder: JsonValue) -> Vec<Arc<Object>> {
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

unsafe impl Send for Graph {}
unsafe impl Sync for Graph {}
