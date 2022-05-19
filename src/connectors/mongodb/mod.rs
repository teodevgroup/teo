use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use serde_json::{Value as JsonValue};
use async_trait::async_trait;
use mongodb::{options::ClientOptions, Client, Database, Collection};
use crate::core::connector::Connector;
use crate::core::graph::{Graph, GraphInner};
use crate::core::object::Object;
use crate::core::builders::GraphBuilder;
use crate::core::model::Model;


#[derive(Debug)]
pub struct MongoDBConnector {
    options: ClientOptions,
    connected: AtomicBool,
    client: RefCell<Option<Client>>,
    database: RefCell<Option<Database>>
}

impl MongoDBConnector {
    pub fn new(options: ClientOptions) -> Arc<MongoDBConnector> {
        Arc::new(MongoDBConnector {
            options,
            connected: AtomicBool::new(false),
            client: RefCell::new(None),
            database: RefCell::new(None),
        })
    }
}

#[async_trait]
impl Connector for MongoDBConnector {
    async fn connect(self: Arc<MongoDBConnector>) {
        let client = Client::with_options(self.options.clone()).unwrap();
        let database = client.database(&self.options.default_database.clone().unwrap());
        *self.client.borrow_mut() = Some(client);
        *self.database.borrow_mut() = Some(database);
        self.connected.store(true, Ordering::SeqCst);
    }

    async fn disconnect(self: Arc<MongoDBConnector>) { }

    async fn sync_graph(self: Arc<MongoDBConnector>, graph: Arc<GraphInner>) {
        todo!()
    }

    async fn save_object(self: Arc<MongoDBConnector>, object: Object) {
        todo!()
    }

    async fn delete_object(self: Arc<MongoDBConnector>, object: Object) {
        todo!()
    }

    async fn find_unique(self: Arc<MongoDBConnector>, model: &Model, finder: JsonValue) -> Object {
        todo!()
    }

    async fn find_one(self: Arc<MongoDBConnector>, model: &Model, finder: JsonValue) -> Object {
        todo!()
    }

    async fn find_many(self: Arc<MongoDBConnector>, model: &Model, finder: JsonValue) -> Vec<Object> {
        todo!()
    }
}

pub trait MongoDBConnectorHelpers {
    fn mongodb(&mut self, options: ClientOptions);
}

impl MongoDBConnectorHelpers for GraphBuilder {

    fn mongodb(&mut self, options: ClientOptions) {
        self.connector = Some(MongoDBConnector::new(options))
    }
}

unsafe impl Sync for MongoDBConnector {}
