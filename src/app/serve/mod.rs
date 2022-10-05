use std::time::SystemTime;
use actix_http::body::BoxBody;
use actix_http::{Method};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::DefaultHeaders;
use chrono::{DateTime, Duration, Local, Utc};
use colored::Colorize;
use serde_json::{json, Value as JsonValue};
use futures_util::StreamExt;
use serde::{Serialize, Deserialize};
use key_path::{KeyPath, path};
use crate::core::action::r#type::ActionType;
use crate::app::app::ServerConfiguration;
use crate::app::serve::jwt_token::{Claims, decode_token, encode_token};
use crate::core::action::r#type::ActionType::{Create, CreateMany, Delete, DeleteMany, FindFirst, FindMany, FindUnique, Update, UpdateMany};
use crate::core::env::Env;
use crate::core::env::position::Position;
use crate::core::env::position::Position::{RootMany, RootSingle};
use crate::core::env::source::Source;
use crate::core::graph::Graph;
use crate::core::input::Input;
use crate::core::input_decoder::decode_field_input;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::pipeline::context::{Context};
use crate::core::error::ActionError;
use crate::utils::json::check_json_keys;

pub(crate) mod response;
pub(crate) mod jwt_token;

fn path_components(path: &str) -> Vec<&str> {
    let components = path.split("/");
    let mut retval: Vec<&str> = Vec::new();
    let mut ignore = true;
    for component in components {
        if ignore == true {
            ignore = false;
        } else {
            retval.push(component);
        }
    }
    retval
}

fn log_unhandled(start: SystemTime, method: &str, path: &str, code: u16) {
    let now = SystemTime::now();
    let local: DateTime<Local> = Local::now();
    let code_string = match code {
        0..=199 => code.to_string().purple().bold(),
        200..=299 => code.to_string().green().bold(),
        300..=399 => code.to_string().yellow().bold(),
        _ => code.to_string().red().bold(),
    };
    let elapsed = now.duration_since(start).unwrap();
    let ms = elapsed.as_millis();
    let ms_str = format!("{ms}ms").normal().clear();
    let local_formatted = format!("{local}").cyan();
    let unhandled = "Unhandled".red();
    println!("{} {} {} on {} - {} {}", local_formatted, unhandled, method.bright_yellow(), path.bright_magenta(), code_string, ms_str);
}

fn log_request(start: SystemTime, action: &str, model: &str, code: u16) {
    let now = SystemTime::now();
    let local: DateTime<Local> = Local::now();
    let code_string = match code {
        0..=199 => code.to_string().purple().bold(),
        200..=299 => code.to_string().green().bold(),
        300..=399 => code.to_string().yellow().bold(),
        _ => code.to_string().red().bold(),
    };
    let elapsed = now.duration_since(start).unwrap();
    let ms = elapsed.as_millis();
    let ms_str = format!("{ms}ms").normal().clear();
    let local_formatted = format!("{local}").cyan();
    println!("{} {} on {} - {} {}", local_formatted, action.bright_yellow(), model.bright_magenta(), code_string, ms_str);
}

async fn get_identity(r: &HttpRequest, graph: &Graph, conf: &ServerConfiguration) -> Result<Option<Object>, ActionError> {
    let header_value = r.headers().get("authorization");
    if let None = header_value {
        return Ok(None);
    }
    let auth_str = header_value.unwrap().to_str().unwrap();
    if auth_str.len() < 7 {
        return Err(ActionError::invalid_authorization_format());
    }
    let token_str = &auth_str[7..];
    let claims_result = decode_token(&token_str.to_string(), &conf.jwt_secret.as_ref().unwrap());
    if let Err(_) = claims_result {
        return Err(ActionError::invalid_auth_token());
    }
    let claims = claims_result.unwrap();
    let model = graph.model(claims.model.as_str()).unwrap();
    let identity = graph.find_unique(
        graph.model(claims.model.as_str()).unwrap().name(),
        &json!({
                "where": claims.id
            }),
        true
    ).await;
    if let Err(_) = identity {
        return Err(ActionError::invalid_auth_token())
    }
    return Ok(Some(identity.unwrap()));
}

