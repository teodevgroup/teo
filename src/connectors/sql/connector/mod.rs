pub mod builder;
pub mod save_session;

use std::sync::Arc;
use std::sync::atomic::Ordering;
use async_recursion::async_recursion;
use async_trait::async_trait;
use chrono::{Date, DateTime, NaiveDate, Utc};
use key_path::{KeyPath, path};
use sqlx::{AnyPool, Column, Database, Error, Executor, Row, ValueRef};
use sqlx::pool::Pool;
use sqlx::any::{AnyRow, AnyValueRef};
use crate::core::model::Model;
use url::Url;
use crate::connectors::sql::migration::migrate::{migrate, SQLMigration};
use crate::connectors::sql::stmts_builder::integration::select::{build_sql_query_from_json, build_where_from_identifier};
use crate::connectors::sql::stmts_builder::integration::value_encoder::ToWrapped;
use crate::connectors::sql::stmts::SQL;
use crate::connectors::sql::to_sql_string::ToSQLString;
use crate::connectors::sql::save_session::SQLSaveSession;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::connector::{Connector, SaveSession};
use crate::core::env::Env;
use crate::core::env::intent::Intent;
use crate::core::error::ActionError;
use crate::core::input::Input;
use crate::core::result::ActionResult;
use crate::prelude::{Graph, Object, Value};
use crate::tson;

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
        SQLMigration::create_database_if_needed(dialect, &mut pool, &database_name, reset_database).await;
        let mut pool: AnyPool = AnyPool::connect(url.as_str()).await.unwrap();
        SQLMigration::migrate(dialect, &mut pool, models).await;
        Self { dialect, pool }
    }

    async fn create_object(&self, object: &Object, session: Arc<dyn SaveSession>) -> ActionResult<()> {
        let model = object.model();
        let keys = object.keys_for_save();
        let auto_keys = model.auto_keys();
        let mut values: Vec<(&str, String)> = vec![];
        for key in keys {
            if let Some(field) = model.field(key) {
                let column_name = field.column_name();
                let val = object.get_value(key).unwrap();
                values.push((column_name, val.to_string(self.dialect)));
            } else if let Some(_property) = model.property(key) {
                let val: Value = object.get_property(key).await.unwrap();
                values.push((key, val.to_string(self.dialect)));
            }
        }
        let value_refs: Vec<(&str, &str)> = values.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let stmt = SQL::insert_into(model.table_name()).values(value_refs).to_string(self.dialect);
        let result = self.pool.execute(&*stmt).await.unwrap();
        for key in auto_keys {
            object.set_value(key, Value::I64(result.last_insert_id().unwrap()))?;
        }
        Ok(())
    }

    async fn update_object(&self, object: &Object, session: Arc<dyn SaveSession>) -> ActionResult<()> {
        let model = object.model();
        let keys = object.keys_for_save();
        let mut values: Vec<(&str, String)> = vec![];
        for key in &keys {
            if let Some(field) = model.field(key) {
                let column_name = field.column_name();
                if let Some(updator) = object.get_atomic_updator(key) {
                    let (key, val) = Input::key_value(updator.as_hashmap().unwrap());
                    match key {
                        "increment" => values.push((column_name, format!("{} + {}", column_name, val.to_string(self.dialect)))),
                        "decrement" => values.push((column_name, format!("{} - {}", column_name, val.to_string(self.dialect)))),
                        "multiply" => values.push((column_name, format!("{} * {}", column_name, val.to_string(self.dialect)))),
                        "divide" => values.push((column_name, format!("{} / {}", column_name, val.to_string(self.dialect)))),
                        "push" => values.push((column_name, format!("ARRAY_APPEND({}, {})", column_name, val.to_string(self.dialect)))),
                        _ => panic!("Unhandled key."),
                    }
                } else {
                    let val = object.get_value(key).unwrap();
                    values.push((column_name, val.to_string(self.dialect)));
                }
            } else if let Some(_property) = model.property(key) {
                let val: Value = object.get_property(key).await.unwrap();
                values.push((key, val.to_string(self.dialect)));
            }
        }
        let value_refs: Vec<(&str, &str)> = values.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let r#where = build_where_from_identifier(model, object.graph(), &object.identifier(), self.dialect);
        let stmt = SQL::update(model.table_name()).values(value_refs).r#where(&r#where).to_string(self.dialect);
        let result = self.pool.execute(stmt.as_str()).await;
        if result.is_err() {
            println!("{:?}", result.err().unwrap());
            return Err(ActionError::unknown_database_write_error());
        }
        let select_stmt = SQL::select(Some(&keys), model.table_name()).r#where(r#where).to_string(self.dialect);
        let results = self.pool.fetch_optional(&*select_stmt).await;
        match results {
            Ok(row) => {
                match row {
                    Some(row) => {
                        self.row_to_object(&row, &object, None, None, false)?;
                        Ok(())
                    }
                    None => {
                        Err(ActionError::object_not_found())
                    }
                }
            }
            Err(err) => {
                println!("{:?}", err);
                Err(ActionError::unknown_database_find_unique_error())
            }
        }
    }

    #[async_recursion]
    async fn perform_query<'a>(&self, graph: &Graph, model: &Model, finder: &Value, mutation_mode: bool, key_path: &KeyPath, additional_where: Option<String>, additional_left_join: Option<String>, env: Env) -> Result<Vec<Object>, ActionError> {
        let select = finder.get("select");
        let include = finder.get("include");
        let sql_query = build_sql_query_from_json(model, graph, QueryPipelineType::Many, mutation_mode, finder, self.dialect, additional_where, additional_left_join.clone(), key_path)?;
        println!("see sql query: {}", sql_query);
        let reverse = Input::has_negative_take(finder);
        let results = self.pool.fetch_all(&*sql_query).await;
        let mut retval: Vec<Object> = vec![];
        match results {
            Ok(rows) => {
                for row in &rows {
                    let obj = graph.new_object(model.name(), env.clone())?;
                    self.row_to_object(&row, &obj, select, include, additional_left_join.is_some())?;
                    if reverse {
                        retval.insert(0, obj);
                    } else {
                        retval.push(obj);
                    }
                }
                if let Some(include) = include {
                    if let Some(include_map) = include.as_hashmap() {

                    }
                }
            }

        }
        Ok(retval)
    }
}

