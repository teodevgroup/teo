pub(crate) mod response;
pub(crate) mod jwt_token;
pub(crate) mod test_context;
pub(crate) mod conf;

use std::future::Future;
use crate::core::result::Result;
use std::sync::Arc;
use futures_util::{future, TryFutureExt};
use std::time::SystemTime;
use actix_http::body::MessageBody;
use actix_http::{HttpMessage, Method};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::DefaultHeaders;
use actix_web::dev::Service;
use futures_util::FutureExt;
use chrono::{DateTime, Duration, Local, Utc};
use colored::Colorize;
use futures_util::StreamExt;
use indexmap::IndexMap;
use key_path::{KeyPath, path};
use serde_json::Value as JsonValue;
use crate::core::action::{
    Action, CREATE, DELETE, ENTRY, FIND, IDENTITY, MANY, SINGLE, UPDATE, UPSERT,
    FIND_UNIQUE_HANDLER, FIND_FIRST_HANDLER, FIND_MANY_HANDLER, CREATE_HANDLER, UPDATE_HANDLER,
    UPSERT_HANDLER, DELETE_HANDLER, CREATE_MANY_HANDLER, UPDATE_MANY_HANDLER, DELETE_MANY_HANDLER,
    COUNT_HANDLER, AGGREGATE_HANDLER, GROUP_BY_HANDLER, SIGN_IN_HANDLER, IDENTITY_HANDLER,
};
use crate::core::initiator::Initiator;
use crate::app::cli::command::SeedCommandAction;
use crate::app::app_ctx::AppCtx;
use crate::app::entrance::Entrance;
use crate::app::program::Program;
use crate::app::routes::action_ctx::{ActionCtxBase, ActionHandlerDefTrait};
use crate::app::routes::middleware_ctx::Middleware;
use crate::app::routes::req::Req;
use crate::core::callbacks::types::callback_without_args::AsyncCallbackWithoutArgs;
use crate::core::connector::connection::Connection;
use crate::server::test_context::TestContext;
use self::jwt_token::{Claims, decode_token, encode_token};
use crate::core::graph::Graph;
use crate::core::model::model::Model;
use crate::core::object::{ErrorIfNotFound, Object};
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::error::Error;
use crate::core::teon::decoder::Decoder;
use crate::prelude::{combine_middleware, Res, UserCtx, Value};
use crate::seeder::seed::seed;
use crate::server::conf::ServerConf;
use crate::teon;

fn log_err_and_return_response(start: SystemTime, model: &str, action: &str, err: Error) -> HttpResponse {
    let http_response: HttpResponse = err.into();
    let code = http_response.status().as_u16();
    log_unhandled(start, action, model, code);
    http_response
}

