use std::sync::Arc;
use std::sync::atomic::Ordering;
use async_trait::async_trait;
use sqlx::{AnyPool, Column, Database, Executor, Row};
use sqlx::pool::Pool;
use serde_json::{Value as JsonValue};
use sqlx::any::AnyRow;
use crate::core::model::Model;
use url::Url;
use crate::connectors::shared::has_negative_take::has_negative_take;
use crate::connectors::shared::query_pipeline_type::QueryPipelineType;
use crate::connectors::sql::migration::migrate::migrate;
use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::integration::select::{build_sql_query_from_json, build_where_from_identifier};
use crate::connectors::sql::query_builder::stmt::SQL;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;
use crate::connectors::sql::save_session::SQLSaveSession;
use crate::core::connector::Connector;
use crate::core::error::ActionError;
use crate::core::input::AtomicUpdateType;
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

    fn row_to_object(&self, row: &AnyRow, object: &Object, select: Option<&JsonValue>, include: Option<&JsonValue>) -> Result<(), ActionError> {
        for column in row.columns() {
            let column_name = column.name();
            if let Some(field) = object.model().field_with_column_name(column_name) {
                if field.r#type().is_bool() {
                    let bool_value: bool = row.get(column_name);
                    object.inner.value_map.lock().unwrap().insert(field.name().to_owned(), Value::Bool(bool_value));
                } else if field.r#type().is_int() {
                    let i64_value: i64 = row.get(column_name);
                    object.inner.value_map.lock().unwrap().insert(field.name().to_owned(), Value::number_from_i64(i64_value, field.r#type()));
                } else if field.r#type().is_float() {
                    let f64_value: f64 = row.get(column_name);
                    object.inner.value_map.lock().unwrap().insert(field.name().to_owned(), Value::number_from_f64(f64_value, field.r#type()));
                } else if field.r#type().is_string() {
                    let string_value: String = row.get(column_name);
                    object.inner.value_map.lock().unwrap().insert(field.name().to_owned(), Value::String(string_value));
                }
            } else if let Some(relation) = object.model().relation(column_name) {}
        }
        object.inner.is_initialized.store(true, Ordering::SeqCst);
        object.inner.is_new.store(false, Ordering::SeqCst);
        if let Some(select) = select {
            object.set_select(Some(select)).unwrap();
        }
        Ok(())

        // // relation
                // let relation = object.model().relation(key);
                // if relation.is_none() {
                //     continue;
                // }
                // let inner_finder = if let Some(include) = include {
                //     include.get(key)
                // } else {
                //     None
                // };
                // let inner_select = if let Some(inner_finder) = inner_finder {
                //     inner_finder.get("select")
                // } else {
                //     None
                // };
                // let inner_include = if let Some(inner_finder) = inner_finder {
                //     inner_finder.get("include")
                // } else {
                //     None
                // };
                // let relation = relation.unwrap();
                // let model_name = &relation.model;
                // let object_bsons = document.get(key).unwrap().as_array().unwrap();
                // let mut related: Vec<Object> = vec![];
                // for related_object_bson in object_bsons {
                //     let related_object = object.graph().new_object(model_name)?;
                //     self.document_to_object(related_object_bson.as_document().unwrap(), &related_object, inner_select, inner_include)?;
                //     related.push(related_object);
                // }
                // object.inner.relation_query_map.lock().unwrap().insert(key.to_string(), related);
    }

    async fn create_object(&self, object: &Object) -> Result<(), ActionError> {
        let model = object.model();
        let field_names = object.keys_for_save();
        let mut values: Vec<(&str, String)> = vec![];
        for field_name in field_names {
            let field = model.field(field_name).unwrap();
            let column_name = field.column_name();
            let val = object.get_value(field_name).unwrap();
            values.push((column_name, val.to_string(self.dialect)));
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
        let model = object.model();
        let field_names = object.keys_for_save();
        let mut values: Vec<(&str, String)> = vec![];
        for field_name in &field_names {
            let column_name = model.field(field_name).unwrap().column_name();
            let updator_map = object.inner.atomic_updator_map.lock().unwrap();
            if updator_map.contains_key(*field_name) {
                let updator = updator_map.get(*field_name).unwrap();
                match updator {
                    AtomicUpdateType::Increment(val) => {
                        values.push((column_name, format!("{} + {}", column_name, val.to_string(self.dialect))));
                    }
                    AtomicUpdateType::Decrement(val) => {
                        values.push((column_name, format!("{} - {}", column_name, val.to_string(self.dialect))));
                    }
                    AtomicUpdateType::Multiply(val) => {
                        values.push((column_name, format!("{} * {}", column_name, val.to_string(self.dialect))));
                    }
                    AtomicUpdateType::Divide(val) => {
                        values.push((column_name, format!("{} / {}", column_name, val.to_string(self.dialect))));
                    }
                    AtomicUpdateType::Push(val) => {
                        values.push((column_name, format!("ARRAY_APPEND({}, {})", column_name, val.to_string(self.dialect))));
                    }
                }
            } else {
                let val = object.get_value(field_name).unwrap();
                values.push((column_name, val.to_string(self.dialect)));
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
        let select_stmt = SQL::select(Some(&field_names), model.table_name()).r#where(r#where).to_string(self.dialect);
        let results = self.pool.fetch_optional(&*select_stmt).await;
        match results {
            Ok(row) => {
                match row {
                    Some(row) => {
                        self.row_to_object(&row, &object, None, None)?;
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

    async fn find_unique(&self, graph: &Graph, model: &Model, finder: &JsonValue, mutation_mode: bool) -> Result<Object, ActionError> {
        let select = finder.get("select");
        let include = finder.get("include");
        let sql_query = build_sql_query_from_json(model, graph, QueryPipelineType::Unique, mutation_mode, finder, self.dialect)?;
        let result = self.pool.fetch_optional(&*sql_query).await;
        match result {
            Ok(row) => {
                match row {
                    Some(row) => {
                        let obj = graph.new_object(model.name())?;
                        self.row_to_object(&row, &obj, select, include)?;
                        Ok(obj)
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

    async fn find_many(&self, graph: &Graph, model: &Model, finder: &JsonValue, mutation_mode: bool) -> Result<Vec<Object>, ActionError> {
        let select = finder.get("select");
        let include = finder.get("include");
        let sql_query = build_sql_query_from_json(model, graph, QueryPipelineType::Many, mutation_mode, finder, self.dialect)?;
        let reverse = has_negative_take(finder);
        let results = self.pool.fetch_all(&*sql_query).await;
        let mut retval: Vec<Object> = vec![];
        match results {
            Ok(rows) => {
                for row in &rows {
                    let obj = graph.new_object(model.name())?;
                    self.row_to_object(&row, &obj, select, include)?;
                    if reverse {
                        retval.insert(0, obj);
                    } else {
                        retval.push(obj);
                    }
                }
            }
            Err(err) => {
                println!("{:?}", err);
                return Err(ActionError::unknown_database_find_error());
            }
        }
        Ok(retval)
    }

    async fn count(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<usize, ActionError> {
        let sql_query = build_sql_query_from_json(model, graph, QueryPipelineType::Count, false, finder, self.dialect)?;
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
