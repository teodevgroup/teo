use std::io::ErrorKind;
use std::sync::Arc;
use futures_util::future;
use std::time::SystemTime;
use actix_http::body::BoxBody;
use actix_http::{Method};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::DefaultHeaders;
use chrono::{DateTime, Duration, Local, Utc};
use colored::Colorize;
use futures_util::StreamExt;
use key_path::{KeyPath, path};
use serde_json::{json, Value as JsonValue};
use to_mut::ToMut;
use crate::core::action::{
    Action, CREATE, DELETE, ENTRY, FIND, IDENTITY, MANY, SINGLE, UPDATE, UPSERT,
    FIND_UNIQUE_HANDLER, FIND_FIRST_HANDLER, FIND_MANY_HANDLER, CREATE_HANDLER, UPDATE_HANDLER,
    UPSERT_HANDLER, DELETE_HANDLER, CREATE_MANY_HANDLER, UPDATE_MANY_HANDLER, DELETE_MANY_HANDLER,
    COUNT_HANDLER, AGGREGATE_HANDLER, GROUP_BY_HANDLER, SIGN_IN_HANDLER, IDENTITY_HANDLER,
};
use crate::core::action::source::ActionSource;
use crate::core::app::builder::AsyncCallbackWithoutArgs;
use crate::core::app::conf::ServerConf;
use crate::core::app::entrance::Entrance;
use crate::core::app::environment::EnvironmentVersion;
use crate::core::app::migrate::migrate;
use crate::core::connector::SaveSession;
use self::jwt_token::{Claims, decode_token, encode_token};
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::pipeline::ctx::{Ctx};
use crate::core::error::Error;
use crate::core::teon::decoder::Decoder;
use crate::prelude::Value;
use crate::teon;

pub(crate) mod response;
pub(crate) mod jwt_token;

fn j(v: Value) -> JsonValue {
    v.into()
}

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
    let ms_str = format!("{ms}ms").dimmed();
    let local_formatted = format!("{local}").dimmed();
    let unhandled = "Unhandled".red();
    println!("{} {} {} on {} - {} {}", local_formatted, unhandled, method.bold(), path, code_string, ms_str);
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
    let local_formatted = format!("{local}").dimmed();
    println!("{} {} on {} - {} {}", local_formatted, action.bold(), model, code_string, ms_str.dimmed());
}

async fn get_identity(r: &HttpRequest, graph: &Graph, conf: &ServerConf) -> Result<Option<Object>, Error> {
    let header_value = r.headers().get("authorization");
    if let None = header_value {
        return Ok(None);
    }
    let auth_str = header_value.unwrap().to_str().unwrap();
    if auth_str.len() < 7 {
        return Err(Error::invalid_auth_token());
    }
    let token_str = &auth_str[7..];
    let claims_result = decode_token(&token_str.to_string(), &conf.jwt_secret.as_ref().unwrap());
    if let Err(_) = claims_result {
        return Err(Error::invalid_auth_token());
    }
    let claims = claims_result.unwrap();
    let json_identifier = claims.id;
    let tson_identifier = Decoder::decode_object(graph.model(&claims.model).unwrap(), graph, &json_identifier)?;
    // Decoder::
    let _model = graph.model(claims.model.as_str()).unwrap();
    let identity = graph.find_unique_internal(
        graph.model(claims.model.as_str()).unwrap().name(),
        &teon!({
            "where": tson_identifier
        }),
        true, Action::from_u32(IDENTITY | FIND | SINGLE | ENTRY), ActionSource::ProgramCode).await;
    if let Err(_) = identity {
        return Err(Error::invalid_auth_token())
    }
    return Ok(Some(identity.unwrap()));
}

async fn handle_find_unique(graph: &Graph, input: &Value, model: &Model, source: ActionSource) -> HttpResponse {
    let action = Action::from_u32(FIND | SINGLE | ENTRY);
    let result = graph.find_unique_internal(model.name(), input, false, action, source).await;
    match result {
        Ok(obj) => {
            let json_data: JsonValue = obj.to_json_internal(&path!["data"]).await.unwrap().into();
            HttpResponse::Ok().json(json!({"data": json_data}))
        }
        Err(err) => {
            err.into()
        }
    }
}