fn log_req_and_return_response<T>(start: SystemTime, model: &str, action: &str, res: T) -> HttpResponse where T: Into<HttpResponse> {
    let http_response = res.into();
    let code = http_response.status().as_u16();
    log_request(start, action, model, code);
    http_response
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

async fn get_identity(r: &HttpRequest, graph: &'static Graph, conf: &ServerConf, connection: Arc<dyn Connection>, req: Req) -> Result<Option<Object>> {
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
    let _model = graph.model(claims.model.as_str()).unwrap();
    let identity = graph.find_unique_internal(
        graph.model(claims.model.as_str()).unwrap().name(),
        &teon!({
            "where": tson_identifier
        }),
        true, Action::from_u32(IDENTITY | FIND | SINGLE | ENTRY), Initiator::ProgramCode(Some(req)), connection).await;
    match identity {
        Err(_) => Err(Error::invalid_auth_token()),
        Ok(identity) => Ok(identity),
    }
}

async fn handle_find_unique(graph: &'static Graph, input: &Value, model: &'static Model, source: Initiator, connection: Arc<dyn Connection>) -> Result<Res> {
    let action = Action::from_u32(FIND | SINGLE | ENTRY);
    let result = graph.find_unique_internal(model.name(), input, false, action, source, connection).await;
    match result {
        Ok(obj) => {
            match obj {
                None => Ok(Res::TeonDataRes(teon!(null))),
                Some(obj) => {
                    let obj_data = obj.to_json_internal(&path!["data"]).await.unwrap();
                    Ok(Res::TeonDataRes(obj_data))
                }
            }
        }
        Err(err) => {
            Err(err)
        }
    }
}

async fn handle_find_first(graph: &'static Graph, input: &Value, model: &'static Model, source: Initiator, connection: Arc<dyn Connection>) -> Result<Res> {
    let action = Action::from_u32(FIND | SINGLE | ENTRY);
    let result = graph.find_first_internal(model.name(), input, false, action, source, connection).await;
    match result {
        Ok(obj) => {
            match obj {
                None => Ok(Res::TeonDataRes(teon!(null))),
                Some(obj) => {
                    let obj_data = obj.to_json_internal(&path!["data"]).await.unwrap();
                    Ok(Res::TeonDataRes(obj_data))
                }
            }
        }
        Err(err) => {
            Err(err)
        }
    }
}

async fn handle_find_many(graph: &'static Graph, input: &Value, model: &'static Model, source: Initiator, connection: Arc<dyn Connection>) -> Result<Res> {
    let action = Action::from_u32(FIND | MANY | ENTRY);
    let result = graph.find_many_internal(model.name(), input, false, action, source, connection.clone()).await;
    match result {
        Ok(results) => {
            let mut count_input = input.clone();
            let count_input_obj = count_input.as_hashmap_mut().unwrap();
            count_input_obj.remove("skip");
            count_input_obj.remove("take");
            count_input_obj.remove("pageSize");
            count_input_obj.remove("pageNumber");
            let count = graph.count(model.name(), &count_input, connection.clone()).await.unwrap();
            let mut meta = teon!({"count": count});
            let page_size = input.get("pageSize");
            if page_size.is_some() {
                let page_size = page_size.unwrap().as_i32().unwrap();
                let count = count as i32;
                let mut number_of_pages = count / page_size;
                if count % page_size != 0 {
                    number_of_pages += 1;
                }
                meta.as_hashmap_mut().unwrap().insert("numberOfPages".to_string(), number_of_pages.into());
            }

            let mut result_json: Vec<Value> = vec![];
            for (index, result) in results.iter().enumerate() {
                match result.to_json_internal(&path!["data", index]).await {
                    Ok(result) => result_json.push(result),
                    Err(_) => return Err(Error::permission_error(path!["data"], "not allowed to read")),
                }
            }
            Ok(Res::TeonDataMetaRes(Value::Vec(result_json), meta))
        },
        Err(err) => Err(err)
    }
}

async fn handle_create_internal<'a>(graph: &'static Graph, create: Option<&'a Value>, include: Option<&'a Value>, select: Option<&'a Value>, model: &'static Model, path: &'a KeyPath<'_>, action: Action, action_source: Initiator, connection: Arc<dyn Connection>) -> Result<Value> {
    let obj = graph.new_object(model.name(), action, action_source, connection)?;
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
    obj.save_with_session_and_path(path).await?;
    let refetched = obj.refreshed(include, select).await?;
    refetched.to_json_internal(&path!["data"]).await
}

async fn handle_create(graph: &'static Graph, input: &Value, model: &'static Model, source: Initiator, connection: Arc<dyn Connection>) -> Result<Res> {
    let transaction = connection.transaction().await.unwrap();
    let action = Action::from_u32(CREATE | ENTRY | SINGLE);
    let input = input.as_hashmap().unwrap();
    let create = input.get("create");
    let include = input.get("include");
    let select = input.get("select");
    let result = handle_create_internal(graph, create, include, select, model, &path!["create"], action, source, transaction.clone()).await;
    transaction.clone().commit().await.unwrap();
    match result {
        Ok(result) => Ok(Res::TeonDataRes(result)),
        Err(err) => Err(err),
    }
}

async fn handle_update_internal<'a>(_graph: &'static Graph, object: Object, update: Option<&'a Value>, include: Option<&'a Value>, select: Option<&'a Value>, _where: Option<&'a Value>, _model: &'static Model) -> Result<Value> {
    let empty = teon!({});
    let updater = if update.is_some() { update.unwrap() } else { &empty };
    object.set_teon_with_path(updater, &path!["update"]).await?;
    object.save().await.unwrap();
    let refetched = object.refreshed(include, select).await?;
    refetched.to_json_internal(&path!["data"]).await
}

