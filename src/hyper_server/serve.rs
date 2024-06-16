use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use colored::Colorize;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use teo_runtime::config::server::Server;
use teo_runtime::namespace::Namespace;
use crate::message::info_message;
use crate::prelude::{Entrance, RuntimeVersion};

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

pub(crate) async fn hyper_serve(
    namespace: &'static Namespace,
    conf: &'static Server,
    runtime_version: &'static RuntimeVersion,
    entrance: &'static Entrance,
    silent: bool,
) -> teo_result::Result<()> {
    let ip_addr = IpAddr::from_str(&conf.bind.0).unwrap();
    let addr = SocketAddr::new(ip_addr, conf.bind.1 as u16);
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            let fut = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(hello));
            // if let Err(err) = future::join(fut, server_start_message(conf.bind.1 as u16, runtime_version, entrance, silent)).await {
            //     eprintln!("Error serving connection: {:?}", err);
            // }
            if let Err(err) = fut.await {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
