use std::sync::Arc;
use std::sync::atomic::Ordering;
use async_trait::async_trait;
use quaint_forked::{prelude::*, ast::Query as QuaintQuery};
use quaint_forked::error::DatabaseConstraint;
use quaint_forked::error::ErrorKind::UniqueConstraintViolation;
use quaint_forked::pooled::PooledConnection;
use quaint_forked::connector::owned_transaction::OwnedTransaction;
use quaint_forked::connector::start_owned_transaction;
use crate::app::app_ctx::AppCtx;
use crate::core::model::model::Model;
use crate::connectors::sql::execution::Execution;
use crate::connectors::sql::migration::migrate::SQLMigration;
use crate::connectors::sql::query::Query;
use crate::connectors::sql::stmts::SQL;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::value::decode::RowDecoder;
use crate::connectors::sql::schema::value::encode::ToSQLString;
use crate::connectors::sql::schema::value::encode::PSQLArrayToSQLString;
use crate::core::action::Action;
use crate::core::connector::connection::Connection;
use crate::core::initiator::Initiator;
use crate::core::error::Error;
use crate::core::field::r#type::FieldTypeOwner;
use crate::core::input::Input;
use crate::core::result::Result;
use crate::prelude::{Object, Value};
use teo_teon::teon;

#[derive(Clone)]
pub(crate) struct SQLConnection {
    pub(super) dialect: SQLDialect,
    pub(super) conn: Arc<PooledConnection>,
    pub(super) tran: Option<Arc<OwnedTransaction>>,
}

impl SQLConnection {
    pub(super) fn new(dialect: SQLDialect, conn: Arc<PooledConnection>, tran: Option<Arc<OwnedTransaction>>) -> Self {
        Self {
            dialect, conn, tran
        }
    }
}

impl SQLConnection {

    fn dialect(&self) -> SQLDialect {
        self.dialect
    }

    fn conn(&self) -> &PooledConnection {
        self.conn.as_ref()
    }

    fn tran(&self) -> Option<&Arc<OwnedTransaction>> {
        self.tran.as_ref()
    }

