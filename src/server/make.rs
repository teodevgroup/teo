use std::sync::Arc;
use std::time::SystemTime;
use actix_web::dev::Service;
use futures_util::FutureExt;
use colored::Colorize;
use futures_util::future;
use serde_json::{Value as JsonValue};
use teo_result::{Error, Result};
use teo_runtime::config::server::Server;
use teo_runtime::namespace::Namespace;
use actix_http::body::MessageBody;
use actix_http::{HttpMessage, Method as HttpMethod};
use actix_web::{App, FromRequest, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::DefaultHeaders;
use teo_parser::ast::handler::HandlerInputFormat;
use teo_runtime::action::Action;
use teo_runtime::handler::action::builtin_action_handler_from_name;
use teo_runtime::handler::Handler;
use teo_runtime::handler::handler::Method;
use teo_runtime::{connection, request};
use teo_runtime::connection::transaction;
use teo_runtime::handler::default::{create, find_first, find_many, find_unique, update, upsert, copy, create_many, update_many, copy_many, delete_many, count, aggregate, group_by, delete};
use teo_runtime::model::Model;
use teo_runtime::response::Response;
use teo_teon::Value;
use crate::cli::entrance::Entrance;
use crate::cli::runtime_version::RuntimeVersion;
use crate::purge;
use crate::seeder::seed::seed;
use crate::server::parse::{parse_form_body, parse_json_body};
use teo_runtime::handler::input::{validate_and_transform_json_input_for_handler, validate_and_transform_json_input_for_builtin_action};
use teo_runtime::handler::r#match::HandlerMatch;
use teo_runtime::schema::load::load_data_sets::load_data_sets;
use crate::app::Ctx;
use crate::app::database::connect_databases;
use crate::cli::command::SeedCommandAction;
use crate::message::{info_message, request_message, unhandled_request_message};
use crate::server::error::WrapError;
use crate::server::request::RequestImpl;
use crate::server::responder::IntoHttpResponse;
use teo_runtime::error_runtime_ext::ErrorRuntimeExt;

fn make_server_app(
    main_namespace: &'static Namespace,
    conf: &'static Server,
) -> App<impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<impl MessageBody>,
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
        .wrap_fn(|req, srv| {
            let start = SystemTime::now();
            let fut = srv.call(req);
            async move {
                let res = fut.await?;
                {
                    let binding = res.request().extensions();
                    let handler_found_info = binding.get::<HandlerMatch>().clone();
                    let time_elapsed = SystemTime::now().duration_since(start).unwrap();
                    let path = res.request().path();
                    let method = res.request().method().as_str();
                    if let Some(handler_found_info) = handler_found_info {
                        request_message(time_elapsed, method, path, &handler_found_info.path, handler_found_info.name.as_str(), res.response().status().as_u16());
                    } else {
                        unhandled_request_message(time_elapsed, method, path, res.response().status().as_u16());
                    }
                }
                Ok(res)
            }
        })
        .default_service(web::route().to(move |http_request: HttpRequest, payload: web::Payload| async move {
            // validate path
            let path = main_namespace.handler_map.remove_path_prefix(http_request.path(), conf.path_prefix.as_ref().map(|s| s.as_str()));
            let method = method_from(http_request.method())?;
            let match_result = if let Some(m_result) = main_namespace.handler_map.r#match(method, path) {
                m_result
            } else if let Some(m_result) = main_namespace.handler_map.default_match(method, path) {
                m_result
            } else {
                Err(Error::not_found_message_only())?
            };

            // High-risk operations for testing
            #[cfg(test)]
            if match_result.path()[0] == "danger" {
                return Ok::<HttpResponse, WrapError>(
                    dangerous_operation(match_result.handler_name())
                        .await?
                        .into_http_response(http_request.clone()),
                );
            }

            // Normal handling
            let mut group = false;
            let dest_namespace = if let Some(d) = main_namespace.namespace_at_path(&match_result.path()) {
                d
            } else if match_result.path().len() > 0 {
                if let Some(d) = main_namespace.namespace_at_path(&match_result.path_without_last()) {
                    group = true;
                    d
                } else {
                    Err(Error::not_found_message_only())?
                }
            } else {
                Err(Error::not_found_message_only())?
            };
            let handler_resolved = if group {
                if let Some(model) = dest_namespace.models.get(match_result.group_name()) {
                    if let Some(group) = dest_namespace.model_handler_groups.get(match_result.group_name()) {
                        if let Some(handler) = group.handlers.get(match_result.handler_name()) {
                            (dest_namespace, HandlerResolved::Custom(handler))
                        } else {
                            if let Some(action) = builtin_action_handler_from_name(match_result.handler_name()) {
                                (dest_namespace, HandlerResolved::Builtin(model, action))
                            } else {
                                Err(Error::not_found_message_only())?
                            }
                        }
                    } else {
                        if let Some(action) = builtin_action_handler_from_name(match_result.handler_name()) {
                            (dest_namespace, HandlerResolved::Builtin(model, action))
                        } else {
                            Err(Error::not_found_message_only())?
                        }
                    }
                } else if let Some(group) = dest_namespace.handler_groups.get(match_result.group_name()) {
                    if let Some(handler) = group.handlers.get(match_result.handler_name()) {
                        (dest_namespace, HandlerResolved::Custom(handler))
                    } else {
                        Err(Error::not_found_message_only())?
                    }
                } else {
                    Err(Error::not_found_message_only())?
                }
            } else {
                if let Some(handler) = dest_namespace.handlers.get(match_result.handler_name()) {
                    (dest_namespace, HandlerResolved::Custom(handler))
                } else {
                    Err(Error::not_found_message_only())?
                }
            };
            let dest_namespace = handler_resolved.0;
            let handler_resolved = handler_resolved.1;
            if method == Method::Options {
                // special handle for options
                let conn_ctx = connection::Ctx::from_namespace(main_namespace);
                let transaction_ctx = transaction::Ctx::new(conn_ctx);
                let ctx = request::Ctx::new(
                    request::Request::new(Arc::new(RequestImpl::new(http_request.clone()))),
                    Arc::new(Value::Null),
                    transaction_ctx,
                    match_result
                );
                return Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async {
                    Ok(Response::empty())
                }).await?.into_http_response(http_request.clone()));
            }
            http_request.extensions_mut().insert(match_result.clone());
            // parse body
            let mut format = HandlerInputFormat::Json;
            match handler_resolved {
                HandlerResolved::Custom(handler) => {
                    format = handler.format;
                }
                _ => (),
            }
            let json_body = match format {
                HandlerInputFormat::Json => if method == Method::Get || method == Method::Delete {
                    JsonValue::Null
                } else {
                    parse_json_body(payload).await?
                },
                HandlerInputFormat::Form => parse_form_body(http_request.clone(), payload).await?,
            };
            return match handler_resolved {
                HandlerResolved::Builtin(model, action) => {
                    let body = validate_and_transform_json_input_for_builtin_action(model, action, &json_body, main_namespace)?;
                    let conn_ctx = connection::Ctx::from_namespace(main_namespace);
                    let transaction_ctx = transaction::Ctx::new(conn_ctx);
                    let ctx = request::Ctx::new(
                        request::Request::new(Arc::new(RequestImpl::new(http_request.clone()))),
                        Arc::new(body),
                        transaction_ctx,
                        match_result.clone(),
                    );
                    match match_result.handler_name() {
                        "findMany" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            find_many(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "findFirst" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            find_first(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "findUnique" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            find_unique(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "create" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            create(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "delete" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            delete(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "update" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            update(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "upsert" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            upsert(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "copy" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            copy(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "createMany" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            create_many(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "updateMany" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            update_many(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "copyMany" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            copy_many(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "deleteMany" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            delete_many(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "count" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            count(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "aggregate" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            aggregate(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "groupBy" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async move {
                            group_by(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        _ => Err(Error::not_found_message_only())?,
                    }
                },
                HandlerResolved::Custom(handler) => {
                    let body = validate_and_transform_json_input_for_handler(handler, &json_body, main_namespace)?;
                    let conn_ctx = connection::Ctx::from_namespace(main_namespace);
                    let transaction_ctx = transaction::Ctx::new(conn_ctx);
                    let ctx = request::Ctx::new(
                        request::Request::new(Arc::new(RequestImpl::new(http_request.clone()))),
                        Arc::new(body),
                        transaction_ctx,
                        match_result
                    );
                    Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack.call(ctx, handler.call).await?.into_http_response(http_request.clone()))
                }
            }
        }));
    app
}

pub(crate) async fn serve(
    namespace: &'static Namespace,
    conf: &'static Server,
    runtime_version: &'static RuntimeVersion,
    entrance: &'static Entrance,
    silent: bool,
) -> Result<()> {
    let bind = conf.bind.clone();
    let port = bind.1;
    let server = HttpServer::new(move || {
        make_server_app(namespace, conf)
    })
        .bind((bind.0, bind.1 as u16))
        .unwrap()
        .run();
    let result = future::join(server, server_start_message(port as u16, runtime_version, entrance, silent)).await;
    result.1
}

async fn server_start_message(port: u16, runtime_version: &'static RuntimeVersion, entrance: &'static Entrance, silent: bool) -> Result<()> {
    if silent { return Ok(()) }
    // Introducing
    let teo_version = env!("CARGO_PKG_VERSION");
    let teo = format!("Teo {}", teo_version);
    info_message(format!("{} ({}, {})", teo, runtime_version.to_string(), entrance.to_str()));
    // Listening
    let port_str = format!("{port}").bold();
    info_message(format!("listening on port {}", port_str));
    Ok(())
}

fn method_from(m: &HttpMethod) -> Result<Method> {
    Ok(match m.as_str() {
        "GET" => Method::Get,
        "POST" => Method::Post,
        "PATCH" => Method::Patch,
        "PUT" => Method::Put,
        "DELETE" => Method::Delete,
        "OPTIONS" => Method::Options,
        _ => Err(Error::new(format!("unknown http method {}", m.as_str())))?
    })
}

async fn dangerous_operation(action :&str)-> Result<Response>{
        let dangerous_operation = DangerousOperations::try_from(action)?;
        match dangerous_operation {
            DangerousOperations::Seed | DangerousOperations::Unseed | DangerousOperations::Reseed => {
                let data_sets = load_data_sets(Ctx::main_namespace(), None, false, Ctx::schema())?;
                let transaction_ctx = transaction::Ctx::new(Ctx::conn_ctx().clone());
                seed(
                    seed_from_dangerous_operation(dangerous_operation)?,
                    data_sets,
                    transaction_ctx,
                    false,
                )
                .await?
            }
            DangerousOperations::PurgeAndSeed => {
                purge::purge().await?;
                connect_databases(Ctx::main_namespace_mut(),true).await?;
                let data_sets = load_data_sets(Ctx::main_namespace(), None, false, Ctx::schema())?;
                let transaction_ctx = transaction::Ctx::new(Ctx::conn_ctx().clone());
                seed(SeedCommandAction::Seed, data_sets, transaction_ctx, false).await?
            }
            DangerousOperations::Purge => purge::purge().await?,
        }
        Ok(Response::data(Value::Bool(true)))
}

enum HandlerResolved<'a> {
    Custom(&'a Handler),
    Builtin(&'a Model, Action),
}

#[derive(Debug)]
enum DangerousOperations {
    Seed,
    Reseed,
    Unseed,
    Purge,
    PurgeAndSeed,
}

impl TryFrom<&str> for DangerousOperations{
    type Error = Error;
    fn try_from(str:&str)->Result<Self>{
        match str.to_lowercase().as_str() {
            "seed"=>Ok(DangerousOperations::Seed),
            "reseed"=>Ok(DangerousOperations::Reseed),
            "unseed"=>Ok(DangerousOperations::Unseed),
            "purge"=>Ok(DangerousOperations::Purge),
            "purge_seed"=>Ok(DangerousOperations::PurgeAndSeed),
            _=> Err(Error::new(format!("unsupport {{{}}} operation",str)))
        }
    }
}

fn seed_from_dangerous_operation (danger_operation :DangerousOperations)->Result<SeedCommandAction>{
    match danger_operation {
        DangerousOperations::Seed=>Ok(SeedCommandAction::Seed),
        DangerousOperations::Reseed=>Ok(SeedCommandAction::Reseed),
        DangerousOperations::Unseed=>Ok(SeedCommandAction::Unseed),
        _ => Err(Error::new("cant create seedCommandAction from DangerousOperation")),
}
}
