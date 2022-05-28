use std::collections::HashMap;
use std::ptr::addr_of;
use actix_http::{Method};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use actix_utils::future::ok;
use futures_util::StreamExt;
use serde_json::{json, Map, Value as JsonValue};
use crate::action::action::ActionType;
use crate::core::builders::graph_builder::GraphBuilder;
use crate::core::connector::Connector;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::value::Value;
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

    pub async fn start_server(&'static self, port: u16) -> std::io::Result<()> {
        HttpServer::new(|| {
            let this: &'static Graph = self;
            App::new()
                .default_service(web::route().to(move |r: HttpRequest, mut payload: web::Payload| async move {
                    let path = r.path();
                    if path.len() > 7 && path.ends_with("/action") {
                        let model_url_segment_name = &path[1..path.len() - 7];
                        match this.model_name_for_url_segment_name(model_url_segment_name) {
                            Some(model_name) => {
                                if r.method() == Method::POST {
                                    let mut body = web::BytesMut::new();
                                    while let Some(chunk) = payload.next().await {
                                        let chunk = chunk.unwrap();
                                        // limit max size of in-memory payload
                                        if (body.len() + chunk.len()) > 262_144usize {
                                            return HttpResponse::InternalServerError()
                                                .json(json!({"error": ActionError::internal_server_error("Memory overflow.".to_string())}));
                                        }
                                        body.extend_from_slice(&chunk);
                                    }
                                    let parsed_body: Result<JsonValue, serde_json::Error> = serde_json::from_slice(&body);
                                    match parsed_body {
                                        Ok(json_body) => {
                                            match json_body.as_object() {
                                                Some(map) => {
                                                    let action_name = map.get("action");
                                                    match action_name {
                                                        Some(name) => {
                                                            match name.as_str() {
                                                                Some(name) => {
                                                                    let action = ActionType::from_str(name);
                                                                    match action {
                                                                        Some(action_type) => {
                                                                            let model_def = this.model(model_name);
                                                                            if model_def.has_action(action_type) {
                                                                                match action_type {
                                                                                    ActionType::FindUnique => {
                                                                                        return this.handle_find_unique(map, model_def).await;
                                                                                    }
                                                                                    ActionType::FindFirst => {
                                                                                        return this.handle_find_first(map, model_def).await;
                                                                                    }
                                                                                    ActionType::FindMany => {
                                                                                        return this.handle_find_many(map, model_def).await;
                                                                                    }
                                                                                    ActionType::Create => {
                                                                                        return this.handle_create(map, model_def).await;
                                                                                    }
                                                                                    ActionType::Update => {
                                                                                        return this.handle_update(map, model_def).await;
                                                                                    }
                                                                                    ActionType::Upsert => {
                                                                                        return this.handle_upsert(map, model_def).await;
                                                                                    }
                                                                                    ActionType::Delete => {
                                                                                        return this.handle_delete(map, model_def).await;
                                                                                    }
                                                                                    ActionType::CreateMany => {
                                                                                        return this.handle_create_many(map, model_def).await;
                                                                                    }
                                                                                    ActionType::UpdateMany => {
                                                                                        return this.handle_update_many(map, model_def).await;
                                                                                    }
                                                                                    ActionType::DeleteMany => {
                                                                                        return this.handle_delete_many(map, model_def).await;
                                                                                    }
                                                                                    ActionType::Count => {
                                                                                        return this.handle_count(map, model_def).await;
                                                                                    }
                                                                                    ActionType::Aggregate => {
                                                                                        return this.handle_aggregate(map, model_def).await;
                                                                                    }
                                                                                    ActionType::GroupBy => {
                                                                                        return this.handle_group_by(map, model_def).await;
                                                                                    }
                                                                                    ActionType::SignIn => {
                                                                                        return this.handle_sign_in(map, model_def).await;
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                return HttpResponse::BadRequest().json(json!({"error": ActionError::unallowed_action()}));
                                                                            }
                                                                        }
                                                                        None => {
                                                                            return HttpResponse::BadRequest().json(json!({"error": ActionError::undefined_action()}));
                                                                        }
                                                                    }
                                                                }
                                                                None => {
                                                                    return HttpResponse::BadRequest().json(json!({"error": ActionError::undefined_action()}));
                                                                }
                                                            }
                                                        }
                                                        None => {
                                                            return HttpResponse::BadRequest().json(json!({"error": ActionError::missing_action_name()}));
                                                        }
                                                    }
                                                }
                                                None => {
                                                    return HttpResponse::BadRequest().json(json!({"error": ActionError::wrong_json_format()}));
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            return HttpResponse::BadRequest().json(json!({"error": ActionError::wrong_json_format()}));
                                        }
                                    }
                                } else {
                                    return HttpResponse::NotFound().json(json!({"error": ActionError::not_found()}));
                                }
                            }
                            None => {
                                return HttpResponse::NotFound().json(json!({"error": ActionError::not_found()}));
                            }
                        }
                    } else {
                        return HttpResponse::NotFound().json(json!({"error": ActionError::not_found()}));
                    }
                }))
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

    async fn handle_find_unique(&'static self, input: &Map<String, JsonValue>, model: &'static Model) -> HttpResponse {
        let result = self.find_unique(model, input).await;
        match result {
            Ok(obj) => {
                let json_data = obj.to_json();
                HttpResponse::Ok().json(json!({"data": json_data}))
            }
            Err(err) => {
                HttpResponse::NotFound().json(json!({"error": err}))
            }
        }
    }

    async fn handle_find_first(&'static self, input: &Map<String, JsonValue>, model: &'static Model) -> HttpResponse {
        let result = self.find_first(model, input).await;
        match result {
            Ok(obj) => {
                let json_data = obj.to_json();
                HttpResponse::Ok().json(json!({"data": json_data}))
            }
            Err(err) => {
                HttpResponse::NotFound().json(json!({"error": err}))
            }
        }
    }

    async fn handle_find_many(&'static self, input: &Map<String, JsonValue>, model: &'static Model) -> HttpResponse {
        let result = self.find_many(model, input).await;
        match result {
            Ok(results) => {
                let count = self.count(model, input).await.unwrap();
                let result_json: Vec<JsonValue> = results.iter().map(|i| { i.to_json() }).collect();
                HttpResponse::Ok().json(json!({
                    "meta": {"count": count},
                    "data": result_json
                }))
            }
            Err(err) => {
                HttpResponse::BadRequest().json(json!({
                    "error": err
                }))
            }
        }
    }

    async fn handle_create(&'static self, input: &Map<String, JsonValue>, model: &Model) -> HttpResponse {
        let create = input.get("create");
        let obj = self.new_object(model.name());
        let set_json_result = match create {
            Some(create) => {
                obj.set_json(create).await
            }
            None => {
                let empty = JsonValue::Object(Map::new());
                obj.set_json(&empty).await
            }
        };
        return match set_json_result {
            Ok(_) => {
                match obj.save().await {
                    Ok(_) => {
                        HttpResponse::Ok().json(json!({"data": obj.to_json()}))
                    }
                    Err(err) => {
                        HttpResponse::BadRequest().json(json!({"error": err}))
                    }
                }
            }
            Err(err) => {
                HttpResponse::BadRequest().json(json!({"error": err}))
            }
        }
    }

    async fn handle_update(&'static self, input: &Map<String, JsonValue>, model: &'static Model) -> HttpResponse {
        let result = self.find_unique(model, input).await;
        match result {
            Ok(obj) => {
                // find the object here
                let update = input.get("update");
                let set_json_result = match update {
                    Some(update) => {
                        obj.set_json(update).await
                    }
                    None => {
                        let empty = JsonValue::Object(Map::new());
                        obj.set_json(&empty).await
                    }
                };
                match set_json_result {
                    Ok(_) => {
                        match obj.save().await {
                            Ok(_) => {
                                HttpResponse::Ok().json(json!({"data": obj.to_json()}))
                            }
                            Err(err) => {
                                HttpResponse::BadRequest().json(json!({"error": err}))
                            }
                        }
                    }
                    Err(err) => {
                        return HttpResponse::BadRequest().json(json!({"error": err}));
                    }
                }
            }
            Err(err) => {
                return HttpResponse::NotFound().json(json!({"error": err}));
            }
        }
    }

    async fn handle_upsert(&'static self, input: &Map<String, JsonValue>, model: &'static Model) -> HttpResponse {
        let result = self.find_unique(model, input).await;
        match result {
            Ok(obj) => {
                // find the object here
                let update = input.get("update");
                let set_json_result = match update {
                    Some(update) => {
                        obj.set_json(update).await
                    }
                    None => {
                        let empty = JsonValue::Object(Map::new());
                        obj.set_json(&empty).await
                    }
                };
                match set_json_result {
                    Ok(_) => {
                        match obj.save().await {
                            Ok(_) => {
                                return HttpResponse::Ok().json(json!({"data": obj.to_json()}));
                            }
                            Err(err) => {
                                return HttpResponse::BadRequest().json(json!({"error": err}));
                            }
                        }
                    }
                    Err(err) => {
                        return HttpResponse::BadRequest().json(json!({"error": err}));
                    }
                }
            }
            Err(err) => {
                let create = input.get("create");
                let obj = self.new_object(model.name());
                let set_json_result = match create {
                    Some(create) => {
                        obj.set_json(create).await
                    }
                    None => {
                        let empty = JsonValue::Object(Map::new());
                        obj.set_json(&empty).await
                    }
                };
                return match set_json_result {
                    Ok(_) => {
                        match obj.save().await {
                            Ok(_) => {
                                HttpResponse::Ok().json(json!({"data": obj.to_json()}))
                            }
                            Err(err) => {
                                HttpResponse::BadRequest().json(json!({"error": err}))
                            }
                        }
                    }
                    Err(err) => {
                        HttpResponse::BadRequest().json(json!({"error": err}))
                    }
                }
            }
        }
    }

    async fn handle_delete(&'static self, input: &Map<String, JsonValue>, model: &'static Model) -> HttpResponse {
        let result = self.find_unique(model, input).await;
        match result {
            Ok(obj) => {
                // find the object here
                return match obj.delete().await {
                    Ok(_) => {
                        HttpResponse::Ok().json(json!({"data": obj.to_json()}))
                    }
                    Err(err) => {
                        HttpResponse::BadRequest().json(json!({"error": err}))
                    }
                }
            }
            Err(err) => {
                return HttpResponse::NotFound().json(json!({"error": err}));
            }
        }
    }

    async fn handle_create_many(&self, input: &Map<String, JsonValue>, model: &Model) -> HttpResponse {
        HttpResponse::Ok().json(json!({"Hello": "World!"}))
    }

    async fn handle_update_many(&self, input: &Map<String, JsonValue>, model: &Model) -> HttpResponse {
        HttpResponse::Ok().json(json!({"Hello": "World!"}))
    }

    async fn handle_delete_many(&self, input: &Map<String, JsonValue>, model: &Model) -> HttpResponse {
        HttpResponse::Ok().json(json!({"Hello": "World!"}))
    }

    async fn handle_count(&self, input: &Map<String, JsonValue>, model: &Model) -> HttpResponse {
        HttpResponse::Ok().json(json!({"Hello": "World!"}))
    }

    async fn handle_aggregate(&self, input: &Map<String, JsonValue>, model: &Model) -> HttpResponse {
        HttpResponse::Ok().json(json!({"Hello": "World!"}))
    }

    async fn handle_group_by(&self, input: &Map<String, JsonValue>, model: &Model) -> HttpResponse {
        HttpResponse::Ok().json(json!({"Hello": "World!"}))
    }

    async fn handle_sign_in(&self, input: &Map<String, JsonValue>, model: &Model) -> HttpResponse {
        let credentials = input.get("credentials");
        if let None = credentials {
            return HttpResponse::BadRequest().json(json!({"error": ActionError::missing_credentials()}));
        }
        let credentials = credentials.unwrap();
        if !credentials.is_object() {
            return HttpResponse::BadRequest().json(json!({"error": ActionError::wrong_json_format()}));
        }
        let credentials = credentials.as_object().unwrap();
        let mut identity_key: Option<&String> = None;
        let mut identity_value: Option<&JsonValue> = None;
        let mut by_key: Option<&String> = None;
        let mut by_value: Option<&JsonValue> = None;
        for (k, v) in credentials {
            if model.auth_identity_keys().contains(&&**k) {
                if identity_key == None {
                    identity_key = Some(k);
                    identity_value = Some(v);
                } else {
                    return HttpResponse::BadRequest().json(json!({"error": ActionError::multiple_auth_identity_provided()}));
                }
            } else if model.auth_by_keys().contains(&&**k) {
                if by_key == None {
                    by_key = Some(k);
                    by_value = Some(v);
                } else {
                    return HttpResponse::BadRequest().json(json!({"error": ActionError::multiple_auth_checker_provided()}));
                }
            } else {
                return HttpResponse::BadRequest().json(json!({"error": ActionError::keys_unallowed()}));
            }
        }
        if identity_key == None && by_key == None {
            return HttpResponse::BadRequest().json(json!({"error": ActionError::missing_credentials()}));
        } else if identity_key == None {
            return HttpResponse::BadRequest().json(json!({"error": ActionError::missing_auth_identity()}));
        } else if by_key == None {
            return HttpResponse::BadRequest().json(json!({"error": ActionError::missing_auth_checker()}));
        }
        let identity_field = model.field(identity_key.unwrap());
        let by_field = model.field(by_key.unwrap());
        //let col = self.find_unique(model, json!{})
        HttpResponse::Ok().json(json!({"Hello": "World!"}))
    }
}

unsafe impl Send for Graph {}
unsafe impl Sync for Graph {}
