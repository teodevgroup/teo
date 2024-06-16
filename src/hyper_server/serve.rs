use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::SystemTime;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{http, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use teo_result::Error;
use tokio::net::TcpListener;
use teo_runtime::config::server::Server;
use teo_runtime::namespace::Namespace;
use teo_result::ErrorSerializable;
use crate::prelude::{Entrance, RuntimeVersion};

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
