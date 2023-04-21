use std::sync::Arc;
use async_trait::async_trait;
use bson::doc;
use mongodb::{Client, Database};
use mongodb::options::ClientOptions;
use crate::connectors::mongodb::connector::connection::MongoDBConnection;
use crate::core::connector::connection::Connection;
use crate::core::connector::connector::Connector;

#[derive(Debug)]
pub struct MongoDBConnector {
    database: Database,
}

impl MongoDBConnector {
    pub(crate) async fn new(url: String) -> Self {
        let options = match ClientOptions::parse(url).await {
            Ok(options) => options,
            Err(_) => panic!("MongoDB url is invalid.")
        };
        let database_name = match &options.default_database {
            Some(database_name) => database_name,
            None => panic!("No database name found in MongoDB url.")
        };
        let client = match Client::with_options(options.clone()) {
            Ok(client) => client,
            Err(_) => panic!("MongoDB client creating error.")
        };
        match client.database("xxxxxpingpingpingxxxxx").run_command(doc! {"ping": 1}, None).await {
            Ok(_) => (),
            Err(_) => panic!("Cannot connect to MongoDB database."),
        }
        let database = client.database(&database_name);
        Self {
            database,
        }
    }
}

#[async_trait]
impl Connector for MongoDBConnector {
    async fn connection(&self) -> crate::prelude::Result<Arc<dyn Connection>> {
        Ok(Arc::new(MongoDBConnection {
            database: self.database.clone()
        }))
    }
}