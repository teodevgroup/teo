pub mod save_session;

use std::sync::Arc;
use std::sync::atomic::Ordering;
use async_trait::async_trait;
use quaint_forked::{prelude::*, pooled::Quaint, ast::Query as QuaintQuery};
use quaint_forked::error::DatabaseConstraint;
use quaint_forked::error::ErrorKind::UniqueConstraintViolation;
use crate::core::model::Model;
use crate::connectors::sql::schema::r#type::field::ToDatabaseType;
use crate::connectors::sql::connector::save_session::SQLSaveSession;
use crate::connectors::sql::execution::Execution;
use crate::connectors::sql::migration::migrate::SQLMigration;
use crate::connectors::sql::query::Query;
use crate::connectors::sql::stmts::SQL;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::value::decode::RowDecoder;
use crate::connectors::sql::schema::value::encode::ToSQLString;
use crate::connectors::sql::schema::value::encode::PSQLArrayToSQLString;
use crate::connectors::sql::url::url_utils;
use crate::core::action::Action;
use crate::core::action::source::ActionSource;
use crate::core::connector::{Connector, SaveSession};
use crate::core::database::r#type::DatabaseType;
use crate::core::error::Error;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::core::input::Input;
use crate::core::result::Result;
use crate::prelude::{Graph, Object, Value};
use crate::teon;

pub(crate) struct SQLConnector {
    dialect: SQLDialect,
    pool: Quaint,
}

impl SQLConnector {

    pub(crate) async fn new(dialect: SQLDialect, url: &str, reset: bool) -> Self {
        SQLMigration::create_database_if_needed(dialect, url, reset).await;
        let url = url_utils::normalized_url(dialect, url);
        let pool = Quaint::builder(url.as_str()).unwrap().build();
        Self { dialect, pool }
    }

    async fn create_object(&self, object: &Object) -> Result<()> {
        let conn = self.pool.check_out().await.unwrap();
        let model = object.model();
        let keys = object.keys_for_save();
        let auto_keys = model.auto_keys();
        let mut values: Vec<(&str, String)> = vec![];
        for key in keys {
            if let Some(field) = model.field(key) {
                let column_name = field.column_name();
                let val = object.get_value(key).unwrap();
                if !(field.auto_increment && val.is_null()) {
                    values.push((column_name, PSQLArrayToSQLString::to_string_with_ft(&val, self.dialect, field.field_type())));
                }
            } else if let Some(property) = model.property(key) {
                let val: Value = object.get_property(key).await.unwrap();
                values.push((key, PSQLArrayToSQLString::to_string_with_ft(&val, self.dialect, property.field_type())));
            }
        }
        let value_refs: Vec<(&str, &str)> = values.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let stmt = SQL::insert_into(model.table_name()).values(value_refs).returning(auto_keys).to_string(self.dialect);
        if self.dialect == SQLDialect::PostgreSQL {
            match conn.query(QuaintQuery::from(stmt)).await {
                Ok(result_set) => {
                    let columns = result_set.columns().clone();
                    let result = result_set.into_iter().next();
                    if result.is_some() {
                        let value = Execution::row_to_value(model, object.graph(), &result.unwrap(), &columns, self.dialect);
                        for (k, v) in value.as_hashmap().unwrap() {
                            object.set_value(k, v.clone())?;
                        }
                    }
                    Ok(())
                }
                Err(err) => {
                    println!("{:?}", err);
                    Err(Self::handle_err_result(self, err))
                }
            }
        } else {
            match conn.query(QuaintQuery::from(stmt)).await {
                Ok(result) => {
                    let id = result.last_insert_id().unwrap();
                    for key in auto_keys {
                        if model.field(key).unwrap().field_type().is_int32() {
                            object.set_value(key, Value::I32(id as i32))?;
                        } else {
                            object.set_value(key, Value::I64(id as i64))?;
                        }
                    }
                    Ok(())
                }
                Err(err) => {
                    println!("{:?}", err);
                    Err(Self::handle_err_result(self,err))
                }
            }
        }
    }

