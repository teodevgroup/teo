use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;
use actix_web::HttpResponse;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{http, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use teo_result::Error;
use tokio::net::TcpListener;
use teo_runtime::config::server::Server;
use teo_runtime::namespace::Namespace;
use teo_result::ErrorSerializable;
use teo_runtime::{connection, request, Value};
use teo_runtime::action::Action;
use teo_runtime::connection::transaction;
use teo_runtime::handler::action::builtin_action_handler_from_name;
use teo_runtime::handler::Handler;
use teo_runtime::model::Model;
use crate::prelude::{Entrance, RuntimeVersion};

enum HandlerResolved<'a> {
    Custom(&'a Handler),
    Builtin(&'a Model, Action),
}

async fn serve_with_error(
    req: Request<hyper::body::Incoming>,
    main_namespace: &'static Namespace,
    conf: &'static Server,
    silent: bool,
) -> teo_result::Result<Response<Full<Bytes>>> {
    // validate path
    let path = main_namespace.handler_map.remove_path_prefix(req.uri().path(), conf.path_prefix.as_ref().map(|s| s.as_str()));
    let method = req.method();
    let match_result = if let Some(m_result) = main_namespace.handler_map.r#match(method.clone(), path) {
        m_result
    } else if let Some(m_result) = main_namespace.handler_map.default_match(method.clone(), path) {
        m_result
    } else {
        Err(Error::not_found())?
    };
    // Normal handling
    let mut group = false;
    let dest_namespace = if let Some(dest) = main_namespace.namespace_at_path(&match_result.path()) {
        dest
    } else if match_result.path().len() > 0 {
        if let Some(d) = main_namespace.namespace_at_path(&match_result.path_without_last()) {
            group = true;
            d
        } else {
            Err(Error::not_found())?
        }
    } else {
        Err(Error::not_found())?
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
                        Err(Error::not_found())?
                    }
                }
            } else {
                if let Some(action) = builtin_action_handler_from_name(match_result.handler_name()) {
                    (dest_namespace, HandlerResolved::Builtin(model, action))
                } else {
                    Err(Error::not_found())?
                }
            }
        } else if let Some(group) = dest_namespace.handler_groups.get(match_result.group_name()) {
            if let Some(handler) = group.handlers.get(match_result.handler_name()) {
                (dest_namespace, HandlerResolved::Custom(handler))
            } else {
                Err(Error::not_found())?
            }
        } else {
            Err(Error::not_found())?
        }
    } else {
        if let Some(handler) = dest_namespace.handlers.get(match_result.handler_name()) {
            (dest_namespace, HandlerResolved::Custom(handler))
        } else {
            Err(Error::not_found())?
        }
    };
    let dest_namespace = handler_resolved.0;
    let handler_resolved = handler_resolved.1;
    if method == Method::OPTIONS {
        // special handle for options
        let conn_ctx = connection::Ctx::from_namespace(main_namespace);
        let transaction_ctx = transaction::Ctx::new(conn_ctx);
        let ctx = request::Ctx::new(
            request::Request::new(req, match_result.clone()),
            Arc::new(Value::Null),
            transaction_ctx,
            match_result
        );
        return Ok::<HttpResponse, crate::server::error::WrapError>(dest_namespace.middleware_stack.call(ctx, &|ctx: request::Ctx| async {
            Ok(teo_runtime::response::Response::empty())
        }).await?.into_http_response(http_request.clone()));
    }
    http_request.extensions_mut().insert(match_result.clone());
    Ok::<Response<Full<Bytes>>, Error>(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

pub(crate) async fn hyper_serve(
    main_namespace: &'static Namespace,
    conf: &'static Server,
    runtime_version: &'static RuntimeVersion,
    entrance: &'static Entrance,
    silent: bool,
) -> teo_result::Result<()> {
    let ip_addr = IpAddr::from_str(&conf.bind.0).unwrap();
    let addr = SocketAddr::new(ip_addr, conf.bind.1 as u16);
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            let fut = http1::Builder::new()
                .serve_connection(io, service_fn(|req: Request<hyper::body::Incoming>| async move {
                    let result = serve_with_error(req, main_namespace, conf, silent).await;
                    match result {
                        Ok(result) => Ok::<Response<Full<Bytes>>, Infallible>(result),
                        Err(error) => {
                            let serializable = ErrorSerializable::from_error(&error);
                            let result_string = serde_json::to_string(&serializable).unwrap();
                            let mut res = Response::new(Full::new(Bytes::from(result_string)));
                            res.headers_mut().insert("content-type", "application/json".parse().unwrap());
                            *res.status_mut() = StatusCode::try_from(serializable.code).unwrap();
                            Ok(res)
                        }
                    }
                }));
            // if let Err(err) = future::join(fut, server_start_message(conf.bind.1 as u16, runtime_version, entrance, silent)).await {
            //     eprintln!("Error serving connection: {:?}", err);
            // }
            if let Err(err) = fut.await {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
