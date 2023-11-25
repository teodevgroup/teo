pub(crate) mod test_context;

use std::env;
use std::io::Write;
use std::path::PathBuf;
use crate::core::result::Result;
use std::sync::Arc;
use futures_util::{future, TryStreamExt};
use std::time::SystemTime;
use actix_http::body::MessageBody;
use actix_http::{HttpMessage, Method};
use actix_multipart::Multipart;
use actix_web::{App, FromRequest, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::DefaultHeaders;
//use futures_util::FutureExt;
use chrono::{DateTime, Duration, Local, Utc};
use colored::Colorize;
use futures_util::StreamExt;
use indexmap::IndexMap;
use key_path::{KeyPath, path};
use regex::Regex;
use serde_json::{json, Value as JsonValue};
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
use crate::app::program::LanguagePlatform;
use crate::app::routes::middleware_ctx::Middleware;
use crate::app::routes::req::Req;
use crate::app::routes::req_local::ReqLocal;
use crate::core::connector::connection::Connection;
use crate::server::test_context::TestContext;
use self::jwt_token::{Claims, decode_token, encode_token};
use crate::core::graph::Graph;
use crate::core::model::model::Model;
use crate::core::object::{ErrorIfNotFound, Object};
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::error::Error;
use crate::core::decode::custom_action_decoder::transform_custom_action_json_into_teon;
use crate::core::decode::decoder::Decoder;
use crate::parser::ast::action::ActionInputFormat;
use crate::prelude::{combine_middleware, Res, UserCtx, Value};
use crate::seeder::seed::seed;
use crate::server::conf::ServerConf;
use teo_teon::teon;

fn log_err_and_return_response(start: SystemTime, model: &str, action: &str, err: Error) -> HttpResponse {
    let http_response: HttpResponse = err.into();
    let code = http_response.status().as_u16();
    log_unhandled(start, action, model, code);
    http_response
}

fn log_req_and_return_response(start: SystemTime, model: &str, action: &str, res: Res) -> HttpResponse {
    let http_response: HttpResponse = res.into();
    let code = http_response.status().as_u16();
    log_request(start, action, model, code);
    http_response
}

fn log_file_req_and_return_response(start: SystemTime, model: &str, action: &str, req: &HttpRequest, res: Res) -> HttpResponse {
    let http_res = res.into_response(req);
    let code = http_res.status().as_u16();
    log_request(start, action, model, code);
    http_res
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
    let json_identifier = &claims.id;
    let teon_identifier = Decoder::decode_object(AppCtx::get().unwrap().model(claims.model_path()).unwrap().unwrap(), graph, &json_identifier)?;
    let identity = graph.find_unique_internal(
        AppCtx::get().unwrap().model(claims.model_path()).unwrap().unwrap(),
        &teon!({
            "where": teon_identifier
        }),
        true, Action::from_u32(IDENTITY | FIND | SINGLE | ENTRY), Initiator::ProgramCode(Some(req)), connection).await;
    match identity {
        Err(_) => Err(Error::invalid_auth_token()),
        Ok(identity) => Ok(identity),
    }
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
    let obj_result = graph.find_unique_internal(model, &teon!({
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
            Err(Error::unexpected_input_value_with_reason("Authentication failed.", path!["credentials", by_key.unwrap()]))
        }
        Ok(_v) => {
            let include = input.get("include");
            let select = input.get("select");
            let obj = obj.refreshed(include, select).await.unwrap();
            let json_data = obj.to_json_internal(&path!["data"]).await;
            let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
            let teon_identifier = obj.identifier();
            let json_identifier: JsonValue = teon_identifier.into();
            let claims = Claims {
                id: json_identifier,
                model: obj.model().path().iter().map(|s| s.to_string()).collect(),
                exp
            };
            if conf.jwt_secret.as_ref().is_none() {
                return Err(Error::internal_server_error("Missing JWT secret."));
            }
            let token = encode_token(claims, &conf.jwt_secret.as_ref().unwrap());
            Ok(Res::teon_data_meta(json_data?, teon!({"token": token})))
        }
    }
}

async fn handle_identity<'a>(_graph: &'static Graph, input: &'a Value, model: &'static Model, _conf: &'a ServerConf, source: Initiator, _connection: Arc<dyn Connection>) -> Result<Res> {
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
    let graph = app_ctx.graph();
    let test_context = app_ctx.test_context();
    let conf = app_ctx.server_conf()?;
    let connection = req_ctx.connection.clone();
    let req = req_ctx.req.clone();
    let identity = req_ctx.identity.clone();
    let source = Initiator::Identity(identity, req.clone());
    let action = req_ctx.transformed_action.unwrap();
    let body = &req_ctx.transformed_teon_body;
    let model_def = AppCtx::get().unwrap().model(req_ctx.path_components.model_path())?.unwrap();
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
            if method == Method::GET {
                match parse_static_files_path(http_request.path(), conf.path_prefix) {
                    Ok(filepath) => {
                        let path = PathBuf::from(filepath);
                        return if path.is_file() {
                            let res = Res::File(path);
                            log_file_req_and_return_response(start, "GET", http_request.path(), &http_request, res)
                        } else {
                            log_err_and_return_response(start, "GET", http_request.path(), Error::destination_not_found())
                        }
                    },
                    Err(err) => return log_err_and_return_response(start, "GET", http_request.path(), err),
                }
            } else {
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
                let format = if http_request.content_type() == "multipart/form-data" {
                    ActionInputFormat::Form
                } else {
                    ActionInputFormat::Json
                };
                let parsed_json_body = if format.is_json() {
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
                    parsed_json_body
                } else {
                    let mut inner_payload = payload.into_inner();
                    let multipart_result = Multipart::from_request(&http_request, &mut inner_payload).await;
                    let mut multipart = match multipart_result {
                        Ok(multipart) => multipart,
                        Err(err) => return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), Error::incorrect_form_format(err.to_string()))
                    };
                    let mut result_value = json!({});
                    while let Some(mut field) = multipart.try_next().await.unwrap() {
                        // A multipart/form-data stream has to contain `content_disposition`
                        if let Some(filename) = field.content_disposition().get_filename().map(|f| f.to_owned()) {
                            let filepath = env::temp_dir().join(filename.clone()).to_str().unwrap().to_owned();
                            let filepath2 = filepath.clone();
                            // File::create is blocking operation, use threadpool
                            let mut f = web::block(move || std::fs::File::create(&filepath)).await.unwrap().unwrap();
                            // Field in turn is stream of *Bytes* object
                            while let Some(chunk) = field.try_next().await.unwrap() {
                                // filesystem operations are blocking, we have to use threadpool
                                f = web::block(move || f.write_all(&chunk).map(|_| f)).await.unwrap().unwrap();
                            }
                            let owned_field_name = field.name().to_owned();
                            if owned_field_name.ends_with("[]") {
                                let field_name_without_suffix = owned_field_name.strip_suffix("[]").unwrap();
                                if !result_value.as_object_mut().unwrap().contains_key(field_name_without_suffix) {
                                    result_value.as_object_mut().unwrap().insert(field_name_without_suffix.to_owned(), json!([]));
                                }
                                result_value.as_object_mut().unwrap().get_mut(field_name_without_suffix).unwrap().as_array_mut().unwrap().push(json!({
                                    "filepath": filepath2,
                                    "contentType": field.content_type().map(|c| c.to_string()),
                                    "filename": filename,
                                    "filenameExt": field.content_disposition().get_filename_ext().map(|e| e.to_string()),
                                }));
                            } else if owned_field_name.ends_with("]") {
                                let regex = Regex::new("(.*)\\[(.*)\\]").unwrap();
                                let found = regex.captures(&owned_field_name).unwrap();
                                let field_name = found.get(1).unwrap().as_str().to_owned();
                                let dict_name = found.get(2).unwrap().as_str().to_owned();
                                if !result_value.as_object_mut().unwrap().contains_key(&field_name) {
                                    result_value.as_object_mut().unwrap().insert(field_name.clone(), json!([]));
                                }
                                result_value.as_object_mut().unwrap().get_mut(&field_name).unwrap().as_object_mut().unwrap().insert(dict_name, json!({
                                    "filepath": filepath2,
                                    "contentType": field.content_type().map(|c| c.to_string()),
                                    "filename": filename,
                                    "filenameExt": field.content_disposition().get_filename_ext().map(|e| e.to_string()),
                                }));
                            } else {
                                result_value.as_object_mut().unwrap().insert(field.name().to_owned(), json!({
                                    "filepath": filepath2,
                                    "contentType": field.content_type().map(|c| c.to_string()),
                                    "filename": filename,
                                    "filenameExt": field.content_disposition().get_filename_ext().map(|e| e.to_string()),
                                }));
                            }
                        } else {
                            let mut body = web::BytesMut::new();
                            while let Some(chunk) = field.try_next().await.unwrap() {
                                body.extend_from_slice(&chunk);
                            }
                            result_value.as_object_mut().unwrap().insert(field.name().to_owned(), serde_json::Value::String(String::from_utf8(body.as_ref().to_vec()).unwrap()));
                        }
                    }
                    result_value
                };

                // Teo Req
                let teo_req = Req::new(http_request.clone());
                // Identity
                let identity = match get_identity(&http_request, &graph, conf, connection.clone(), teo_req.clone()).await {
                    Ok(identity) => { identity },
                    Err(err) => return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
                };
                let original_action = Action::handler_from_name(path_components.action.as_str());
                let model_def = AppCtx::get().unwrap().model(path_components.model_path()).unwrap();
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
                    let app_ctx = AppCtx::get().unwrap();
                    if app_ctx.main_namespace().has_custom_action_declaration_for(path_components.model.as_str(), path_components.action.as_str()) {
                        let custom_action_declaration = app_ctx.main_namespace().get_custom_action_declaration_for(path_components.model.as_str(), path_components.action.as_str());
                        let input = &custom_action_declaration.input_fields;
                        let result = transform_custom_action_json_into_teon(&parsed_json_body, input, &path![]);
                        match result {
                            Err(err) => return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
                            Ok(value) => value,
                        }
                    } else {
                        return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), Error::destination_not_found());
                    }
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
                            (new_val, Some(new_action))
                        } else {
                            let ctx = PipelineCtx::initial_state_with_value(original_teon_body, connection.clone(), Some(teo_req.clone())).with_action(original_action);
                            match model_def.transformed_action(ctx).await {
                                Ok(result) => (result.0, Some(result.1)),
                                Err(err) => return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
                            }
                        }
                    } else {
                        (original_teon_body, Some(original_action))
                    }
                } else {
                    (original_teon_body, original_action)
                };
                // Save the request local data into the extension
                let req_ctx = ReqCtx {
                    start,
                    connection,
                    path_components: path_components.clone(),
                    req: teo_req,
                    user_ctx,
                    transformed_action,
                    transformed_teon_body,
                    identity,
                    req_local: ReqLocal::new()
                };
                let result = if AppCtx::get().unwrap().main_namespace().has_action_handler_for(&path_components.model, &path_components.action) {
                    combined_middleware.call(req_ctx, &|req_ctx: ReqCtx| async {
                        let path_components = req_ctx.path_components.clone();
                        let action_def = AppCtx::get().unwrap().main_namespace().get_action_handler(&path_components.model, &path_components.action);
                        action_def.call(req_ctx).await
                    }).await
                } else {
                    combined_middleware.call(req_ctx, &handler).await
                };
                match result {
                    Ok(res) => log_req_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), res),
                    Err(err) => log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
                }
            }
      }));
    app
}

