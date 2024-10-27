use std::future::Future;
use std::net::SocketAddr;
use actix_web::web::Bytes;
use http_body_util::Full;
use hyper::{Request, Response};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use crate::app::App;
use crate::prelude::Result;
use crate::server::make::server_start_message;
use crate::prelude::Error;

pub struct Server {
    app: App,
}

impl Server {

    pub fn new(app: App) -> Self {
        Self { app }
    }

    pub async fn before_serve(&self) -> Result<()> {
        Ok(())
    }

    pub async fn serve(&self, silent: bool) -> Result<()> {
        let bind = &self.app.compiled_main_namespace().server().unwrap().bind;
        let addr = SocketAddr::from(bind);
        let listener = TcpListener::bind(addr).await?;
        server_start_message(bind.1, &self.app.runtime_version(), &self.app.entrance(), silent)?;
        // We start a loop to continuously accept incoming connections
        loop {
            let (stream, _) = listener.accept().await?;

            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io = TokioIo::new(stream);

            // Spawn a tokio task to serve multiple connections concurrently
            tokio::task::spawn(async move {
                // Finally, we bind the incoming connection to our `hello` service
                if let Err(err) = http1::Builder::new()
                    // `service_fn` converts our function in a `Service`
                    .serve_connection(io, self)
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }

    async fn handler(&self, hyper_request: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>> {
        Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
    }
}

impl Service<Request<Incoming>> for Server {
    type Response = Full<Bytes>;
    type Error = Error;
    type Future = ();

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        todo!()
    }
}