async fn handle_find_first(graph: &Graph, input: &Value, model: &Model, source: ActionSource) -> HttpResponse {
    let action = Action::from_u32(FIND | SINGLE | ENTRY);
    let result = graph.find_first_internal(model.name(), input, false, action, source).await;
    match result {
        Ok(obj) => {
            let json_data: JsonValue = obj.to_json_internal(&path!["data"]).await.unwrap().into();
            HttpResponse::Ok().json(json!({"data": json_data}))
        }
        Err(err) => {
            err.into()
        }
    }
}

async fn handle_find_many(graph: &Graph, input: &Value, model: &Model, source: ActionSource) -> HttpResponse {
    let action = Action::from_u32(FIND | MANY | ENTRY);
    let result = graph.find_many_internal(model.name(), input, false, action, source).await;
    match result {
        Ok(results) => {
            let mut count_input = input.clone();
            let count_input_obj = count_input.as_hashmap_mut().unwrap();
            count_input_obj.remove("skip");
            count_input_obj.remove("take");
            count_input_obj.remove("pageSize");
            count_input_obj.remove("pageNumber");
            let count = graph.count(model.name(), &count_input).await.unwrap();
            let mut meta = json!({"count": count});
            let page_size = input.get("pageSize");
            if page_size.is_some() {
                let page_size = page_size.unwrap().as_i32().unwrap();
                let count = count as i32;
                let mut number_of_pages = count / page_size;
                if count % page_size != 0 {
                    number_of_pages += 1;
                }
                meta.as_object_mut().unwrap().insert("numberOfPages".to_string(), number_of_pages.into());
            }

            let mut result_json: Vec<JsonValue> = vec![];
            for (index, result) in results.iter().enumerate() {
                match result.to_json_internal(&path!["data", index]).await {
                    Ok(result) => result_json.push(result.into()),
                    Err(_) => return Error::permission_error(path!["data"], "not allowed to read").into(),
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

async fn handle_create_internal(graph: &Graph, create: Option<&Value>, include: Option<&Value>, select: Option<&Value>, model: &Model, path: &KeyPath<'_>, action: Action, action_source: ActionSource, session: Arc<dyn SaveSession>) -> Result<Value, Error> {
    let obj = graph.new_object(model.name(), action, action_source)?;
    let set_json_result = match create {
        Some(create) => {
            if !create.is_hashmap() {
                return Err(Error::unexpected_input_type("object", path));
            }
            obj.set_teon_with_path(create, path).await
        }
        None => {
            obj.set_teon_with_path(&teon!({}), path).await
        }
    };
    if set_json_result.is_err() {
        return Err(set_json_result.err().unwrap());
    }
    obj.save_with_session_and_path(session.clone(), path).await?;
    let refetched = obj.refreshed(include, select).await?;
    refetched.to_json_internal(&path!["data"]).await
}

async fn handle_create(graph: &Graph, input: &Value, model: &Model, source: ActionSource) -> HttpResponse {
    let action = Action::from_u32(CREATE | ENTRY | SINGLE);
    let input = input.as_hashmap().unwrap();
    let create = input.get("create");
    let include = input.get("include");
    let select = input.get("select");
    let session = graph.connector().new_save_session();
    let result = handle_create_internal(graph, create, include, select, model, &path!["create"], action, source, session).await;
    match result {
        Ok(val) => {
            let json_val: JsonValue = val.into();
            HttpResponse::Ok().json(json!({"data": json_val}))
        },
        Err(err) => HttpResponse::BadRequest().json(json!({"error": err}))
    }
}

async fn handle_update_internal(_graph: &Graph, object: Object, update: Option<&Value>, include: Option<&Value>, select: Option<&Value>, _where: Option<&Value>, _model: &Model) -> Result<Value, Error> {
    let empty = teon!({});
    let updator = if update.is_some() { update.unwrap() } else { &empty };
    object.set_teon_with_path(updator, &path!["update"]).await?;
    object.save().await?;
    let refetched = object.refreshed(include, select).await?;
    refetched.to_json_internal(&path!["data"]).await
}

async fn handle_update(graph: &Graph, input: &Value, model: &Model, source: ActionSource) -> HttpResponse {
    let action = Action::from_u32(UPDATE | ENTRY | SINGLE);
    let result = graph.find_unique_internal(model.name(), input, true, action, source).await;
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
            let json_val: JsonValue = value.into();
            HttpResponse::Ok().json(json!({"data": json_val}))
        }
        Err(err) => {
            HttpResponse::BadRequest().json(json!({"error": err}))
        }
    }
}

async fn handle_upsert(graph: &Graph, input: &Value, model: &Model, source: ActionSource) -> HttpResponse {
    let action = Action::from_u32(UPSERT | UPDATE | ENTRY | SINGLE);
    let result = graph.find_unique_internal(model.name(), input, true, action, source.clone()).await;
    let include = input.get("include");
    let select = input.get("select");
    return match result {
        Ok(obj) => {
            // find the object here
            let update = input.get("update");
            let set_json_result = match update {
                Some(update) => {
                    obj.set_teon_with_path(update, &path!["update"]).await
                }
                None => {
                    let empty = teon!({});
                    obj.set_teon_with_path(&empty, &path!["update"]).await
                }
            };
            match set_json_result {
                Ok(_) => {
                    match obj.save().await {
                        Ok(_) => {
                            // refetch here
                            let refetched = obj.refreshed(include, select).await.unwrap();
                            let json_val: JsonValue = refetched.to_json_internal(&path!["data"]).await.unwrap().into();
                            HttpResponse::Ok().json(json!({"data": json_val}))
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
            let action = Action::from_u32(UPSERT | CREATE | ENTRY | SINGLE);
            let obj = graph.new_object(model.name(), action, source).unwrap();
            let set_json_result = match create {
                Some(create) => {
                    obj.set_teon_with_path(create, &path!["create"]).await
                }
                None => {
                    let empty = teon!({});
                    obj.set_teon_with_path(&empty, &path!["create"]).await
                }
            };
            match set_json_result {
                Ok(_) => {
                    match obj.save().await {
                        Ok(_) => {
                            // refetch here
                            let refetched = obj.refreshed(include, select).await.unwrap();
                            let json_data: JsonValue = refetched.to_json_internal(&path!["data"]).await.unwrap().into();
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

async fn handle_delete(graph: &Graph, input: &Value, model: &Model, source: ActionSource) -> HttpResponse {
    let action = Action::from_u32(DELETE | SINGLE | ENTRY);
    let result = graph.find_unique_internal(model.name(), input, true, action, source).await;
    if result.is_err() {
        return HttpResponse::NotFound().json(json!({"error": result.err()}));
    }
    let result = result.unwrap();
    // find the object here
    return match result.delete_internal(path!["delete"]).await {
        Ok(_) => {
            let json_data: JsonValue = result.to_json_internal(&path!["data"]).await.unwrap().into();
            HttpResponse::Ok().json(json!({"data": json_data}))
        }
        Err(err) => {
            err.into()
        }
    }
}

async fn handle_create_many(graph: &Graph, input: &Value, model: &Model, source: ActionSource) -> HttpResponse {
    let action = Action::from_u32(CREATE | MANY | ENTRY);
    let input = input.as_hashmap().unwrap();
    let create = input.get("create");
    let include = input.get("include");
    let select = input.get("select");
    if create.is_none() {
        let err = Error::missing_required_input_with_type("array", path!["create"]);
        return HttpResponse::BadRequest().json(json!({"error": err}));
    }
    let create = create.unwrap();
    if !create.is_vec() {
        let err = Error::unexpected_input_type("array", path!["create"]);
        return HttpResponse::BadRequest().json(json!({"error": err}));
    }
    let create = create.as_vec().unwrap();
    let mut count = 0;
    let mut ret_data: Vec<Value> = vec![];
    let session = graph.connector().new_save_session();
    for (index, val) in create.iter().enumerate() {
        let result = handle_create_internal(graph, Some(val), include, select, model, &path!["create", index], action, source.clone(), session.clone()).await;
        match result {
            Err(err) => {
                println!("{:?}", err.errors);
            },
            Ok(val) => {
                count += 1;
                ret_data.push(val);
            }
        }
    }
    let json_ret_data: JsonValue = Value::Vec(ret_data).into();
    HttpResponse::Ok().json(json!({
        "meta": {"count": count},
        "data": json_ret_data
    }))
}

async fn handle_update_many(graph: &Graph, input: &Value, model: &Model, source: ActionSource) -> HttpResponse {
    let action = Action::from_u32(UPDATE | MANY | ENTRY);
    let result = graph.find_many_internal(model.name(), input, true, action, source).await;
    if result.is_err() {
        return HttpResponse::BadRequest().json(json!({"error": result.err()}));
    }
    let result = result.unwrap();
    let update = input.get("update");
    let include = input.get("include");
    let select = input.get("select");

    let mut count = 0;
    let mut ret_data: Vec<Value> = vec![];
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
            "data": j(Value::Vec(ret_data))
        }))
}

async fn handle_delete_many(graph: &Graph, input: &Value, model: &Model, source: ActionSource) -> HttpResponse {
    let action = Action::from_u32(DELETE | MANY | ENTRY);
    let result = graph.find_many_internal(model.name(), input, true, action, source).await;
    if result.is_err() {
        return HttpResponse::BadRequest().json(json!({"error": result.err()}));
    }
    let result = result.unwrap();
    let mut count = 0;
    let mut retval: Vec<Value> = vec![];
    for (index, object) in result.iter().enumerate() {
        match object.delete_internal(path!["delete"]).await {
            Ok(_) => {
                match object.to_json_internal(&path!["data", index]).await {
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
            "data": j(Value::Vec(retval))
        }))
}

async fn handle_count(graph: &Graph, input: &Value, model: &Model, _source: ActionSource) -> HttpResponse {
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

async fn handle_aggregate(graph: &Graph, input: &Value, model: &Model, _source: ActionSource) -> HttpResponse {
    match graph.aggregate(model.name(), input).await {
        Ok(count) => {
            HttpResponse::Ok().json(json!({"data": j(count)}))
        }
        Err(err) => {
            HttpResponse::BadRequest().json(json!({"error": err}))
        }
    }
}

async fn handle_group_by(graph: &Graph, input: &Value, model: &Model, _source: ActionSource) -> HttpResponse {
    match graph.group_by(model.name(), input).await {
        Ok(count) => {
            HttpResponse::Ok().json(json!({"data": j(count)}))
        }
        Err(err) => {
            HttpResponse::BadRequest().json(json!({"error": err}))
        }
    }
}

async fn handle_sign_in(graph: &Graph, input: &Value, model: &Model, conf: &ServerConf) -> HttpResponse {
    let input = input.as_hashmap().unwrap();
    let credentials = input.get("credentials");
    if let None = credentials {
        return Error::missing_required_input_with_type("object", path!["credentials"]).into();
    }
    let credentials = credentials.unwrap();
    if !credentials.is_hashmap() {
        return Error::unexpected_input_type("object", path!["credentials"]).into();
    }
    let credentials = credentials.as_hashmap().unwrap();
    let mut identity_key: Option<&String> = None;
    let mut identity_value: Option<&Value> = None;
    let mut by_key: Option<&String> = None;
    let mut by_value: Option<&Value> = None;
    for (k, v) in credentials {
        if model.auth_identity_keys().contains(k) {
            if identity_key == None {
                identity_key = Some(k);
                identity_value = Some(v);
            } else {
                return Error::unexpected_input_value_with_reason("Multiple auth identity provided", path!["credentials", k]).into();
            }
        } else if model.auth_by_keys().contains(k) {
            if by_key == None {
                by_key = Some(k);
                by_value = Some(v);
            } else {
                return Error::unexpected_input_value_with_reason("Multiple auth checker provided", path!["credentials", k]).into();
            }
        } else {
            return Error::unexpected_input_key(k, path!["credentials", k]).into();
        }
    }
    if identity_key == None {
        return Error::missing_required_input_with_type("auth identity", path!["credentials"]).into();
    } else if by_key == None {
        return Error::missing_required_input_with_type("auth checker", path!["credentials"]).into();
    }
    let by_field = model.field(by_key.unwrap()).unwrap();
    let obj_result = graph.find_unique_internal(model.name(), &teon!({
        "where": {
            identity_key.unwrap(): identity_value.unwrap()
        }
    }), true, Action::from_u32(FIND | SINGLE | ENTRY), ActionSource::ProgramCode).await;
    if let Err(_err) = obj_result {
        return Error::unexpected_input_value("This identity is not found.", path!["credentials", identity_key.unwrap()]).into();
    }
    let obj = obj_result.unwrap();
    let auth_by_arg = by_field.identity_checker.as_ref().unwrap();
    let pipeline = auth_by_arg.as_pipeline().unwrap();
    let action_by_input = by_value.unwrap();
    let ctx = Ctx::initial_state_with_object(obj.clone()).with_value(action_by_input.clone());
    let result = pipeline.process(ctx).await;
    return match result {
        Err(_err) => {
            return Error::unexpected_input_value_with_reason("Authentication failed.", path!["credentials", by_key.unwrap()]).into();
        }
        Ok(_v) => {
            let include = input.get("include");
            let select = input.get("select");
            let obj = obj.refreshed(include, select).await.unwrap();
            let json_data = obj.to_json_internal(&path!["data"]).await;
            let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
            let tson_identifier = obj.identifier();
            let json_identifier: JsonValue = tson_identifier.into();
            let claims = Claims {
                id: json_identifier,
                model: obj.model().name().to_string(),
                exp
            };
            if conf.jwt_secret.as_ref().is_none() {
                return super::super::error::Error::internal_server_error("Missing JWT secret.").into();
            }
            let token = encode_token(claims, &conf.jwt_secret.as_ref().unwrap());
            HttpResponse::Ok().json(json!({
            "meta": {
                "token": token
            },
            "data": j(json_data.unwrap())
        }))
        }
    }
}

async fn handle_identity(_graph: &Graph, input: &Value, model: &Model, _conf: &ServerConf, source: ActionSource) -> HttpResponse {
    let identity = source.as_identity();
    if let Some(identity) = identity {
        if identity.model() != model {
            return HttpResponse::Unauthorized().json(json!({"error": Error::wrong_identity_model()}));
        }
        let select = input.get("select");
        let include = input.get("include");
        let refreshed = identity.refreshed(include, select).await.unwrap();
        let json_data = refreshed.to_json_internal(&path!["data"]).await;
        HttpResponse::Ok().json(json!({
            "data": j(json_data.unwrap())
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "data": null
        }))
    }
}

pub fn make_app(graph: Graph, conf: ServerConf) ->  App<impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<BoxBody>,
    Config = (),
    InitError = (),
    Error = actix_web::Error,
> + 'static> {
    let leaked_graph = Box::leak(Box::new(graph));
    let leaked_conf = Box::leak(Box::new(conf));
    Graph::set_current(leaked_graph);
    make_app_inner(leaked_graph, leaked_conf)
}

fn make_app_inner(graph: &'static Graph, conf: &'static ServerConf) -> App<impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<BoxBody>,
    Config = (),
    InitError = (),
    Error = actix_web::Error,
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
                    return Error::destination_not_found().into();
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
                return Error::destination_not_found().into();
            }
            let path_components = path_components(&path);
            let first_component = path_components.get(1).unwrap();
            if !(path_components.len() == 3 && first_component == &"action") {
                log_unhandled(start, r.method().as_str(), &path, 404);
                return Error::destination_not_found().into();
            }
            let model_url_segment_name = path_components[0];
            let action_segment_name = path_components[2];
            let action = Action::handler_from_name(action_segment_name);
            let action = match action {
                Some(a) => a,
                None => {
                    log_unhandled(start, r.method().as_str(), &path, 404);
                    return Error::destination_not_found().into();
                }
            };
            let model_def = match graph.model(model_url_segment_name) {
                Some(name) => name,
                None => {
                    log_unhandled(start, r.method().as_str(), &path, 404);
                    return Error::destination_not_found().into();
                }
            };
            if !model_def.has_action(action) || model_def.is_teo_internal() {
                log_unhandled(start, r.method().as_str(), &path, 400);
                return Error::destination_not_found().into();
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
                        .json(json!({"error": Error::internal_server_error("Memory overflow.".to_string())}));
                }
                body.extend_from_slice(&chunk);
            }
            let parsed_body: Result<JsonValue, serde_json::Error> = serde_json::from_slice(&body);
            let parsed_body = match parsed_body {
                Ok(b) => b,
                Err(_) => {
                    log_unhandled(start, r.method().as_str(), &path, 400);
                    return HttpResponse::BadRequest().json(json!({"error": Error::incorrect_json_format()}));
                }
            };

            if !parsed_body.is_object() {
                log_unhandled(start, r.method().as_str(), &path, 400);
                return HttpResponse::BadRequest().json(json!({"error": Error::unexpected_input_root_type("object")}));
            }
            let identity = match get_identity(&r, &graph, conf).await {
                Ok(identity) => { identity },
                Err(err) => return HttpResponse::Unauthorized().json(json!({"error": err }))
            };

            let parsed_body = match Decoder::decode_action_arg(model_def, graph, action, &parsed_body) {
                Ok(body) => body,
                Err(err) => return err.into()
            };
            let (transformed_body, transformed_action) = if model_def.has_action_transformers() || parsed_body.as_hashmap().unwrap().get("include").is_some() {
                if ((action.to_u32() == CREATE_MANY_HANDLER) || (action.to_u32() == CREATE_HANDLER)) && (parsed_body.get("create").unwrap().is_vec()) {
                    // create with many items
                    let entries = parsed_body.get("create").unwrap().as_vec().unwrap();
                    let mut transformed_entries: Vec<Value> = vec![];
                    let mut new_action = action;
                    for (_index, entry) in entries.iter().enumerate() {
                        let ctx = Ctx::initial_state_with_value(teon!({"create": entry})).with_action(action);
                        match model_def.transformed_action(ctx).await {
                            Ok(result) => {
                                transformed_entries.push(result.0.get("create").unwrap().clone());
                                new_action = result.1;
                            },
                            Err(err) => return err.into(),
                        }
                    }
                    let mut new_val = parsed_body.clone();
                    new_val.as_hashmap_mut().unwrap().insert("create".to_owned(), Value::Vec(transformed_entries));
                    (new_val, new_action)
                } else {
                    let ctx = Ctx::initial_state_with_value(parsed_body).with_action(action);
                    match model_def.transformed_action(ctx).await {
                        Ok(result) => result,
                        Err(err) => return err.into(),
                    }
                }
            } else {
                (parsed_body, action)
            };
            let source = ActionSource::Identity(identity);
            match transformed_action.to_u32() {
                FIND_UNIQUE_HANDLER => {
                    let result = handle_find_unique(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    return result;
                }
                FIND_FIRST_HANDLER => {
                    let result = handle_find_first(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    result
                }
                FIND_MANY_HANDLER => {
                    let result = handle_find_many(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    result
                }
                CREATE_HANDLER => {
                    let result = handle_create(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    result
                }
                UPDATE_HANDLER => {
                    let result = handle_update(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    result
                }
                UPSERT_HANDLER => {
                    let result = handle_upsert(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    result
                }
                DELETE_HANDLER => {
                    let result = handle_delete(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    result
                }
                CREATE_MANY_HANDLER => {
                    let result = handle_create_many(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    result
                }
                UPDATE_MANY_HANDLER => {
                    let result = handle_update_many(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    result
                }
                DELETE_MANY_HANDLER => {
                    let result = handle_delete_many(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    result
                }
                COUNT_HANDLER => {
                    let result = handle_count(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    result
                }
                AGGREGATE_HANDLER => {
                    let result = handle_aggregate(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    result
                }
                GROUP_BY_HANDLER => {
                    let result = handle_group_by(&graph, &transformed_body, model_def, source.clone()).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    result
                }
                SIGN_IN_HANDLER => {
                    let result = handle_sign_in(&graph, &transformed_body, model_def, conf).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    result
                }
                IDENTITY_HANDLER => {
                    let result = handle_identity(&graph, &transformed_body, model_def, conf, source.clone()).await;
                    log_request(start, action.as_handler_str(), model_def.name(), result.status().as_u16());
                    result
                }
                _ => unreachable!()
            }
        }));
    app
}

async fn server_start_message(port: u16, environment_version: EnvironmentVersion, entrance: Entrance) -> Result<(), std::io::Error> {
    // Introducing
    let now: DateTime<Local> = Local::now();
    let now_formatted = format!("{now}").dimmed();
    let teo_version = env!("CARGO_PKG_VERSION");
    let teo = format!("Teo {}", teo_version);
    println!("{} {} ({}, {})", now_formatted, teo, environment_version.to_string(), entrance.to_str());
    // Listening
    let now: DateTime<Local> = Local::now();
    let now_formatted = format!("{now}").dimmed();
    let port_str = format!("{port}").bold();
    let text = "Listening";
    println!("{} {} on port {}", now_formatted, text, port_str);
    Ok(())
}

pub(crate) async fn serve(
    graph: Graph,
    conf: ServerConf,
    environment_version: EnvironmentVersion,
    entrance: Entrance,
    no_migration: bool,
    before_server_start: Option<Arc<dyn AsyncCallbackWithoutArgs>>,
) -> Result<(), std::io::Error> {
    if !no_migration {
        migrate(graph.to_mut(), false).await;
    }
    let leaked_graph = Box::leak(Box::new(graph.clone()));
    Graph::set_current(leaked_graph);
    if let Some(cb) = before_server_start {
        match cb.call().await {
            Ok(()) => (),
            Err(err) => return Err(std::io::Error::new(ErrorKind::Other, err.message())),
        }
    }
    let bind = conf.bind.clone();
    let port = bind.1;
    let server = HttpServer::new(move || {
        make_app(graph.clone(), conf.clone())
    })
        .bind(bind)
        .unwrap()
        .run();
    let result = future::join(server, server_start_message(port, environment_version, entrance)).await;
    result.0
}