async fn handle_find_unique(graph: &Graph, input: &JsonValue, model: &Model, source: Source) -> HttpResponse {
    let env = Env::new(source, FindUnique, RootSingle);
    let result = graph.find_unique(model.name(), input, false, env).await;
    match result {
        Ok(obj) => {
            let json_data = obj.to_json().await;
            HttpResponse::Ok().json(json!({"data": json_data}))
        }
        Err(err) => {
            err.into()
        }
    }
}

async fn handle_find_first(graph: &Graph, input: &JsonValue, model: &Model, source: Source) -> HttpResponse {
    let env = Env::new(source, FindFirst, RootSingle);
    let result = graph.find_first(model.name(), input, false, env).await;
    match result {
        Ok(obj) => {
            let json_data = obj.to_json().await;
            HttpResponse::Ok().json(json!({"data": json_data}))
        }
        Err(err) => {
            err.into()
        }
    }
}

async fn handle_find_many(graph: &Graph, input: &JsonValue, model: &Model, source: Source) -> HttpResponse {
    let env = Env::new(source, FindMany, RootMany);
    let result = graph.find_many(model.name(), input, false, env).await;
    match result {
        Ok(results) => {
            let mut count_input = input.clone();
            let count_input_obj = count_input.as_object_mut().unwrap();
            count_input_obj.remove("skip");
            count_input_obj.remove("take");
            count_input_obj.remove("pageSize");
            count_input_obj.remove("pageNumber");
            let count = graph.count(model.name(), &count_input).await.unwrap();
            let mut meta = json!({"count": count});
            let page_size = input.get("pageSize");
            if page_size.is_some() {
                let page_size = page_size.unwrap().as_i64().unwrap();
                let count = count as i64;
                let mut number_of_pages = count / page_size;
                if count % page_size != 0 {
                    number_of_pages += 1;
                }
                meta.as_object_mut().unwrap().insert("numberOfPages".to_string(), JsonValue::Number(number_of_pages.into()));
            }

            let mut result_json: Vec<JsonValue> = vec![];
            for result in results {
                match result.to_json().await {
                    Ok(result) => result_json.push(result),
                    Err(_) => ()
                }
            }
            HttpResponse::Ok().json(json!({
                    "meta": meta,
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

async fn handle_create_internal(graph: &Graph, create: Option<&JsonValue>, include: Option<&JsonValue>, select: Option<&JsonValue>, model: &Model, path: &KeyPath<'_>, env: Env) -> Result<JsonValue, ActionError> {
    let obj = graph.new_object(model.name(), env)?;
    let set_json_result = match create {
        Some(create) => {
            if !create.is_object() {
                return Err(ActionError::unexpected_input_type("object", path));
            }
            obj._set_json(create, path).await
        }
        None => {
            obj._set_json(&json!({}), path).await
        }
    };
    if set_json_result.is_err() {
        return Err(set_json_result.err().unwrap());
    }
    obj.save().await?;
    let refetched = obj.refreshed(include, select).await?;
    refetched.to_json().await
}

async fn handle_create(graph: &Graph, input: &JsonValue, model: &Model, source: Source) -> HttpResponse {
    let env = Env::new(source, Create, RootSingle);
    let input = input.as_object().unwrap();
    let create = input.get("create");
    let include = input.get("include");
    let select = input.get("select");
    let result = handle_create_internal(graph, create, include, select, model, &path!["create"], env).await;
    match result {
        Ok(val) => HttpResponse::Ok().json(json!({"data": val})),
        Err(err) => HttpResponse::BadRequest().json(json!({"error": err}))
    }
}

async fn handle_update_internal(_graph: &Graph, object: Object, update: Option<&JsonValue>, include: Option<&JsonValue>, select: Option<&JsonValue>, _where: Option<&JsonValue>, _model: &Model) -> Result<JsonValue, ActionError> {
    let empty = json!({});
    let updator = if update.is_some() { update.unwrap() } else { &empty };
    object.set_json(updator).await?;
    object.save().await?;
    let refetched = object.refreshed(include, select).await?;
    refetched.to_json().await
}

async fn handle_update(graph: &Graph, input: &JsonValue, model: &Model, source: Source) -> HttpResponse {
    let env = Env::new(source, Update, RootSingle);
    let result = graph.find_unique(model.name(), input, true, env).await;
    if result.is_err() {
        return HttpResponse::NotFound().json(json!({"error": result.err()}));
    }
    let result = result.unwrap();
    let update = input.get("update");
    let include = input.get("include");
    let select = input.get("select");
    let r#where = input.get("where");
    let update_result = handle_update_internal(graph, result.clone(), update, include, select, r#where, model).await;
    match update_result {
        Ok(value) => {
            HttpResponse::Ok().json(json!({"data": value}))
        }
        Err(err) => {
            HttpResponse::BadRequest().json(json!({"error": err}))
        }
    }
}

async fn handle_upsert(graph: &Graph, input: &JsonValue, model: &Model, source: Source) -> HttpResponse {
    let env = Env::new(source, Update, RootSingle);
    let result = graph.find_unique(model.name(), input, true, env).await;
    let include = input.get("include");
    let select = input.get("select");
    match result {
        Ok(obj) => {
            // find the object here
            let update = input.get("update");
            let set_json_result = match update {
                Some(update) => {
                    obj.set_json(update).await
                }
                None => {
                    let empty = json!({});
                    obj.set_json(&empty).await
                }
            };
            return match set_json_result {
                Ok(_) => {
                    match obj.save().await {
                        Ok(_) => {
                            // refetch here
                            let refetched = obj.refreshed(include, select).await.unwrap();
                            HttpResponse::Ok().json(json!({"data": refetched.to_json().await}))
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
        Err(_) => {
            let create = input.get("create");
            let env = Env::new(source.clone(), Create, RootSingle);
            let obj = graph.new_object(model.name(), env).unwrap();
            let set_json_result = match create {
                Some(create) => {
                    obj.set_json(create).await
                }
                None => {
                    let empty = json!({});
                    obj.set_json(&empty).await
                }
            };
            return match set_json_result {
                Ok(_) => {
                    match obj.save().await {
                        Ok(_) => {
                            // refetch here
                            let refetched = obj.refreshed(include, select).await.unwrap();
                            let json_data = refetched.to_json().await;
                            return HttpResponse::Ok().json(json!({"data": json_data}));
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

async fn handle_delete(graph: &Graph, input: &JsonValue, model: &Model, source: Source) -> HttpResponse {
    let env = Env::new(source, Delete, RootSingle);
    let result = graph.find_unique(model.name(), input, true, env).await;
    if result.is_err() {
        return HttpResponse::NotFound().json(json!({"error": result.err()}));
    }
    let result = result.unwrap();
    // find the object here
    return match result.delete().await {
        Ok(_) => {
            let json_data = result.to_json().await;
            HttpResponse::Ok().json(json!({"data": json_data}))
        }
        Err(err) => {
            err.into()
        }
    }
}

async fn handle_create_many(graph: &Graph, input: &JsonValue, model: &Model, source: Source) -> HttpResponse {
    let env = Env::new(source, CreateMany, RootMany);
    let input = input.as_object().unwrap();
    let create = input.get("create");
    let include = input.get("include");
    let select = input.get("select");
    if create.is_none() {
        let err = ActionError::missing_required_input("array", path!["create"]);
        return HttpResponse::BadRequest().json(json!({"error": err}));
    }
    let create = create.unwrap();
    if !create.is_array() {
        let err = ActionError::unexpected_input_type("array", path!["create"]);
        return HttpResponse::BadRequest().json(json!({"error": err}));
    }
    let create = create.as_array().unwrap();
    let mut count = 0;
    let mut ret_data: Vec<JsonValue> = vec![];
    for (index, val) in create.iter().enumerate() {
        let result = handle_create_internal(graph, Some(val), include, select, model, &path!["create", index], env.clone()).await;
        match result {
            Err(_) => (),
            Ok(val) => {
                count += 1;
                ret_data.push(val);
            }
        }
    }
    HttpResponse::Ok().json(json!({
            "meta": {"count": count},
            "data": ret_data
        }))
}

async fn handle_update_many(graph: &Graph, input: &JsonValue, model: &Model, source: Source) -> HttpResponse {
    let env = Env::new(source, UpdateMany, RootMany);
    let result = graph.find_many(model.name(), input, true, env).await;
    if result.is_err() {
        return HttpResponse::BadRequest().json(json!({"error": result.err()}));
    }
    let result = result.unwrap();
    let update = input.get("update");
    let include = input.get("include");
    let select = input.get("select");

    let mut count = 0;
    let mut ret_data: Vec<JsonValue> = vec![];
    for object in result {
        let update_result = handle_update_internal(graph, object.clone(), update, include, select, None, model).await;
        match update_result {
            Ok(json_value) => {
                ret_data.push(json_value);
                count += 1;
            }
            Err(_err) => {}
        }
    }
    HttpResponse::Ok().json(json!({
            "meta": {
                "count": count
            },
            "data": ret_data
        }))
}

async fn handle_delete_many(graph: &Graph, input: &JsonValue, model: &Model, source: Source) -> HttpResponse {
    let env = Env::new(source, DeleteMany, RootMany);
    let result = graph.find_many(model.name(), input, true, env).await;
    if result.is_err() {
        return HttpResponse::BadRequest().json(json!({"error": result.err()}));
    }
    let result = result.unwrap();
    let mut count = 0;
    let mut retval: Vec<JsonValue> = vec![];
    for object in result {
        match object.delete().await {
            Ok(_) => {
                match object.to_json().await {
                    Ok(result) => {
                        retval.push(result);
                        count += 1;
                    },
                    Err(_) => ()
                }
            }
            Err(_) => {}
        }
    }
    HttpResponse::Ok().json(json!({
            "meta": {
                "count": count
            },
            "data": retval
        }))
}

async fn handle_count(graph: &Graph, input: &JsonValue, model: &Model, source: Source) -> HttpResponse {
    let result = graph.count(model.name(), input).await;
    match result {
        Ok(count) => {
            HttpResponse::Ok().json(json!({"data": count}))
        }
        Err(err) => {
            HttpResponse::BadRequest().json(json!({"error": err}))
        }
    }
}

async fn handle_aggregate(graph: &Graph, input: &JsonValue, model: &Model, source: Source) -> HttpResponse {
    match graph.aggregate(model.name(), input).await {
        Ok(count) => {
            HttpResponse::Ok().json(json!({"data": count}))
        }
        Err(err) => {
            HttpResponse::BadRequest().json(json!({"error": err}))
        }
    }
}

async fn handle_group_by(graph: &Graph, input: &JsonValue, model: &Model, source: Source) -> HttpResponse {
    match graph.group_by(model.name(), input).await {
        Ok(count) => {
            HttpResponse::Ok().json(json!({"data": count}))
        }
        Err(err) => {
            HttpResponse::BadRequest().json(json!({"error": err}))
        }
    }
}

async fn handle_sign_in(graph: &Graph, input: &JsonValue, model: &Model, conf: &ServerConfiguration) -> HttpResponse {
    let input = input.as_object().unwrap();
    let credentials = input.get("credentials");
    if let None = credentials {
        return ActionError::missing_required_input("object", path!["credentials"]).into();
    }
    let credentials = credentials.unwrap();
    if !credentials.is_object() {
        return ActionError::unexpected_input_type("object", path!["credentials"]).into();
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
                return ActionError::unexpected_input_value_validation("Multiple auth identity provided", path!["credentials", k]).into();
            }
        } else if model.auth_by_keys().contains(k) {
            if by_key == None {
                by_key = Some(k);
                by_value = Some(v);
            } else {
                return ActionError::unexpected_input_value_validation("Multiple auth checker provided", path!["credentials", k]).into();
            }
        } else {
            return ActionError::unexpected_input_key(k, path!["credentials", k]).into();
        }
    }
    if identity_key == None {
        return ActionError::missing_required_input("auth identity", path!["credentials"]).into();
    } else if by_key == None {
        return ActionError::missing_required_input("auth checker", path!["credentials"]).into();
    }
    let by_field = model.field(by_key.unwrap()).unwrap();
    let obj_result = graph.find_unique(model.name(), &json!({
        "where": {
            identity_key.unwrap(): identity_value.unwrap()
        }
    }), true).await;
    if let Err(err) = obj_result {
        return ActionError::unexpected_input_value("This identity is not found.", path!["credentials", identity_key.unwrap()]).into();
    }
    let obj = obj_result.unwrap();
    let auth_by_arg = by_field.auth_by_arg.as_ref().unwrap();
    let pipeline = auth_by_arg.as_pipeline().unwrap();
    let action_by_input = decode_field_input(obj.graph(), by_value.unwrap(), &by_field.field_type, by_field.optionality.clone(), &path!["credentials", by_field.name()]);
    let _action_by_value = match action_by_input {
        Err(_err) => {
            return ActionError::unexpected_input_type("field value", path!["credentials", by_key.unwrap()]).into();
        }
        Ok(val) => {
            match val {
                Input::SetValue(value) => {
                    value
                }
                _ => panic!()
            }
        }
    };
    let ctx = Context::initial_state(obj.clone());
    let final_ctx = pipeline.process(ctx).await;
    return match final_ctx.invalid_reason() {
        Some(_reason) => {
            return ActionError::unexpected_input_value_validation("Authentication failed.", path!["credentials", by_key.unwrap()]).into();
        }
        None => {
            let include = input.get("include");
            let select = input.get("select");
            let obj = obj.refreshed(include, select).await.unwrap();
            let json_data = obj.to_json().await;
            let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
            let claims = Claims {
                id: obj.json_identifier(),
                model: obj.model().name().to_string(),
                exp
            };
            let token = encode_token(claims, &conf.jwt_secret.as_ref().unwrap());
            HttpResponse::Ok().json(json!({
            "meta": {
                "token": token
            },
            "data": json_data
        }))
        }
    }
}

async fn handle_identity(_graph: &Graph, input: &JsonValue, model: &Model, _conf: &ServerConfiguration, identity: Option<&Object>) -> HttpResponse {
    if let Some(identity) = identity {
        if identity.model() != model {
            return HttpResponse::Unauthorized().json(json!({"error": ActionError::wrong_identity_model()}));
        }
        let select = input.get("select");
        let include = input.get("include");
        let refreshed = identity.refreshed(include, select).await.unwrap();
        let json_data = refreshed.to_json().await;
        HttpResponse::Ok().json(json!({
            "data": json_data
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "data": null
        }))
    }
}

pub fn make_app(graph: Graph, conf: ServerConfiguration) ->  App<impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<BoxBody>,
    Config = (),
    InitError = (),
    Error = Error,
> + 'static> {
    let leaked_graph = Box::leak(Box::new(graph));
    let leaked_conf = Box::leak(Box::new(conf));
    make_app_inner(leaked_graph, leaked_conf)
}

fn make_app_inner(graph: &'static Graph, conf: &'static ServerConfiguration) -> App<impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<BoxBody>,
    Config = (),
    InitError = (),
    Error = Error,
> + 'static> {
    let app = App::new()
        .wrap(DefaultHeaders::new()
            .add(("Access-Control-Allow-Origin", "*"))
            .add(("Access-Control-Allow-Methods", "OPTIONS, POST, GET"))
            .add(("Access-Control-Allow-Headers", "*"))
            .add(("Access-Control-Max-Age", "86400")))
        .default_service(web::route().to(move |r: HttpRequest, mut payload: web::Payload| async move {
            let start = SystemTime::now();
            let mut path = r.path().to_string();
            if let Some(prefix) = &conf.path_prefix {
                if !path.starts_with(prefix) {
                    log_unhandled(start, r.method().as_str(), &path, 404);
                    return ActionError::destination_not_found().into();
                }
                path = path.strip_prefix(prefix).unwrap().to_string();
            }
            let path = if path.len() > 1 && path.ends_with("/") {
                path[0..path.len() - 1].to_string()
            } else {
                path
            };
            if (r.method() != Method::POST) && (r.method() != Method::OPTIONS) {
                log_unhandled(start, r.method().as_str(), &path, 404);
                return ActionError::destination_not_found().into();
            }
            let path_components = path_components(&path);
            let first_component = path_components.get(1).unwrap();
            if !(path_components.len() == 3 && first_component == &"action") {
                log_unhandled(start, r.method().as_str(), &path, 404);
                return ActionError::destination_not_found().into();
            }
            let model_url_segment_name = path_components[0];
            let action_segment_name = path_components[2];
            let action = ActionType::from_url_segment(action_segment_name);
            let action = match action {
                Some(a) => a,
                None => {
                    log_unhandled(start, r.method().as_str(), &path, 404);
                    return ActionError::destination_not_found().into();
                }
            };
            let model_name = match graph.model_name_for_url_segment_name(model_url_segment_name) {
                Some(name) => name,
                None => {
                    log_unhandled(start, r.method().as_str(), &path, 404);
                    return ActionError::destination_not_found().into();
                }
            };
            let model_def = graph.model(model_name).unwrap();
            if !model_def.has_action(action) {
                log_unhandled(start, r.method().as_str(), &path, 400);
                return ActionError::destination_not_found().into();
            }
            if r.method() == Method::OPTIONS {
                return HttpResponse::Ok().json(json!({}));
            }
            // read body
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
            let parsed_body = match parsed_body {
                Ok(b) => b,
                Err(_) => {
                    log_unhandled(start, r.method().as_str(), &path, 400);
                    return HttpResponse::BadRequest().json(json!({"error": ActionError::incorrect_json_format()}));
                }
            };
            if !parsed_body.is_object() {
                log_unhandled(start, r.method().as_str(), &path, 400);
                return HttpResponse::BadRequest().json(json!({"error": ActionError::unexpected_input_root_type("object")}));
            }
            let identity = match get_identity(&r, &graph, conf).await {
                Ok(identity) => { identity },
                Err(err) => return HttpResponse::Unauthorized().json(json!({"error": err }))
            };
            let action_def = model_def.get_action_def(action);
            let transformed_body = if let Some(action_def) = action_def {
                action_def.transform(&parsed_body, identity.clone()).await
            } else {
                parsed_body
            };
            match check_json_keys(&transformed_body, action.allowed_input_json_keys(), &path![]) {
                Ok(()) => (),
                Err(err) => { return err.into(); }
            };
            let source = Source::Identity(identity);
            match action {
                ActionType::FindUnique => {
                    let result = handle_find_unique(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, "FindUnique", model_def.name(), result.status().as_u16());
                    return result;
                }
                ActionType::FindFirst => {
                    let result = handle_find_first(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, "FindFirst", model_def.name(), result.status().as_u16());
                    result
                }
                ActionType::FindMany => {
                    let result = handle_find_many(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, "FindMany", model_def.name(), result.status().as_u16());
                    result
                }
                ActionType::Create => {
                    let result = handle_create(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, "Create", model_def.name(), result.status().as_u16());
                    result
                }
                ActionType::Update => {
                    let result = handle_update(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, "Update", model_def.name(), result.status().as_u16());
                    result
                }
                ActionType::Upsert => {
                    let result = handle_upsert(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, "Upsert", model_def.name(), result.status().as_u16());
                    result
                }
                ActionType::Delete => {
                    let result = handle_delete(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, "Delete", model_def.name(), result.status().as_u16());
                    result
                }
                ActionType::CreateMany => {
                    let result = handle_create_many(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, "CreateMany", model_def.name(), result.status().as_u16());
                    result
                }
                ActionType::UpdateMany => {
                    let result = handle_update_many(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, "UpdateMany", model_def.name(), result.status().as_u16());
                    result
                }
                ActionType::DeleteMany => {
                    let result = handle_delete_many(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, "DeleteMany", model_def.name(), result.status().as_u16());
                    result
                }
                ActionType::Count => {
                    let result = handle_count(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, "Count", model_def.name(), result.status().as_u16());
                    result
                }
                ActionType::Aggregate => {
                    let result = handle_aggregate(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, "Aggregate", model_def.name(), result.status().as_u16());
                    result
                }
                ActionType::GroupBy => {
                    let result = handle_group_by(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, "GroupBy", model_def.name(), result.status().as_u16());
                    result
                }
                ActionType::SignIn => {
                    let result = handle_sign_in(&graph, &transformed_body, model_def, conf).await;
                    log_request(start, "SignIn", model_def.name(), result.status().as_u16());
                    result
                }
                ActionType::Identity => {
                    let result = handle_identity(&graph, &transformed_body, model_def, conf, source.clone()).await;
                    log_request(start, "Identity", model_def.name(), result.status().as_u16());
                    result
                }
            }
        }));
    app
}

pub(crate) async fn serve(graph: Graph, conf: ServerConfiguration) -> Result<(), std::io::Error> {
    let conf2 = conf.clone();
    HttpServer::new(move || {
        make_app(graph.clone(), conf.clone())
    })
        .bind(conf2.bind.clone())
        .unwrap()
        .run()
        .await
}