#[derive(Clone)]
pub(crate) struct PathComponents {
    ns: Vec<String>,
    model: String,
    action: String,
}

impl PathComponents {
    pub(crate) fn model_path(&self) -> Vec<&str> {
        let mut result: Vec<&str> = self.ns.iter().map(|n| n.as_str()).collect();
        result.push(self.model.as_str());
        result
    }
}

fn parse_static_files_path<'a>(path: &'a str, prefix: Option<&'a str>) -> Result<PathBuf> {
    let purified_path = if let Some(prefix) = prefix {
        if path.starts_with(prefix) {
            PathBuf::from(path.strip_prefix(prefix).unwrap())
        } else {
            PathBuf::from(path)
        }
    } else {
        PathBuf::from(path)
    };
    for (k, v) in AppCtx::get()?.static_files() {
        if purified_path.starts_with(k) {
            let content_path = purified_path.strip_prefix(k).unwrap();
            return Ok(PathBuf::from(v).join(content_path));
        }
    }
    Err(Error::destination_not_found())
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
    if components.len() < 2 {
        return Err(Error::destination_not_found());
    }
    let model = components.get(components.len() - 2).unwrap();
    let action = components.get(components.len() - 1).unwrap();
    let ns = components.as_slice()[..components.len() - 2].iter().map(|s| s.to_string()).collect();
    Ok(PathComponents {
        ns,
        model: model.to_string(),
        action: action.to_string()
    })
}

