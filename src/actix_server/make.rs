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
use teo_runtime::handler::Method;
use teo_runtime::{connection, request};
use teo_runtime::app::entrance::Entrance;
use teo_runtime::app::runtime_version::RuntimeVersion;
use teo_runtime::connection::transaction;
use teo_runtime::handler::default::{create, find_first, find_many, find_unique, update, upsert, copy, create_many, update_many, copy_many, delete_many, count, aggregate, group_by, delete};
use teo_runtime::model::Model;
use teo_runtime::response::Response;
use teo_runtime::Value;
use crate::server::parse::{parse_form_body, parse_json_body};
use teo_runtime::handler::input::{validate_and_transform_json_input_for_handler, validate_and_transform_json_input_for_builtin_action};
use teo_runtime::handler::r#match::HandlerMatch;
use crate::message::{info_message, request_message, unhandled_request_message};
use crate::server::error::WrapError;
use crate::server::responder::IntoHttpResponse;

pub fn make_server_app(
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
        .default_service(web::route().to(move |http_request: HttpRequest, payload: web::Payload| async move {
            // parse body
            let mut format = HandlerInputFormat::Json;
            match handler_resolved {
                HandlerResolved::Custom(handler) => {
                    format = handler.format();
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
                        request::Request::new(http_request.clone()),
                        Arc::new(body),
                        transaction_ctx,
                        match_result.clone(),
                    );
                    match match_result.handler_name() {
                        "findMany" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            find_many(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "findFirst" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            find_first(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "findUnique" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            find_unique(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "create" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            create(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "delete" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            delete(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "update" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            update(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "upsert" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            upsert(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "copy" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            copy(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "createMany" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            create_many(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "updateMany" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            update_many(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "copyMany" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            copy_many(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "deleteMany" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            delete_many(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "count" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            count(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "aggregate" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            aggregate(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        "groupBy" => Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, &|ctx: request::Ctx| async move {
                            group_by(&ctx).await
                        }).await?.into_http_response(http_request.clone())),
                        _ => Err(Error::not_found())?,
                    }
                },
                HandlerResolved::Custom(handler) => {
                    let body = validate_and_transform_json_input_for_handler(handler, &json_body, main_namespace)?;
                    let conn_ctx = connection::Ctx::from_namespace(main_namespace);
                    let transaction_ctx = transaction::Ctx::new(conn_ctx);
                    let ctx = request::Ctx::new(
                        request::Request::new(http_request.clone()),
                        Arc::new(body),
                        transaction_ctx,
                        match_result
                    );
                    Ok::<HttpResponse, WrapError>(dest_namespace.middleware_stack().call(ctx, handler.call()).await?.into_http_response(http_request.clone()))
                }
            }
        }));
    app
}