async fn handle_update(graph: &'static Graph, input: &Value, model: &'static Model, source: Initiator, connection: Arc<dyn Connection>) -> Result<Res> {
    let transaction = connection.transaction().await.unwrap();
    let action = Action::from_u32(UPDATE | ENTRY | SINGLE);
    let result = graph.find_unique_internal(model.name(), input, true, action, source, transaction.clone()).await.into_not_found_error();
    if result.is_err() {
        return Err(result.err().unwrap());
    }
    let result = result.unwrap();
    let update = input.get("update");
    let include = input.get("include");
    let select = input.get("select");
    let r#where = input.get("where");
    let update_result = handle_update_internal(graph, result.clone(), update, include, select, r#where, model).await;
    transaction.clone().commit().await.unwrap();
    match update_result {
        Ok(value) => Ok(Res::TeonDataRes(value)),
        Err(err) => Err(err),
    }
}

async fn handle_upsert(graph: &'static Graph, input: &Value, model: &'static Model, source: Initiator, connection: Arc<dyn Connection>) -> Result<Res> {
    let transaction = connection.transaction().await.unwrap();
    let action = Action::from_u32(UPSERT | UPDATE | ENTRY | SINGLE);
    let result = graph.find_unique_internal(model.name(), input, true, action, source.clone(), transaction.clone()).await.into_not_found_error();
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
                            let json_val = refetched.to_json_internal(&path!["data"]).await.unwrap();
                            transaction.clone().commit().await.unwrap();
                            Ok(Res::TeonDataRes(json_val))
                        }
                        Err(err) => {
                            transaction.clone().commit().await.unwrap();
                            Err(err)
                        }
                    }
                }
                Err(err) => {
                    transaction.clone().commit().await.unwrap();
                    Err(err)
                }
            }
        }
        Err(_) => {
            let create = input.get("create");
            let action = Action::from_u32(UPSERT | CREATE | ENTRY | SINGLE);
            let obj = graph.new_object(model.name(), action, source, transaction.clone()).unwrap();
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
                            let json_data = refetched.to_json_internal(&path!["data"]).await.unwrap();
                            transaction.clone().commit().await.unwrap();
                            return Ok(Res::TeonDataRes(json_data));
                        }
                        Err(err) => {
                            transaction.clone().commit().await.unwrap();
                            return Err(err);
                        }
                    }
                }
                Err(err) => {
                    transaction.clone().commit().await.unwrap();
                    return Err(err);
                }
            }
        }
    }
}

async fn handle_delete(graph: &'static Graph, input: &Value, model: &'static Model, source: Initiator, connection: Arc<dyn Connection>) -> Result<Res> {
    let transaction = connection.transaction().await.unwrap();
    let action = Action::from_u32(DELETE | SINGLE | ENTRY);
    let result = graph.find_unique_internal(model.name(), input, true, action, source, transaction.clone()).await.into_not_found_error();
    if result.is_err() {
        transaction.clone().commit().await.unwrap();
        return Err(result.err().unwrap())
    }
    let result = result.unwrap();
    // find the object here
    return match result.delete_internal(path!["delete"]).await {
        Ok(_) => {
            transaction.clone().commit().await.unwrap();
            let json_data = result.to_json_internal(&path!["data"]).await.unwrap();
            Ok(Res::TeonDataRes(json_data))
        }
        Err(err) => {
            transaction.clone().commit().await.unwrap();
            Err(err)
        }
    }
}

