use std::sync::Arc;
use async_trait::async_trait;
use sqlx::Database;
use sqlx::pool::Pool;
use serde_json::{Value as JsonValue};
use crate::connectors::sql::query_builder::SQLDialect;
use crate::core::model::Model;
use sqlx::postgres::PgPoolOptions;
use crate::core::connector::Connector;
use crate::core::error::ActionError;
use crate::core::save_session::SaveSession;
use crate::prelude::{Graph, Object};

#[derive(Debug)]
pub(crate) struct SQLConnector<T: Database> {
    pool: Pool<T>,
}

impl<T: Database> SQLConnector<T> {
    pub(crate) async fn new(url: String, models: &Vec<Model>, reset_database: bool) -> Self {
        let pool: Pool<T> = Pool::connect(&url).await.unwrap();
        Self::setup_database(&pool).await;
        Self {
            pool,
        }
    }

    async fn setup_database(pool: &Pool<T>) {

    }
}

#[async_trait]
impl<T: Database> Connector for SQLConnector<T> {
    async fn save_object(&self, object: &Object) -> Result<(), ActionError> {
        todo!()
    }

    async fn delete_object(&self, object: &Object) -> Result<(), ActionError> {
        todo!()
    }

    async fn find_unique(&self, graph: &Graph, model: &Model, finder: &JsonValue, mutation_mode: bool) -> Result<Object, ActionError> {
        todo!()
    }

    async fn find_first(&self, graph: &Graph, model: &Model, finder: &JsonValue, mutation_mode: bool) -> Result<Object, ActionError> {
        todo!()
    }

    async fn find_many(&self, graph: &Graph, model: &Model, finder: &JsonValue, mutation_mode: bool) -> Result<Vec<Object>, ActionError> {
        todo!()
    }

    async fn count(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<usize, ActionError> {
        todo!()
    }

    async fn aggregate(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<JsonValue, ActionError> {
        todo!()
    }

    async fn group_by(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<JsonValue, ActionError> {
        todo!()
    }

    fn new_save_session(&self) -> Arc<dyn SaveSession> {
        todo!()
    }
}
