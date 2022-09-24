use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{AnyPool, Database};
use sqlx::pool::Pool;
use serde_json::{Value as JsonValue};
use crate::connectors::sql::query_builder::SQLDialect;
use crate::core::model::Model;
use sqlx::postgres::PgPoolOptions;
use url::Url;
use crate::connectors::sql::migration::migrate::migrate;
use crate::core::connector::Connector;
use crate::core::error::ActionError;
use crate::core::save_session::SaveSession;
use crate::prelude::{Graph, Object};

#[derive(Debug)]
pub(crate) struct SQLConnector {
    dialect: SQLDialect,
    pool: AnyPool,
}

impl SQLConnector {
    pub(crate) async fn new(dialect: SQLDialect, url: String, models: &Vec<Model>, reset_database: bool) -> Self {
        let url_result = Url::parse(&url);
        if url_result.is_err() {
            panic!("Data source URL is invalid.");
        }
        let mut url = url_result.unwrap();
        let database_name = url.path()[1..].to_string();
        url.set_path("/");
        let mut pool: AnyPool = AnyPool::connect(url.as_str()).await.unwrap();
        Self::setup_database(dialect, &mut pool, database_name, models, reset_database).await;
        Self {
            dialect,
            pool,
        }
    }

    async fn setup_database(dialect: SQLDialect, pool: &mut AnyPool, db_name: String, models: &Vec<Model>, reset_database: bool) {
        migrate(dialect, pool, db_name, models, reset_database).await
    }
}

#[async_trait]
impl Connector for SQLConnector {
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
