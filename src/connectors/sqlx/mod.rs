use std::borrow::BorrowMut;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use serde_json::{Value as JsonValue};
use async_trait::async_trait;
use sqlx::{Pool, Database};
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::graph::{Graph};
use crate::core::object::Object;
use crate::core::builders::GraphBuilder;
use crate::core::model::Model;
use crate::error::ActionError;


#[derive(Debug)]
pub struct SqlxConnector<DB> where DB: Database {
    pool: Pool<DB>
}

impl<DB> SqlxConnector<DB> where DB: Database {
    pub(crate) fn new(pool: Pool<DB>, models: &Vec<Model>) -> SqlxConnector<DB> {
        SqlxConnector { pool }
    }
}

#[async_trait]
impl<DB> Connector for SqlxConnector<DB> where DB: Database {

    async fn drop_database(&self) {
        todo!()
    }

    async fn save_object(&self, object: &Object) -> Result<(), ActionError> {
        todo!()
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
pub(crate) struct SqlxConnectorBuilder<DB> where DB: Database {
    pool: Pool<DB>
}

impl<DB> SqlxConnectorBuilder<DB> where DB: Database {
    pub(crate) fn new(pool: Pool<DB>) -> SqlxConnectorBuilder<DB> {
        SqlxConnectorBuilder { pool }
    }
}

#[async_trait]
impl<DB> ConnectorBuilder for SqlxConnectorBuilder<DB> where DB: Database {
    async fn build_connector(&self, models: &Vec<Model>) -> Box<dyn Connector> {
        Box::new(SqlxConnector::new(self.pool.clone(), models))
    }
}

pub trait SqlxConnectorHelpers {
    fn sqlx<DB>(&mut self, pool: Pool<DB>) where DB: Database;
}

impl SqlxConnectorHelpers for GraphBuilder {

    fn sqlx<DB>(&mut self, pool: Pool<DB>) where DB: Database {
        self.connector_builder = Some(Box::new(SqlxConnectorBuilder::new(pool)))
    }
}

unsafe impl<DB> Sync for SqlxConnector<DB> where DB: Database {}