    async fn update_object(&self, object: &Object) -> Result<()> {
        let conn = self.pool.check_out().await.unwrap();
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
                    values.push((column_name, PSQLArrayToSQLString::to_string_with_ft(&val, self.dialect, field.field_type())));
                }
            } else if let Some(property) = model.property(key) {
                let val: Value = object.get_property(key).await.unwrap();
                values.push((key, PSQLArrayToSQLString::to_string_with_ft(&val, self.dialect, property.field_type())));
            }
        }
        let value_refs: Vec<(&str, &str)> = values.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let identifier = object.identifier();
        let r#where = Query::where_from_identifier(object, self.dialect);
        if !value_refs.is_empty() {
            let stmt = SQL::update(model.table_name()).values(value_refs).r#where(&r#where).to_string(self.dialect);
            let result = conn.execute(QuaintQuery::from(stmt)).await;
            if result.is_err() {
                println!("{:?}", result.err().unwrap());
                return Err(Error::unknown_database_write_error());
            }
        }
        let result = Execution::query(&self.pool, model, object.graph(), &teon!({"where": identifier, "take": 1}), self.dialect).await?;
        if result.is_empty() {
            Err(Error::object_not_found())
        } else {
            object.set_from_database_result_value(result.get(0).unwrap(), None, None);
            Ok(())
        }
    }

    fn handle_err_result(&self, err: quaint_forked::error::Error) -> Error {
        match err.kind() {
            UniqueConstraintViolation { constraint } => {
                match constraint {
                    DatabaseConstraint::Fields(fields) => {
                        Error::unique_value_duplicated(fields.get(0).unwrap())
                    }
                    DatabaseConstraint::Index(index) => {
                        Error::unique_value_duplicated(index.clone())
                    }
                    _ => {
                        Error::unknown_database_write_error()
                    }
                }
            }
            _ => {
                Error::unknown_database_write_error()
            }
        }
    }
}

#[async_trait]
impl Connector for SQLConnector {

    fn default_database_type(&self, field_type: &FieldType) -> DatabaseType {
        field_type.to_database_type(self.dialect)
    }

    async fn migrate(&mut self, models: &Vec<Model>, _reset_database: bool) -> Result<()> {
        SQLMigration::migrate(self.dialect, &self.pool, models).await;
        Ok(())
    }

    async fn query_raw(&self, query: &Value) -> Result<Value> {
        let conn = self.pool.check_out().await.unwrap();
        let result = conn.query(QuaintQuery::from(query.as_str().unwrap())).await;
        if result.is_err() {
            let err = result.unwrap_err();
            let msg = err.original_message();
            return Err(Error::internal_server_error(msg.unwrap()));
        } else {
            let result = result.unwrap();
            if result.is_empty() {
                return Ok(Value::Null);
            } else {
                return Ok(RowDecoder::decode_raw_result_set(result));
            }
        }
    }

    async fn save_object(&self, object: &Object, _session: Arc<dyn SaveSession>) -> Result<()> {
        let is_new = object.inner.is_new.load(Ordering::SeqCst);
        if is_new {
            self.create_object(object).await
        } else {
            self.update_object(object).await
        }
    }

    async fn delete_object(&self, object: &Object, _session: Arc<dyn SaveSession>) -> Result<()> {
        let conn = self.pool.check_out().await.unwrap();
        if object.inner.is_new.load(Ordering::SeqCst) {
            return Err(Error::object_is_not_saved_thus_cant_be_deleted());
        }
        let model = object.model();
        let r#where = Query::where_from_identifier(object, self.dialect);
        let stmt = SQL::delete_from(model.table_name()).r#where(r#where).to_string(self.dialect);
        let result = conn.execute(QuaintQuery::from(stmt)).await;
        if result.is_err() {
            println!("{:?}", result.err().unwrap());
            return Err(Error::unknown_database_write_error());
        } else {
            Ok(())
        }
    }

    async fn find_unique(&self, graph: &Graph, model: &Model, finder: &Value, _mutation_mode: bool, action: Action, action_source: ActionSource) -> Result<Object> {
        let objects = Execution::query_objects(&self.pool, model, graph, finder, self.dialect, action, action_source.clone()).await?;
        if objects.is_empty() {
            Err(Error::object_not_found())
        } else {
            Ok(objects.get(0).unwrap().clone())
        }
    }

    async fn find_many(&self, graph: &Graph, model: &Model, finder: &Value, _mutation_mode: bool, action: Action, action_source: ActionSource) -> Result<Vec<Object>> {
        Execution::query_objects(&self.pool, model, graph, finder, self.dialect, action, action_source).await
    }

    async fn count(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<usize> {
        match Execution::query_count(&self.pool, model, graph, finder, self.dialect).await {
            Ok(c) => Ok(c as usize),
            Err(e) => Err(e),
        }
    }

    async fn aggregate(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<Value> {
        Execution::query_aggregate(&self.pool, model, graph, finder, self.dialect).await
    }

    async fn group_by(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<Value> {
        Execution::query_group_by(&self.pool, model, graph, finder, self.dialect).await
    }

    fn new_save_session(&self) -> Arc<dyn SaveSession> {
        Arc::new(SQLSaveSession { })
    }
}
