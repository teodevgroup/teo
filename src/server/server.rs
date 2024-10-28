use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper_util::rt::TokioIo;
use teo_runtime::connection;
use teo_runtime::connection::transaction;
use teo_runtime::request::Request;
use teo_runtime::response::convert::hyper_response_from;
use teo_runtime::response::Response;
use tokio::net::TcpListener;
use crate::app::App;
use crate::server::message::server_start_message;
use crate::prelude::Result;
use crate::prelude::Error;
use crate::server::droppable_next::DroppableNext;

pub struct Server {
    app: App,
}

impl Server {

    pub fn new(app: &App) -> Self {
        Self { app: app.clone() }
    }

    pub async fn before_serve(&self) -> Result<()> {
        Ok(())
    }

    pub async fn serve(&'static self, silent: bool) -> Result<()> {
        let bind = &self.app.compiled_main_namespace().server().unwrap().bind;
        let addr: SocketAddr = match format!("{}:{}", bind.0, bind.1).parse() {
            Ok(addr) => addr,
            Err(_) => return Err(Error::new(format!("cannot parse server bind address: {}:{}", bind.0, bind.1))),
        };
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

    async fn hyper_handler(&self, original_hyper_request: hyper::Request<Incoming>) -> Result<hyper::Response<Full<Bytes>>> {
        let main_namespace = self.app.compiled_main_namespace();
        let conf = self.app.compiled_main_namespace().server().unwrap();
        let conn_ctx = connection::Ctx::from_namespace(main_namespace);
        let transaction_ctx = transaction::Ctx::new(conn_ctx);
        let (parts, incoming) = original_hyper_request.into_parts();
        let hyper_request = hyper::Request::from_parts(parts, ());
        let request = Request::new(hyper_request, transaction_ctx);
        let droppable_next = DroppableNext::new(|request: Request| async move {
            return Ok::<Response, Error>(Response::string("Hello, world!", "text/plain"))
        });
        let response = main_namespace.request_middleware_stack().call(request.clone(), droppable_next.get_next()).await?;
        hyper_response_from(request, response)
    }
}

impl Service<hyper::Request<Incoming>> for Server {
    type Response = hyper::Response<Full<Bytes>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = core::result::Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: hyper::Request<Incoming>) -> Self::Future {
        let self_ = unsafe { &*(self as *const Server) } as &'static Server;
        Box::pin(self_.hyper_handler(req))
    }
}