use std::collections::HashMap;
use std::ptr::addr_of;
use std::sync::Arc;
use serde_json::{Map, Value as JsonValue};
use crate::core::builders::graph_builder::GraphBuilder;
use crate::core::client::Client;
use crate::core::connector::Connector;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::r#enum::Enum;
use crate::error::ActionError;


pub struct Graph {
    enums: HashMap<String, Enum>,
    models_vec: Vec<Model>,
    models_map: HashMap<String, * const Model>,
    url_segment_name_map: HashMap<String, String>,
    connector: Option<Box<dyn Connector>>,
    jwt_secret: String,
    host_url: String,
    clients: Vec<Arc<dyn Client>>,
}

impl Graph {

    pub async fn new<'a, F: Fn(&mut GraphBuilder)>(build: F) -> Graph {
        let mut builder = GraphBuilder::new();
        build(&mut builder);
        if builder.host_url.is_none() {
            panic!("Graph must have a host url.");
        }
        let mut graph = Graph {
            enums: builder.build_enums(),
            models_vec: Vec::new(),
            models_map: HashMap::new(),
            url_segment_name_map: HashMap::new(),
            connector: None,
            jwt_secret: builder.jwt_secret.clone(),
            host_url: builder.host_url.as_ref().unwrap().clone(),
            clients: builder.clients.clone(),
        };
        graph.models_vec = builder.models.iter().map(|mb| unsafe { mb.build(&builder.connector_builder()) }).collect();
        let mut models_map: HashMap<String, * const Model> = HashMap::new();
        let mut url_segment_name_map: HashMap<String, String> = HashMap::new();
        for model in graph.models_vec.iter() {
            models_map.insert(model.name().clone(), addr_of!(*model));
            url_segment_name_map.insert(model.url_segment_name().clone(), model.name().clone());
        }
        graph.models_map = models_map;
        graph.url_segment_name_map = url_segment_name_map;
        graph.connector = Some(builder.connector_builder().build_connector(&graph.models_vec, builder.reset_database).await);
        graph
    }

    pub(crate) fn connector(&self) -> &dyn Connector {
        match &self.connector {
            Some(c) => { c.as_ref() }
            None => { panic!() }
        }
    }

    pub fn model(&self, name: &str) -> &Model {
        unsafe {
            &(**self.models_map.get(name).unwrap())
        }
    }

    pub(crate) fn r#enum(&self, name: impl Into<String>) -> &Vec<String> {
        &self.enums.get(&name.into()).unwrap().values
    }

    pub(crate) fn models(&self) -> &Vec<Model> { &self.models_vec }

    pub(crate) fn enums(&self) -> &HashMap<String, Enum> { &self.enums }

    pub async fn find_unique(&self, model: &Model, finder: &JsonValue, mutation_mode: bool) -> Result<Object, ActionError> {
        self.connector().find_unique(self, model, finder, mutation_mode).await
    }

    pub async fn find_first(&self, model: &Model, finder: &JsonValue, mutation_mode: bool) -> Result<Object, ActionError> {
        self.connector().find_first(self, model, finder, mutation_mode).await
    }

    pub async fn find_many(&self, model: &Model, finder: &JsonValue, mutation_mode: bool) -> Result<Vec<Object>, ActionError> {
        self.connector().find_many(self, model, finder, mutation_mode).await
    }

    pub async fn count(&self, model: &Model, finder: &JsonValue) -> Result<usize, ActionError> {
        self.connector().count(self, model, finder).await
    }

    pub fn new_object(&self, model: &str) -> Object {
        Object::new(self, self.model(model))
    }

    pub(crate) fn model_name_for_url_segment_name(&self, segment_name: &str) -> Option<&String> {
        match self.url_segment_name_map.get(segment_name) {
            Some(val) => Some(val),
            None => None
        }
    }

    pub(crate) fn jwt_secret(&self) -> &str {
        return if self.jwt_secret == "" {
            panic!("A graph with identity must have a custom JWT secret.")
        } else {
            &self.jwt_secret
        }
    }

    pub async fn generate_packages(&'static self) -> std::io::Result<()> {
        for client in &self.clients {
            client.generate(self).await?
        }
        Ok(())
    }

    pub(crate) fn host_url(&'static self) -> &str {
        &self.host_url
    }
}

unsafe impl Send for Graph {}
unsafe impl Sync for Graph {}