async fn handle_create_many(graph: &'static Graph, input: &Value, model: &'static Model, source: Initiator, connection: Arc<dyn Connection>) -> Result<Res> {
    let transaction = connection.transaction().await.unwrap();
    let action = Action::from_u32(CREATE | MANY | ENTRY);
    let input = input.as_hashmap().unwrap();
    let create = input.get("create");
    let include = input.get("include");
    let select = input.get("select");
    if create.is_none() {
        let err = Error::missing_required_input_with_type("array", path!["create"]);
        return Err(err);
    }
    let create = create.unwrap();
    if !create.is_vec() {
        let err = Error::unexpected_input_type("array", path!["create"]);
        return Err(err);
    }
    let create = create.as_vec().unwrap();
    let mut count = 0;
    let mut ret_data: Vec<Value> = vec![];
    for (index, val) in create.iter().enumerate() {
        let result = handle_create_internal(graph, Some(val), include, select, model, &path!["create", index], action, source.clone(), transaction.clone()).await;
        match result {
            Err(_err) => {
                //println!("{:?}", err.errors);
            },
            Ok(val) => {
                count += 1;
                ret_data.push(val);
            }
        }
    }
    transaction.commit().await.unwrap();
    let json_ret_data = Value::Vec(ret_data);
    Ok(Res::TeonDataMetaRes(json_ret_data, Value::I32(count)))
}

async fn handle_update_many(graph: &'static Graph, input: &Value, model: &'static Model, source: Initiator, connection: Arc<dyn Connection>) -> Result<Res> {
    let transaction = connection.transaction().await.unwrap();
    let action = Action::from_u32(UPDATE | MANY | ENTRY);
    let result = graph.find_many_internal(model.name(), input, true, action, source, transaction.clone()).await;
    if result.is_err() {
        transaction.commit().await.unwrap();
        return Err(result.err().unwrap());
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
    transaction.commit().await.unwrap();
    Ok(Res::TeonDataMetaRes(Value::Vec(ret_data), teon!(count)))
}

async fn handle_delete_many(graph: &'static Graph, input: &Value, model: &'static Model, source: Initiator, connection: Arc<dyn Connection>) -> Result<Res> {
    let transaction = connection.transaction().await.unwrap();
    let action = Action::from_u32(DELETE | MANY | ENTRY);
    let result = graph.find_many_internal(model.name(), input, true, action, source, transaction.clone()).await;
    if result.is_err() {
        transaction.commit().await.unwrap();
        return Err(result.err().unwrap());
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
    transaction.commit().await.unwrap();
    return Ok(Res::TeonDataMetaRes(Value::Vec(retval), teon!(count)));
}

async fn handle_count(graph: &'static Graph, input: &Value, model: &'static Model, _source: Initiator, connection: Arc<dyn Connection>) -> Result<Res> {
    graph.count(model.name(), input, connection).await.map(|count| Res::teon_data(teon!(count)))
}

async fn handle_aggregate(graph: &'static Graph, input: &Value, model: &'static Model, _source: Initiator, connection: Arc<dyn Connection>) -> Result<Res> {
    graph.aggregate(model.name(), input, connection).await.map(|result| Res::teon_data(result))
}

async fn handle_group_by(graph: &'static Graph, input: &Value, model: &'static Model, _source: Initiator, connection: Arc<dyn Connection>) -> Result<Res> {
    graph.group_by(model.name(), input, connection).await.map(|result| Res::teon_data(result))
}

async fn handle_sign_in<'a>(graph: &'static Graph, input: &'a Value, model: &'static Model, conf: &'a ServerConf, connection: Arc<dyn Connection>, req: Req) -> Result<Res> {
    let input = input.as_hashmap().unwrap();
    let credentials = input.get("credentials");
    if let None = credentials {
        return Err(Error::missing_required_input_with_type("object", path!["credentials"]));
    }
    let credentials = credentials.unwrap();
    if !credentials.is_hashmap() {
        return Err(Error::unexpected_input_type("object", path!["credentials"]));
    }
    let credentials = credentials.as_hashmap().unwrap();
    let mut identity_key: Option<&String> = None;
    let mut identity_value: Option<&Value> = None;
    let mut by_key: Option<&String> = None;
    let mut by_value: Option<&Value> = None;
    for (k, v) in credentials {
        if model.auth_identity_keys().contains(&k.as_str()) {
            if identity_key == None {
                identity_key = Some(k);
                identity_value = Some(v);
            } else {
                return Err(Error::unexpected_input_value_with_reason("Multiple auth identity provided", path!["credentials", k]));
            }
        } else if model.auth_by_keys().contains(&k.as_str()) {
            if by_key == None {
                by_key = Some(k);
                by_value = Some(v);
            } else {
                return Err(Error::unexpected_input_value_with_reason("Multiple auth checker provided", path!["credentials", k]));
            }
        } else {
            return Err(Error::unexpected_input_key(k, path!["credentials", k]));
        }
    }
    if identity_key == None {
        return Err(Error::missing_required_input_with_type("auth identity", path!["credentials"]));
    } else if by_key == None {
        return Err(Error::missing_required_input_with_type("auth checker", path!["credentials"]));
    }
    let by_field = model.field(by_key.unwrap()).unwrap();
    let obj_result = graph.find_unique_internal(model.name(), &teon!({
        "where": {
            identity_key.unwrap(): identity_value.unwrap()
        }
    }), true, Action::from_u32(FIND | SINGLE | ENTRY), Initiator::ProgramCode(Some(req.clone())), connection.clone()).await.into_not_found_error();
    if let Err(_err) = obj_result {
        return Err(Error::unexpected_input_value("This identity is not found.", path!["credentials", identity_key.unwrap()]));
    }
    let obj = obj_result.unwrap();
    let auth_by_arg = by_field.identity_checker.as_ref().unwrap();
    let pipeline = auth_by_arg.as_pipeline().unwrap();
    let action_by_input = by_value.unwrap();
    let ctx = PipelineCtx::initial_state_with_object(obj.clone(), connection.clone(), Some(req)).with_value(action_by_input.clone());
    let result = pipeline.process(ctx).await;
    return match result {
        Err(_err) => {
            return Err(Error::unexpected_input_value_with_reason("Authentication failed.", path!["credentials", by_key.unwrap()]));
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
                return Err(Error::internal_server_error("Missing JWT secret."));
            }
            let token = encode_token(claims, &conf.jwt_secret.as_ref().unwrap());
            return Ok(Res::teon_data_meta(json_data?, teon!(token)))
        }
    }
}

