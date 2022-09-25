use std::sync::Arc;
use std::sync::atomic::Ordering;
use async_trait::async_trait;
use sqlx::{AnyPool, Database, Executor};
use sqlx::pool::Pool;
use serde_json::{Value as JsonValue};
use crate::core::model::Model;
use url::Url;
use crate::connectors::shared::query_pipeline_type::QueryPipelineType;
use crate::connectors::sql::migration::migrate::migrate;
use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::integration::select::build_sql_query_from_json;
use crate::connectors::sql::query_builder::integration::value_encoder::encode_value_to_sql_input;
use crate::connectors::sql::query_builder::stmt::SQL;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;
use crate::connectors::sql::save_session::SQLSaveSession;
use crate::core::connector::Connector;
use crate::core::error::ActionError;
use crate::core::save_session::SaveSession;
use crate::prelude::{Graph, Object, Value};

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

    async fn create_object(&self, object: &Object) -> Result<(), ActionError> {
        let model = object.model();
        let field_names = object.keys_for_save();
        let mut values: Vec<(&str, String)> = vec![];
        for field_name in field_names {
            let field = model.field(field_name).unwrap();
            let column_name = field.column_name();
            let val = object.get_value(field_name).unwrap();
            values.push((column_name, encode_value_to_sql_input(val)));
        }
        let value_refs: Vec<(&str, &str)> = values.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let stmt = SQL::insert_into(model.table_name()).values(value_refs).to_string(self.dialect);
        let result = self.pool.execute(&*stmt).await.unwrap();
        if let Some(primary_key_name) = model.primary_field_name() {
            object.set_value(primary_key_name, Value::I64(result.last_insert_id().unwrap())).unwrap();
        }
        Ok(())
    }

    async fn update_object(&self, object: &Object) -> Result<(), ActionError> {
        Ok(())
    }
}

#[async_trait]
impl Connector for SQLConnector {
    async fn save_object(&self, object: &Object) -> Result<(), ActionError> {
        let is_new = object.inner.is_new.load(Ordering::SeqCst);
        if is_new {
            self.create_object(object).await
        } else {
            self.update_object(object).await
        }
    }

    async fn delete_object(&self, object: &Object) -> Result<(), ActionError> {
        todo!()
    }

    async fn find_unique(&self, graph: &Graph, model: &Model, finder: &JsonValue, mutation_mode: bool) -> Result<Object, ActionError> {
        let select = finder.get("select");
        let include = finder.get("include");
        let aggregate_input = build_sql_query_from_json(model, graph, QueryPipelineType::Unique, mutation_mode, finder)?;

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
        Arc::new(SQLSaveSession { })
    }
}
