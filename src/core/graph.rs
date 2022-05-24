use std::collections::HashMap;
use std::ptr::addr_of;
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_utils::future::ok;
use serde_json::{json, Value as JsonValue};
use crate::core::builders::GraphBuilder;
use crate::core::connector::Connector;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::error::ActionError;


#[derive(Debug)]
pub struct Graph {
    enums: HashMap<&'static str, Vec<&'static str>>,
    models_vec: Vec<Model>,
    models_map: HashMap<&'static str, * const Model>,
    url_segment_name_map: HashMap<String, &'static str>,
    connector: Option<Box<dyn Connector>>,
}

impl Graph {

    pub async fn new<'a, F: Fn(&mut GraphBuilder)>(build: F) -> Graph {
        let mut builder = GraphBuilder::new();
        build(&mut builder);
        let mut graph = Graph {
            enums: builder.enums.clone(),
            models_vec: Vec::new(),
            models_map: HashMap::new(),
            url_segment_name_map: HashMap::new(),
            connector: None
        };
        graph.models_vec = builder.models.iter().map(move |mb| Model::new(mb)).collect();
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

    pub(crate) async fn find_unique(&'static self, model_name: &'static str, finder: JsonValue) -> Option<Object> {
        self.connector().find_unique(self.model(model_name), finder).await
    }

    pub(crate) async fn find_one(&'static self, model_name: &'static str, finder: JsonValue) -> Option<Object> {
        self.connector().find_one(self.model(model_name), finder).await
    }

    pub(crate) async fn find_many(&'static self, model_name: &'static str, finder: JsonValue) -> Vec<Object> {
        self.connector().find_many(self.model(model_name), finder).await
    }

    pub fn new_object(&'static self, model: &'static str) -> Object {
        Object::new(self, self.model(model))
    }

    pub async fn start_server(&'static self, port: u16) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .default_service(|r: ServiceRequest| {
                    let path = r.path();
                    if path.len() > 7 && path.ends_with("/action") {
                        let model_url_segment_name = &path[1..path.len() - 7];
                        match self.model_name_for_url_segment_name(model_url_segment_name) {
                            Some(model_name) => {
                                let http_response = HttpResponse::Ok().json(json!({"hello": "world!"}));
                                ok(r.into_response(http_response))
                            }
                            None => {
                                let http_response = HttpResponse::NotFound().json(json!({"error": ActionError::not_found()}));
                                ok(r.into_response(http_response))
                            }
                        }
                    } else {
                        let http_response = HttpResponse::NotFound().json(json!({"error": ActionError::not_found()}));
                        ok(r.into_response(http_response))
                    }
                })
        })
            .bind(("127.0.0.1", port))
            .unwrap()
            .run()
            .await
    }

    fn model_name_for_url_segment_name(&self, segment_name: &str) -> Option<&str> {
        match self.url_segment_name_map.get(segment_name) {
            Some(val) => Some(*val),
            None => None
        }
    }
}

unsafe impl Send for Graph {}
unsafe impl Sync for Graph {}