    async fn create_object(&self, object: &Object) -> Result<()> {
        let model = object.model();
        let keys = object.keys_for_save();
        let auto_keys = model.auto_keys();
        let mut values: Vec<(&str, String)> = vec![];
        for key in keys {
            if let Some(field) = model.field(key) {
                let column_name = field.column_name();
                let val = object.get_value(key).unwrap();
                if !(field.auto_increment && val.is_null()) {
                    values.push((column_name, PSQLArrayToSQLString::to_string_with_ft(&val, self.dialect(), field.field_type())));
                }
            } else if let Some(property) = model.property(key) {
                let val: Value = object.get_property(key).await.unwrap();
                values.push((key, PSQLArrayToSQLString::to_string_with_ft(&val, self.dialect(), property.field_type())));
            }
        }
        let value_refs: Vec<(&str, &str)> = values.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let stmt = SQL::insert_into(model.table_name()).values(value_refs).returning(auto_keys).to_string(self.dialect());
        // println!("create stmt: {}", stmt);
        if self.dialect() == SQLDialect::PostgreSQL {
            match self.conn().query(QuaintQuery::from(stmt)).await {
                Ok(result_set) => {
                    let columns = result_set.columns().clone();
                    let result = result_set.into_iter().next();
                    if result.is_some() {
                        let value = Execution::row_to_value(model, &result.unwrap(), &columns, self.dialect());
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
            match self.conn().query(QuaintQuery::from(stmt)).await {
                Ok(result) => {
                    if let Some(id) = result.last_insert_id() {
                        for key in auto_keys {
                            if model.field(key).unwrap().field_type().is_int32() {
                                object.set_value(key, Value::I32(id as i32))?;
                            } else {
                                object.set_value(key, Value::I64(id as i64))?;
                            }
                        }
                    }
                    Ok(())
                }
                Err(err) => {
                    println!("create object error: {:?}", err);
                    Err(Self::handle_err_result(self,err))
                }
            }
        }
    }

    async fn update_object(&self, object: &Object) -> Result<()> {
        let model = object.model();
        let keys = object.keys_for_save();
        let mut values: Vec<(&str, String)> = vec![];
        for key in &keys {
            if let Some(field) = model.field(key) {
                let column_name = field.column_name();
                if let Some(updator) = object.get_atomic_updator(key) {
                    let (key, val) = Input::key_value(updator.as_hashmap().unwrap());
                    match key {
                        "increment" => values.push((column_name, format!("{} + {}", column_name, ToSQLString::to_string(&val, self.dialect())))),
                        "decrement" => values.push((column_name, format!("{} - {}", column_name, ToSQLString::to_string(&val, self.dialect())))),
                        "multiply" => values.push((column_name, format!("{} * {}", column_name, ToSQLString::to_string(&val, self.dialect())))),
                        "divide" => values.push((column_name, format!("{} / {}", column_name, ToSQLString::to_string(&val, self.dialect())))),
                        "push" => values.push((column_name, format!("ARRAY_APPEND({}, {})", column_name, ToSQLString::to_string(&val, self.dialect())))),
                        _ => unreachable!(),
                    }
                } else {
                    let val = object.get_value(key).unwrap();
                    values.push((column_name, PSQLArrayToSQLString::to_string_with_ft(&val, self.dialect(), field.field_type())));
                }
            } else if let Some(property) = model.property(key) {
                let val: Value = object.get_property(key).await.unwrap();
                values.push((key, PSQLArrayToSQLString::to_string_with_ft(&val, self.dialect(), property.field_type())));
            }
        }
        let value_refs: Vec<(&str, &str)> = values.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let identifier = object.identifier();
        let r#where = Query::where_from_previous_identifier(object, self.dialect());
        if !value_refs.is_empty() {
            let stmt = SQL::update(model.table_name()).values(value_refs).r#where(&r#where).to_string(self.dialect());
            // println!("update stmt: {}", stmt);
            let result = self.conn().execute(QuaintQuery::from(stmt)).await;
            if result.is_err() {
                println!("{:?}", result.err().unwrap());
                return Err(Error::unknown_database_write_error());
            }
        }
        let result = Execution::query(self.conn(), model, &teon!({"where": identifier, "take": 1}), self.dialect()).await?;
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
                        Error::unique_value_duplicated(fields.get(0).unwrap().to_string())
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
impl Connection for SQLConnection {

    async fn migrate(self: Arc<Self>, models: Vec<&Model>, _reset_database: bool) -> Result<()> {
        SQLMigration::migrate(self.dialect(), self.conn(), models, self.clone()).await;
        Ok(())
    }

    async fn purge(&self) -> Result<()> {
        for model in AppCtx::get().unwrap().models() {
            let escape = self.dialect().escape();
            self.conn().execute(QuaintQuery::from(format!("DELETE FROM {escape}{}{escape}", model.table_name()))).await.unwrap();
        }
        Ok(())
    }

    async fn query_raw(&self, query: &Value) -> Result<Value> {
        let result = self.conn().query(QuaintQuery::from(query.as_str().unwrap())).await;
        if result.is_err() {
            let err = result.unwrap_err();
            let msg = err.original_message();
            return Err(Error::internal_server_error(msg.unwrap()));
        } else {
            let result = result.unwrap();
            return if result.is_empty() {
                Ok(Value::Null)
            } else {
                Ok(RowDecoder::decode_raw_result_set(result))
            }
        }
    }

    async fn save_object(&self, object: &Object) -> Result<()> {
        let is_new = object.inner.is_new.load(Ordering::SeqCst);
        if is_new {
            self.create_object(object).await
        } else {
            self.update_object(object).await
        }
    }

    async fn delete_object(&self, object: &Object) -> Result<()> {
        if object.inner.is_new.load(Ordering::SeqCst) {
            return Err(Error::object_is_not_saved_thus_cant_be_deleted());
        }
        let model = object.model();
        let r#where = Query::where_from_identifier(object, self.dialect());
        let stmt = SQL::delete_from(model.table_name()).r#where(r#where).to_string(self.dialect());
        // println!("see delete stmt: {}", stmt);
        let result = self.conn().execute(QuaintQuery::from(stmt)).await;
        if result.is_err() {
            println!("{:?}", result.err().unwrap());
            return Err(Error::unknown_database_write_error());
        } else {
            Ok(())
        }
    }

    async fn find_unique<'a>(&'a self, model: &'static Model, finder: &'a Value, _mutation_mode: bool, action: Action, action_source: Initiator) -> Result<Option<Object>> {
        let objects = Execution::query_objects(self.conn(), model, finder, self.dialect(), action, action_source.clone(), Arc::new(self.clone())).await?;
        if objects.is_empty() {
            Ok(None)
        } else {
            Ok(Some(objects.get(0).unwrap().clone()))
        }
    }

    async fn find_many<'a>(&'a self, model: &'static Model, finder: &'a Value, _mutation_mode: bool, action: Action, action_source: Initiator) -> Result<Vec<Object>> {
        Execution::query_objects(self.conn(), model, finder, self.dialect(), action, action_source, Arc::new(self.clone())).await
    }

    async fn count(&self, model: &Model, finder: &Value) -> Result<usize> {
        match Execution::query_count(self.conn(), model, finder, self.dialect()).await {
            Ok(c) => Ok(c as usize),
            Err(e) => Err(e),
        }
    }

    async fn aggregate(&self, model: &Model, finder: &Value) -> Result<Value> {
        Execution::query_aggregate(self.conn(), model, finder, self.dialect()).await
    }

    async fn group_by(&self, model: &Model, finder: &Value) -> Result<Value> {
        Execution::query_group_by(self.conn(), model, finder, self.dialect()).await
    }

    async fn transaction(&self) -> Result<Arc<dyn Connection>> {
        Ok(Arc::new(SQLConnection {
            dialect: self.dialect,
            conn: self.conn.clone(),
            tran: Some(Arc::new(start_owned_transaction(self.conn.clone(), None).await.unwrap()))
        }))
    }

    async fn commit(&self) -> Result<()> {
        if let Some(tran) = &self.tran {
            tran.commit().await.unwrap()
        }
        Ok(())
    }
}
