use std::collections::HashMap;
use std::ptr::addr_of;
use std::sync::Arc;
use actix_http::{Method};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use chrono::{Duration, Utc};
use futures_util::StreamExt;
use serde_json::{json, Map, Value as JsonValue};
use crate::action::action::ActionType;
use crate::core::builders::graph_builder::GraphBuilder;
use crate::core::client::Client;
use crate::core::connector::Connector;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::stage::Stage;
use crate::error::ActionError;
use crate::server::jwt::{Claims, decode_token, encode_token};


#[derive(Debug)]
pub struct Graph {
    enums: HashMap<&'static str, Vec<&'static str>>,
    models_vec: Vec<Model>,
    models_map: HashMap<&'static str, * const Model>,
    url_segment_name_map: HashMap<String, &'static str>,
    connector: Option<Box<dyn Connector>>,
    jwt_secret: &'static str,
    host_url: &'static str,
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
            enums: builder.enums.clone(),
            models_vec: Vec::new(),
            models_map: HashMap::new(),
            url_segment_name_map: HashMap::new(),
            connector: None,
            jwt_secret: builder.jwt_secret,
            host_url: builder.host_url.unwrap(),
            clients: builder.clients.clone(),
        };
        graph.models_vec = builder.models.iter().map(|mb| mb.build(&builder.connector_builder())).collect();
        let mut models_map: HashMap<&'static str, * const Model> = HashMap::new();
        let mut url_segment_name_map: HashMap<String, &'static str> = HashMap::new();
        for model in graph.models_vec.iter() {
            models_map.insert(model.name(), addr_of!(*model));
            url_segment_name_map.insert(model.url_segment_name().clone(), model.name());
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

    pub(crate) fn model(&self, name: &str) -> &Model {
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

    pub(crate) async fn find_unique(&'static self, model: &'static Model, finder: &Map<String, JsonValue>) -> Result<Object, ActionError> {
        self.connector().find_unique(self, model, finder).await
    }

    pub(crate) async fn find_first(&'static self, model: &'static Model, finder: &Map<String, JsonValue>) -> Result<Object, ActionError> {
        self.connector().find_first(self, model, finder).await
    }

    pub(crate) async fn find_many(&'static self, model: &'static Model, finder: &Map<String, JsonValue>) -> Result<Vec<Object>, ActionError> {
        self.connector().find_many(self, model, finder).await
    }

    pub(crate) async fn count(&'static self, model: &'static Model, finder: &Map<String, JsonValue>) -> Result<usize, ActionError> {
        self.connector().count(self, model, finder).await
    }

    pub fn new_object(&'static self, model: &'static str) -> Object {
        Object::new(self, self.model(model))
    }

    pub(crate) fn model_name_for_url_segment_name(&self, segment_name: &str) -> Option<&str> {
        match self.url_segment_name_map.get(segment_name) {
            Some(val) => Some(*val),
            None => None
        }
    }

    pub(crate) fn jwt_secret(&self) -> &'static str {
        return if self.jwt_secret == "" {
            panic!("A graph with identity must have a custom JWT secret.")
        } else {
            self.jwt_secret
        }
    }

    pub async fn generate_packages(&'static self) -> std::io::Result<()> {
        for client in &self.clients {
            client.generate(self).await?
        }
        Ok(())
    }
}

unsafe impl Send for Graph {}
unsafe impl Sync for Graph {}