#[async_trait]
impl Connector for SQLConnector {

    async fn save_object(&self, object: &Object, session: Arc<dyn SaveSession>) -> ActionResult<()> {
        let is_new = object.inner.is_new.load(Ordering::SeqCst);
        if is_new {
            self.create_object(object, session.clone()).await
        } else {
            self.update_object(object, session.clone()).await
        }
    }

    async fn delete_object(&self, object: &Object, session: Arc<dyn SaveSession>) -> ActionResult<()> {
        if object.inner.is_new.load(Ordering::SeqCst) {
            return Err(ActionError::object_is_not_saved());
        }
        let model = object.model();
        let r#where = build_where_from_identifier(model, object.graph(), &object.identifier(), self.dialect);
        let stmt = SQL::delete_from(model.table_name()).r#where(r#where).to_string(self.dialect);
        let result = self.pool.execute(stmt.as_str()).await;
        if result.is_err() {
            println!("{:?}", result.err().unwrap());
            return Err(ActionError::unknown_database_write_error());
        } else {
            Ok(())
        }
    }

    async fn find_unique(&self, graph: &Graph, model: &Model, finder: &Value, mutation_mode: bool, env: Env) -> Result<Object, ActionError> {
        let objects = self.perform_query(graph, model, finder, mutation_mode, &path![], None, None, env).await?;
        if objects.is_empty() {
            Err(ActionError::object_not_found())
        } else {
            Ok(objects.get(0).unwrap().clone())
        }
    }

    async fn find_many(&self, graph: &Graph, model: &Model, finder: &Value, mutation_mode: bool, env: Env) -> Result<Vec<Object>, ActionError> {
        self.perform_query(graph, model, finder, mutation_mode, &path![], None, None, env).await
    }

    async fn count(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<usize, ActionError> {
        let sql_query = build_sql_query_from_json(model, graph, QueryPipelineType::Count, false, finder, self.dialect, None, None, &path![])?;
        let result = self.pool.fetch_one(&*sql_query).await;
        match result {
            Ok(row) => {
                let result: i64 = row.get(0);
                Ok(result as usize)
            }
            Err(err) => {
                println!("{:?}", err);
                Err(ActionError::unknown_database_find_error())
            }
        }
    }

    async fn aggregate(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<Value, ActionError> {
        todo!()
    }

    async fn group_by(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<Value, ActionError> {
        todo!()
    }

    fn new_save_session(&self) -> Arc<dyn SaveSession> {
        Arc::new(SQLSaveSession { })
    }
}