async fn server_start_message(port: u16, environment_version: &'static LanguagePlatform, entrance: &'static Entrance) -> Result<()> {
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

async fn reset_after_mutation_if_needed(test_context: Option<&'static TestContext>, graph: &'static Graph, connection: Arc<dyn Connection>) -> Result<()> {
    if let Some(test_context) = test_context {
        if test_context.reset_mode.after_mutation() {
            connection.purge().await.unwrap();
            seed(SeedCommandAction::Seed, graph, test_context.datasets.iter().collect(), test_context.datasets.iter().map(|d| d.name.join(".")).collect()).await?;
        }
    }
    Ok(())
}

async fn reset_after_query_if_needed(test_context: Option<&'static TestContext>, graph: &'static Graph, connection: Arc<dyn Connection>) -> Result<()> {
    if let Some(test_context) = test_context {
        if test_context.reset_mode.after_query() {
            connection.purge().await.unwrap();
            seed(SeedCommandAction::Seed, graph, test_context.datasets.iter().collect(), test_context.datasets.iter().map(|d| d.name.join(".")).collect()).await?;
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
    pub req_local: ReqLocal,
}

impl ReqCtx {
    pub fn req_local(&self) -> ReqLocal {
        self.req_local.clone()
    }
}

unsafe impl Send for ReqCtx { }
unsafe impl Sync for ReqCtx { }