async fn handle_identity<'a>(_graph: &'static Graph, input: &'a Value, model: &'static Model, _conf: &'a ServerConf, source: Initiator, connection: Arc<dyn Connection>) -> Result<Res> {
    let identity = source.as_identity();
    return if let Some(identity) = identity {
        if identity.model() != model {
            return Err(Error::wrong_identity_model());
        }
        let select = input.get("select");
        let include = input.get("include");
        let refreshed = identity.refreshed(include, select).await.unwrap();
        let json_data = refreshed.to_json_internal(&path!["data"]).await;
        Ok(Res::TeonDataRes(json_data?))
    } else {
        Ok(Res::TeonDataRes(teon!(null)))
    }
}

async fn handler(req_ctx: ReqCtx) -> Result<Res> {
    let app_ctx = AppCtx::get()?;
    let graph = app_ctx.graph()?;
    let test_context = app_ctx.test_context();
    let conf = app_ctx.server_conf()?;
    let connection = req_ctx.connection.clone();
    let req = req_ctx.req.clone();
    let identity = req_ctx.identity.clone();
    let source = Initiator::Identity(identity, req.clone());
    let action = req_ctx.transformed_action.unwrap();
    let body = &req_ctx.transformed_teon_body;
    let model_def = graph.model(req_ctx.path_components.model.as_str()).unwrap();
    match action.to_u32() {
        FIND_UNIQUE_HANDLER => {
            let result = handle_find_unique(&graph, &body, model_def, source.clone(), connection.clone()).await;
            let _ = reset_after_query_if_needed(test_context, graph, connection.clone()).await;
            return result;
        }
        FIND_FIRST_HANDLER => {
            let result = handle_find_first(&graph, &body, model_def, source.clone(), connection.clone()).await;
            let _ = reset_after_query_if_needed(test_context, graph, connection.clone()).await;
            result
        }
        FIND_MANY_HANDLER => {
            let result = handle_find_many(&graph, &body, model_def, source.clone(), connection.clone()).await;
            let _ = reset_after_query_if_needed(test_context, graph, connection.clone()).await;
            result
        }
        CREATE_HANDLER => {
            let result = handle_create(&graph, &body, model_def, source.clone(), connection.clone()).await;
            let _ = reset_after_mutation_if_needed(test_context, graph, connection.clone()).await;
            result
        }
        UPDATE_HANDLER => {
            let result = handle_update(&graph, &body, model_def, source.clone(), connection.clone()).await;
            let _ = reset_after_mutation_if_needed(test_context, graph, connection.clone()).await;
            result
        }
        UPSERT_HANDLER => {
            let result = handle_upsert(&graph, &body, model_def, source.clone(), connection.clone()).await;
            let _ = reset_after_mutation_if_needed(test_context, graph, connection.clone()).await;
            result
        }
        DELETE_HANDLER => {
            let result = handle_delete(&graph, &body, model_def, source.clone(), connection.clone()).await;
            let _ = reset_after_mutation_if_needed(test_context, graph, connection.clone()).await;
            result
        }
        CREATE_MANY_HANDLER => {
            let result = handle_create_many(&graph, &body, model_def, source.clone(), connection.clone()).await;
            let _ = reset_after_mutation_if_needed(test_context, graph, connection.clone()).await;
            result
        }
        UPDATE_MANY_HANDLER => {
            let result = handle_update_many(&graph, &body, model_def, source.clone(), connection.clone()).await;
            let _ = reset_after_mutation_if_needed(test_context, graph, connection.clone()).await;
            result
        }
        DELETE_MANY_HANDLER => {
            let result = handle_delete_many(&graph, &body, model_def, source.clone(), connection.clone()).await;
            let _ = reset_after_mutation_if_needed(test_context, graph, connection.clone()).await;
            result
        }
        COUNT_HANDLER => handle_count(&graph, &body, model_def, source.clone(), connection).await,
        AGGREGATE_HANDLER => handle_aggregate(&graph, &body, model_def, source.clone(), connection).await,
        GROUP_BY_HANDLER => handle_group_by(&graph, &body, model_def, source.clone(), connection).await,
        SIGN_IN_HANDLER => handle_sign_in(&graph, &body, model_def, conf, connection, req.clone()).await,
        IDENTITY_HANDLER => handle_identity(&graph, &body, model_def, conf, source.clone(), connection).await,
        _ => unreachable!()
    }

}

