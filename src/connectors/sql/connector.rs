use std::sync::Arc;
use std::sync::atomic::Ordering;
use async_trait::async_trait;
use sqlx::{AnyPool, Database, Executor};
use sqlx::pool::Pool;
use serde_json::{Value as JsonValue};
use crate::connectors::sql::query_builder::{SQL, SQLDialect, ToSQLString};
use crate::core::model::Model;
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
        let mut url_without_db = url_result.unwrap();
        let database_name = url_without_db.path()[1..].to_string();
        url_without_db.set_path("/");
        let mut pool: AnyPool = AnyPool::connect(url_without_db.as_str()).await.unwrap();
        Self::create_database_if_needed(dialect, &mut pool, &database_name, reset_database).await;
        let mut pool: AnyPool = AnyPool::connect(url.as_str()).await.unwrap();
        Self::setup_database(dialect, &mut pool, models).await;
        Self {
            dialect,
            pool,
        }
    }

    async fn create_database_if_needed(dialect: SQLDialect, pool: &mut AnyPool, db_name: &str, reset_database: bool) {
        // drop database if needed
        if reset_database {
            let stmt = SQL::drop().database(db_name).
                if_exists().to_string(dialect);
            pool.execute(&*stmt).await.unwrap();
        }
        // create and use database
        let stmt = SQL::create().database(db_name).if_not_exists().to_string(dialect);
        pool.execute(&*stmt).await.unwrap();
        let stmt = SQL::r#use().database(db_name).to_string(dialect);
        pool.execute(&*stmt).await.unwrap();
    }

    async fn setup_database(dialect: SQLDialect, pool: &mut AnyPool, models: &Vec<Model>) {
        migrate(dialect, pool, models).await
    }

    fn create_object(&self, object: &Object) -> Result<(), ActionError> {
        let model = object.model();
        let field_names = object.keys_for_save();
        for field_name in field_names {
            let field = model.field(field_name).unwrap();
            let column_name = field.column_name();
            let val = object.get_value(field_name).unwrap();

        }
        Ok(())
    }

    fn update_object(&self, object: &Object) -> Result<(), ActionError> {
        Ok(())
    }
}

#[async_trait]
impl Connector for SQLConnector {
    async fn save_object(&self, object: &Object) -> Result<(), ActionError> {
        let is_new = object.inner.is_new.load(Ordering::SeqCst);
        if is_new {
            self.create_object(object)
        } else {
            self.update_object(object)
        }
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
