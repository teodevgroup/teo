use std::borrow::BorrowMut;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use crate::client::kotlin::KotlinClient;
use crate::client::swift::SwiftClient;
use crate::client::typescript::TypeScriptClient;
use crate::connectors::mongodb::MongoDBConnectorBuilder;
use crate::connectors::mysql::MySQLConnectorBuilder;
use crate::connectors::postgres::PostgresConnectorBuilder;
use crate::core::builders::graph_builder::GraphBuilder;
use crate::core::client::Client;


pub struct TypeScriptClientBuilder<'a> {
    pub(crate) graph_builder: &'a mut GraphBuilder,
    client: Arc<TypeScriptClient>,
}

impl<'a> TypeScriptClientBuilder<'a> {
    pub(crate) fn new(graph_builder: &'a mut GraphBuilder) -> Self {
        let client = Arc::new(TypeScriptClient::new());
        graph_builder.clients.push(client.clone());
        TypeScriptClientBuilder { graph_builder, client }
    }

    pub fn at(&mut self, path: impl Into<String>) -> &mut Self {
        let c = self.client.clone();
        let mut s = c.at.lock().unwrap();
        *s = path.into();
        self
    }

    pub fn react_hooks(&mut self) -> &mut Self {
        self.client.react_hooks.store(true, Ordering::SeqCst);
        self
    }
}

pub struct SwiftClientBuilder<'a> {
    pub(crate) graph_builder: &'a mut GraphBuilder,
    client: Arc<SwiftClient>
}

impl<'a> SwiftClientBuilder<'a> {
    pub(crate) fn new(graph_builder: &'a mut GraphBuilder) -> Self {
        let client = Arc::new(SwiftClient::new());
        graph_builder.clients.push(client.clone());
        SwiftClientBuilder { graph_builder, client }
    }

    pub fn at(&mut self, path: &'static str) -> &mut Self {
        let c = self.client.clone();
        let mut s = c.at.lock().unwrap();
        *s = path.into();
        self
    }

    pub fn combine_observable_objects(&mut self) -> &mut Self {
        self.client.combine_observable_objects.store(true, Ordering::SeqCst);
        self
    }
}

pub struct KotlinClientBuilder<'a> {
    pub(crate) graph_builder: &'a mut GraphBuilder,
    client: Arc<KotlinClient>,
}

impl<'a> KotlinClientBuilder<'a> {
    pub(crate) fn new(graph_builder: &'a mut GraphBuilder) -> Self {
        let client = Arc::new(KotlinClient::new());
        graph_builder.clients.push(client.clone());
        KotlinClientBuilder { graph_builder, client }
    }

    pub fn at(&mut self, path: impl Into<String>) -> &mut Self {
        let c = self.client.clone();
        let mut s = c.at.lock().unwrap();
        *s = path.into();
        self
    }

    pub fn jetpack_compose_states(&mut self) -> &mut Self {
        self.client.jetpack_compose_states.store(true, Ordering::SeqCst);
        self
    }
}

pub struct ClientBuilder<'a> {
    pub(crate) graph_builder: &'a mut GraphBuilder,
}

impl<'a> ClientBuilder<'a> {

    pub(crate) fn new(graph_builder: &'a mut GraphBuilder) -> Self {
        ClientBuilder { graph_builder }
    }

    pub fn typescript(&mut self) -> TypeScriptClientBuilder {
        TypeScriptClientBuilder::new(self.graph_builder)
    }

    pub fn swift(&mut self) -> SwiftClientBuilder {
        SwiftClientBuilder::new(self.graph_builder)
    }

    pub fn kotlin(&mut self) -> KotlinClientBuilder {
        KotlinClientBuilder::new(self.graph_builder)
    }
}