fn make_app(
    graph: &'static Graph,
    conf: &'static ServerConf,
    middlewares: &'static IndexMap<&'static str, &'static dyn Middleware>,
    action_defs: &'static Vec<Arc<dyn ActionHandlerDefTrait>>,
    test_context: Option<&'static TestContext>
) -> App<impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<impl MessageBody>,
    Config = (),
    InitError = (),
    Error = actix_web::Error,
> + 'static> {
    let combined_middleware = combine_middleware(middlewares.clone());
    let app = App::new()
        .wrap(DefaultHeaders::new()
            .add(("Access-Control-Allow-Origin", "*"))
            .add(("Access-Control-Allow-Methods", "OPTIONS, POST, GET"))
            .add(("Access-Control-Allow-Headers", "*"))
            .add(("Access-Control-Max-Age", "86400")))
        .default_service(web::route().to(move |http_request: HttpRequest, mut payload: web::Payload| async move {
            // This is where our stack begins
            let start = SystemTime::now();
            // Validate method
            let method = http_request.method();
            if (method != Method::POST) && (method != Method::OPTIONS) {
                return log_err_and_return_response(start, method.as_str(), http_request.path(), Error::destination_not_found());
            }
            // Validate path
            let path_components = match parse_path(http_request.path(), conf.path_prefix) {
                Ok(components) => components,
                Err(_err) => return log_err_and_return_response(start, method.as_str(), http_request.path(), Error::destination_not_found()),
            };
            // Return for OPTIONS
            if http_request.method() == Method::OPTIONS {
                return Res::EmptyRes.into();
            }
            // Pass data
            // Acquire a connection
            let connection = AppCtx::get().unwrap().connector().unwrap().connection().await.unwrap();
            let teo_req = Req::new(http_request.clone());
            let user_ctx = UserCtx::new(connection.clone(), Some(teo_req.clone()));
            // Parse body
            let mut body = web::BytesMut::new();
            while let Some(chunk) = payload.next().await {
                let chunk = chunk.unwrap();
                // limit max size of in-memory payload
                if (body.len() + chunk.len()) > 262_144usize {
                    return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), Error::internal_server_error("Memory overflow."));
                }
                body.extend_from_slice(&chunk);
            }
            let parsed_json_body_result: std::result::Result<JsonValue, serde_json::Error> = serde_json::from_slice(&body);
            let parsed_json_body = match parsed_json_body_result {
                Ok(b) => b,
                Err(_) => {
                    return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), Error::incorrect_json_format());
                }
            };
            if !parsed_json_body.is_object() {
                return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), Error::unexpected_input_root_type("object"));
            }
            // Teo Req
            let teo_req = Req::new(http_request.clone());
            // Identity
            let identity = match get_identity(&http_request, &graph, conf, connection.clone(), teo_req.clone()).await {
                Ok(identity) => { identity },
                Err(err) => return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
            };
            let original_action = Action::handler_from_name(path_components.action.as_str());
            let model_def = match graph.model(path_components.model.as_str()) {
                Ok(m) => Some(m),
                Err(_) => None,
            };
            let original_teon_body = if let (Some(original_action), Some(model_def)) = (original_action, model_def) {
                // Check whether this action is supported by this model
                if !model_def.has_action(original_action) || model_def.is_teo_internal() {
                    return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), Error::destination_not_found());
                }
                // Parse body the predefined way
                match Decoder::decode_action_arg(model_def, graph, original_action, &parsed_json_body) {
                    Ok(body) => body,
                    Err(err) => return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
                }
            } else {
                // Parse body the user defined way
                unreachable!("Currently parsing user defined body is not implemented.")
            };
            let (transformed_teon_body, transformed_action) = if let (Some(original_action), Some(model_def)) = (original_action, model_def) {
                if model_def.has_action_transformers() || original_teon_body.as_hashmap().unwrap().get("include").is_some() {
                    if ((original_action.to_u32() == CREATE_MANY_HANDLER) || (original_action.to_u32() == CREATE_HANDLER)) && (original_teon_body.get("create").unwrap().is_vec()) {
                        // create with many items
                        let entries = original_teon_body.get("create").unwrap().as_vec().unwrap();
                        let mut transformed_entries: Vec<Value> = vec![];
                        let mut new_action = original_action;
                        for (_index, entry) in entries.iter().enumerate() {
                            let ctx = PipelineCtx::initial_state_with_value(teon!({"create": entry}), connection.clone(), Some(teo_req.clone())).with_action(original_action);
                            match model_def.transformed_action(ctx).await {
                                Ok(result) => {
                                    transformed_entries.push(result.0.get("create").unwrap().clone());
                                    new_action = result.1;
                                },
                                Err(err) => return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
                            }
                        }
                        let mut new_val = original_teon_body.clone();
                        new_val.as_hashmap_mut().unwrap().insert("create".to_owned(), Value::Vec(transformed_entries));
                        (new_val, new_action)
                    } else {
                        let ctx = PipelineCtx::initial_state_with_value(original_teon_body, connection.clone(), Some(teo_req.clone())).with_action(original_action);
                        match model_def.transformed_action(ctx).await {
                            Ok(result) => result,
                            Err(err) => return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
                        }
                    }
                } else {
                    (original_teon_body, original_action)
                }
            } else {
                (original_teon_body, original_action.unwrap())
            };
            // Save the request local data into the extension
            let req_ctx = ReqCtx {
                start,
                connection,
                path_components: path_components.clone(),
                req: teo_req,
                user_ctx,
                transformed_action: Some(transformed_action),
                transformed_teon_body,
                identity
            };
            let result = combined_middleware.call(req_ctx, Box::leak(Box::new(handler))).await;
            match result {
                Ok(res) => log_req_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), res),
                Err(err) => log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
            }
      }));
    // for action_def in action_defs {
    //     app = app.route(format!("/{}/action/{}", action_def.group(), action_def.name()).as_str(), web::post().to(move |request: HttpRequest| async move {
    //         let binding = request.clone();
    //         let exts = binding.extensions();
    //         let req_ctx = exts.get::<ReqCtx>().unwrap();
    //         let user_ctx = req_ctx.user_ctx.clone();
    //         let ctx = ActionCtxBase {
    //             req: req_ctx.req.clone(),
    //             user_ctx,
    //         };
    //         let res = action_def.call(ctx).await;
    //         match res {
    //             Ok(ok) => {
    //                 <Res as Into<HttpResponse>>::into(ok)
    //             },
    //             Err(err) => {
    //                 <Error as Into<HttpResponse>>::into(err)
    //             },
    //         }
    //     }));
    // }
    app
}

