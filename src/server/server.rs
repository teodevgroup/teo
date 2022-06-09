use actix_http::body::MessageBody;
use actix_http::Response;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web, error::Error};
use actix_web::dev::{HttpServiceFactory, ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::http::Method;
use chrono::{Duration, Utc};
use futures_util::StreamExt;
use serde_json::{json, Map, Value as JsonValue};
use crate::action::action::ActionType;
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::stage::Stage;
use crate::error::ActionError;
use crate::server::jwt::{Claims, decode_token, encode_token};


pub struct Server {
    graph: &'static Graph
}

impl Server {
    pub fn new(graph: &'static Graph) -> Self {
        Self { graph }
    }

    pub async fn start(&'static self, port: u16) -> std::io::Result<()> {
        HttpServer::new(move || {
            self.make_app()
        })
            .bind(("127.0.0.1", port))
            .unwrap()
            .run()
            .await
    }

    fn make_app(&'static self) -> App<impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >> {
        let this = self.graph;
        let app = App::new()
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
                                                                                    return match self.get_identity(&r).await {
                                                                                        Ok(_identity) => self.handle_find_unique(map, model_def).await,
                                                                                        Err(err) => HttpResponse::Unauthorized().json(json!({"error": err }))
                                                                                    };
                                                                                }
                                                                                ActionType::FindFirst => {
                                                                                    return match self.get_identity(&r).await {
                                                                                        Ok(_identity) => self.handle_find_first(map, model_def).await,
                                                                                        Err(err) => HttpResponse::Unauthorized().json(json!({"error": err }))
                                                                                    };
                                                                                }
                                                                                ActionType::FindMany => {
                                                                                    return match self.get_identity(&r).await {
                                                                                        Ok(_identity) => self.handle_find_many(map, model_def).await,
                                                                                        Err(err) => HttpResponse::Unauthorized().json(json!({"error": err }))
                                                                                    };
                                                                                }
                                                                                ActionType::Create => {
                                                                                    return match self.get_identity(&r).await {
                                                                                        Ok(_identity) => self.handle_create(map, model_def).await,
                                                                                        Err(err) => HttpResponse::Unauthorized().json(json!({"error": err }))
                                                                                    };
                                                                                }
                                                                                ActionType::Update => {
                                                                                    return match self.get_identity(&r).await {
                                                                                        Ok(_identity) => self.handle_update(map, model_def).await,
                                                                                        Err(err) => HttpResponse::Unauthorized().json(json!({"error": err }))
                                                                                    };
                                                                                }
                                                                                ActionType::Upsert => {
                                                                                    return match self.get_identity(&r).await {
                                                                                        Ok(_identity) => self.handle_upsert(map, model_def).await,
                                                                                        Err(err) => HttpResponse::Unauthorized().json(json!({"error": err }))
                                                                                    };
                                                                                }
                                                                                ActionType::Delete => {
                                                                                    return match self.get_identity(&r).await {
                                                                                        Ok(_identity) => self.handle_delete(map, model_def).await,
                                                                                        Err(err) => HttpResponse::Unauthorized().json(json!({"error": err }))
                                                                                    };
                                                                                }
                                                                                ActionType::CreateMany => {
                                                                                    return match self.get_identity(&r).await {
                                                                                        Ok(_identity) => self.handle_create_many(map, model_def).await,
                                                                                        Err(err) => HttpResponse::Unauthorized().json(json!({"error": err }))
                                                                                    };
                                                                                }
                                                                                ActionType::UpdateMany => {
                                                                                    return match self.get_identity(&r).await {
                                                                                        Ok(_identity) => self.handle_update_many(map, model_def).await,
                                                                                        Err(err) => HttpResponse::Unauthorized().json(json!({"error": err }))
                                                                                    };
                                                                                }
                                                                                ActionType::DeleteMany => {
                                                                                    return match self.get_identity(&r).await {
                                                                                        Ok(_identity) => self.handle_delete_many(map, model_def).await,
                                                                                        Err(err) => HttpResponse::Unauthorized().json(json!({"error": err }))
                                                                                    };
                                                                                }
                                                                                ActionType::Count => {
                                                                                    return match self.get_identity(&r).await {
                                                                                        Ok(_identity) => self.handle_count(map, model_def).await,
                                                                                        Err(err) => HttpResponse::Unauthorized().json(json!({"error": err }))
                                                                                    };
                                                                                }
                                                                                ActionType::Aggregate => {
                                                                                    return match self.get_identity(&r).await {
                                                                                        Ok(_identity) => self.handle_aggregate(map, model_def).await,
                                                                                        Err(err) => HttpResponse::Unauthorized().json(json!({"error": err }))
                                                                                    };
                                                                                }
                                                                                ActionType::GroupBy => {
                                                                                    return match self.get_identity(&r).await {
                                                                                        Ok(_identity) => self.handle_group_by(map, model_def).await,
                                                                                        Err(err) => HttpResponse::Unauthorized().json(json!({"error": err }))
                                                                                    };
                                                                                }
                                                                                ActionType::SignIn => {
                                                                                    return self.handle_sign_in(map, model_def).await;
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
            }));
        app
    }

    async fn get_identity(&'static self, r: &HttpRequest) -> Result<Option<Object>, ActionError> {
        let header_value = r.headers().get("authorization");
        if let None = header_value {
            return Ok(None);
        }
        let auth_str = header_value.unwrap().to_str().unwrap();
        if auth_str.len() < 7 {
            return Err(ActionError::invalid_authorization_format());
        }
        let token_str = &auth_str[7..];
        let claims_result = decode_token(&token_str.to_string(), self.graph.jwt_secret());
        if let Err(_) = claims_result {
            return Err(ActionError::invalid_jwt_token());
        }
        let claims = claims_result.unwrap();
        let model = self.graph.model(claims.model.as_str());
        let primary_field_name = model.primary_field_name().unwrap();
        let identity = self.graph.find_unique(
            self.graph.model(claims.model.as_str()),
            json!({
                "where": { primary_field_name: claims.id }
            }).as_object().unwrap()
        ).await;
        if let Err(_) = identity {
            return Err(ActionError::identity_is_not_found());
        }
        return Ok(Some(identity.unwrap()));
    }


    async fn handle_find_unique(&'static self, input: &Map<String, JsonValue>, model: &'static Model) -> HttpResponse {
        let result = self.graph.find_unique(model, input).await;
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
        let result = self.graph.find_first(model, input).await;
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
        let result = self.graph.find_many(model, input).await;
        match result {
            Ok(results) => {
                let count = self.graph.count(model, input).await.unwrap();
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

    async fn handle_create(&self, input: &Map<String, JsonValue>, model: &Model) -> HttpResponse {
        let create = input.get("create");
        let obj = self.graph.new_object(model.name());
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
        let result = self.graph.find_unique(model, input).await;
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
        let result = self.graph.find_unique(model, input).await;
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
            Err(_) => {
                let create = input.get("create");
                let obj = self.graph.new_object(model.name());
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
        let result = self.graph.find_unique(model, input).await;
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

    async fn handle_sign_in(&'static self, input: &Map<String, JsonValue>, model: &'static Model) -> HttpResponse {
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
            if model.auth_identity_keys().contains(k) {
                if identity_key == None {
                    identity_key = Some(k);
                    identity_value = Some(v);
                } else {
                    return HttpResponse::BadRequest().json(json!({"error": ActionError::multiple_auth_identity_provided()}));
                }
            } else if model.auth_by_keys().contains(k) {
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
        let by_field = model.field(by_key.unwrap());
        let obj_result = self.graph.find_unique(model, json!({
            "where": {
                identity_key.unwrap(): identity_value.unwrap()
            }
        }).as_object().unwrap()).await;
        if let Err(err) = obj_result {
            return HttpResponse::BadRequest().json(json!({"error": err}));
        }
        let obj = obj_result.unwrap();
        let auth_by_arg = by_field.unwrap().auth_by_arg.as_ref().unwrap();
        let pipeline = auth_by_arg.as_pipeline().unwrap();
        let action_by_value = by_field.unwrap().field_type.decode_value(by_value.unwrap(), self.graph);
        if let Err(err) = action_by_value {
            return HttpResponse::BadRequest().json(json!({"error": ActionError::wrong_input_type()}));
        }
        let stage = Stage::Value(action_by_value.unwrap());
        let final_stage = pipeline.process(stage, &obj).await;
        let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
        let claims = Claims {
            id: "not work anymore".to_string(),
            //id: obj.identifier().to_bson_value().as_object_id().unwrap().to_hex(), // change here later
            model: obj.inner.model.name().to_string(),
            exp
        };
        let token = encode_token(claims, self.graph.jwt_secret());
        return if let Stage::Value(_) = final_stage {
            HttpResponse::Ok().json(json!({
                "meta": token,
                "data": obj.to_json()
            }))
        } else {
            HttpResponse::BadRequest().json(json!({"error": ActionError::authentication_failed()}))
        }
    }

}
