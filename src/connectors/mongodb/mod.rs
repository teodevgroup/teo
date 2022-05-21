use std::collections::HashMap;
use std::fmt::{Debug};
use std::sync::Arc;
use std::sync::atomic::{Ordering};
use actix_web::options;
use serde_json::{Value as JsonValue};
use async_trait::async_trait;
use bson::Document;
use mongodb::{options::ClientOptions, Client, Database, Collection};
use mongodb::options::DropDatabaseOptions;
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::graph::{Graph};
use crate::core::object::Object;
use crate::core::builders::GraphBuilder;
use crate::core::model::Model;
use crate::error::ActionError;


#[derive(Debug)]
pub struct MongoDBConnector {
    client: Client,
    database: Database,
    collections: HashMap<&'static str, Collection<Document>>
}

impl MongoDBConnector {
    pub(crate) fn new(options: ClientOptions, models: &Vec<Model>) -> MongoDBConnector {
        let client = Client::with_options(options.clone()).unwrap();
        let database = client.database(&options.default_database.clone().unwrap());
        MongoDBConnector {
            client: client,
            database: database,
            collections: HashMap::new()
        }
    }
}

#[async_trait]
impl Connector for MongoDBConnector {

    async fn drop_database(&self) {
        let options = DropDatabaseOptions::builder().build();
        &self.database.drop(options).await;
    }

    async fn save_object(&self, object: &Object) -> Result<(), ActionError> {
        let is_new = object.inner.is_new.load(Ordering::SeqCst);
        let keys = if is_new {
            object.inner.model.save_keys().clone()
        } else {
            object.inner.model.save_keys().iter().filter(|k| {
                object.inner.modified_fields.borrow().contains(&k.to_string())
            }).map(|k| {*k}).collect()
        };

        Ok(())
    }

    async fn delete_object(&self, object: &Object) {
        todo!()
    }

    async fn find_unique(&self, model: &Model, finder: JsonValue) -> Option<Object> {
        todo!()
    }

    async fn find_one(&self, model: &Model, finder: JsonValue) -> Option<Object> {
        todo!()
    }

    async fn find_many(&self, model: &Model, finder: JsonValue) -> Vec<Object> {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct MongoDBConnectorBuilder {
    options: ClientOptions
}

impl MongoDBConnectorBuilder {
    pub(crate) fn new(options: ClientOptions) -> MongoDBConnectorBuilder {
        MongoDBConnectorBuilder { options }
    }
}

#[async_trait]
impl ConnectorBuilder for MongoDBConnectorBuilder {
    async fn build_connector(&self, models: &Vec<Model>) -> Box<dyn Connector> {
        // for model in graph.models() {
        //     let col: Collection<Document> = self.database.collection(model.table_name());
        //     self.collections.insert(model.name(), col);
        // }
        Box::new(MongoDBConnector::new(self.options.clone(), models))
    }
}

pub trait MongoDBConnectorHelpers {
    fn mongodb(&mut self, options: ClientOptions);
}

impl MongoDBConnectorHelpers for GraphBuilder {

    fn mongodb(&mut self, options: ClientOptions) {
        self.connector_builder = Some(Box::new(MongoDBConnectorBuilder::new(options)))
    }
}

unsafe impl Sync for MongoDBConnector {}
unsafe impl Send for MongoDBConnector {}