#[derive(Clone)]
pub(crate) struct PathComponents {
    model: String,
    action: String,
}

fn parse_path(path: &str, prefix: Option<&str>) -> Result<PathComponents> {
    let mut path_striped = if let Some(prefix) = prefix {
        if !path.starts_with(prefix) {
            return Err(Error::destination_not_found());
        } else {
            path.strip_prefix(prefix).unwrap()
        }
    } else {
        path
    };
    if path_striped.ends_with("/") {
        path_striped = path_striped.strip_suffix("/").unwrap();
    }
    if path_striped.starts_with("/") {
        path_striped = path_striped.strip_prefix("/").unwrap();
    }
    let components: Vec<&str> = path_striped.split("/").into_iter().collect();
    if components.len() != 3 {
        return Err(Error::destination_not_found());
    }
    if *components.get(1).unwrap() != "action" {
        return Err(Error::destination_not_found());
    }
    let model = components.get(0).unwrap();
    let action = components.get(2).unwrap();
    Ok(PathComponents { model: model.to_string(), action: action.to_string() })
}

async fn server_start_message(port: u16, environment_version: &'static Program, entrance: &'static Entrance) -> Result<()> {
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
    graph: &'static Graph,
    conf: &'static ServerConf,
    environment_version: &'static Program,
    entrance: &'static Entrance,
    before_server_start: Option<Arc<dyn AsyncCallbackWithoutArgs>>,
    middlewares: &'static IndexMap<&'static str, &'static dyn Middleware>,
    action_defs: &'static Vec<Arc<dyn ActionHandlerDefTrait>>,
    test_context: Option<&'static TestContext>,
) -> Result<()> {
    if let Some(cb) = before_server_start {
        cb.call().await.unwrap();
    }
    let bind = conf.bind.clone();
    let port = bind.1;
    let server = HttpServer::new(move || {
        make_app(graph, conf, middlewares, action_defs, test_context)
    })
        .bind(bind)
        .unwrap()
        .run();
    let result = future::join(server, server_start_message(port, environment_version, entrance)).await;
    result.1
}

