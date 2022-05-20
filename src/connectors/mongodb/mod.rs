use std::cell::RefCell;
use std::fmt::{Debug};
use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use serde_json::{Value as JsonValue};
use async_trait::async_trait;
use mongodb::{options::ClientOptions, Client, Database, Collection};
use mongodb::change_stream::event::OperationType::DropDatabase;
use mongodb::options::DropDatabaseOptions;
use crate::core::connector::Connector;
use crate::core::graph::{Graph, GraphInner};
use crate::core::object::Object;
use crate::core::builders::GraphBuilder;
use crate::core::model::Model;


#[derive(Debug)]
pub struct MongoDBConnector {
    client: Client,
    database: Database,
}

impl MongoDBConnector {
    pub fn new(options: ClientOptions) -> Arc<MongoDBConnector> {
        let client = Client::with_options(options.clone()).unwrap();
        let database = client.database(&options.default_database.clone().unwrap());

        Arc::new(MongoDBConnector {
            client: client,
            database: database,
        })
    }
}

#[async_trait]
impl Connector for MongoDBConnector {

    async fn drop_database(self: Arc<Self>) {
        let options = DropDatabaseOptions::builder().build();
        &self.database.drop(options).await;
    }

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
unsafe impl Send for MongoDBConnector {}