async fn reset_after_mutation_if_needed(test_context: Option<&'static TestContext>, graph: &'static Graph, connection: Arc<dyn Connection>) -> Result<()> {
    if let Some(test_context) = test_context {
        if test_context.reset_mode.after_mutation() {
            connection.purge(graph).await.unwrap();
            let _ = seed(SeedCommandAction::Seed, graph, &test_context.datasets, test_context.datasets.iter().map(|d| d.name.clone()).collect()).await;
        }
    }
    Ok(())
}

async fn reset_after_query_if_needed(test_context: Option<&'static TestContext>, graph: &'static Graph, connection: Arc<dyn Connection>) -> Result<()> {
    if let Some(test_context) = test_context {
        if test_context.reset_mode.after_query() {
            connection.purge(graph).await.unwrap();
            let _ = seed(SeedCommandAction::Seed, graph, &test_context.datasets, test_context.datasets.iter().map(|d| d.name.clone()).collect()).await;
        }
    }
    Ok(())
}

#[derive(Clone)]
pub struct ReqCtx {
    pub start: SystemTime,
    pub connection: Arc<dyn Connection>,
    pub(crate) path_components: PathComponents,
    pub req: Req,
    pub user_ctx: UserCtx,
    pub transformed_action: Option<Action>,
    pub transformed_teon_body: Value,
    pub identity: Option<Object>,
}