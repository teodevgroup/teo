pub mod save_session;

use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::env;
use std::sync::atomic::Ordering;
use async_trait::async_trait;
use regex::Regex;
use sqlx::{AnyPool, Executor};
use crate::core::model::Model;
use url::Url;
use crate::connectors::sql::schema::r#type::field::ToDatabaseType;
use crate::connectors::sql::connector::save_session::SQLSaveSession;
use crate::connectors::sql::execution::Execution;
use crate::connectors::sql::migration::migrate::SQLMigration;
use crate::connectors::sql::query::Query;
use crate::connectors::sql::stmts::SQL;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::value::encode::ToSQLString;
use crate::core::action::Action;
use crate::core::action::source::ActionSource;
use crate::core::connector::{Connector, SaveSession};
use crate::core::database::r#type::DatabaseType;
use crate::core::error::Error;
use crate::core::field::r#type::FieldType;
use crate::core::input::Input;
use crate::core::result::Result;
use crate::prelude::{Graph, Object, Value};
use crate::teon;

#[derive(Debug)]
pub(crate) struct SQLConnector {
    loaded: bool,
    dialect: SQLDialect,
    pool: AnyPool,
}

impl SQLConnector {
    pub(crate) async fn new(dialect: SQLDialect, url: String, reset_database: bool) -> Self {
        if dialect == SQLDialect::SQLite {
            let filename = &url[7..];
            let loc = PathBuf::from(filename);
            let absolute_location = if loc.is_absolute() {
                loc
            } else {
                env::current_dir().unwrap().join(loc)
            };
            if !absolute_location.exists() {
                fs::File::create(absolute_location).expect("SQLite database create failed.");
            } else if reset_database {
                let _ = fs::remove_file(absolute_location.clone());
                fs::File::create(absolute_location).expect("SQLite database create failed.");
            }
            let pool: AnyPool = AnyPool::connect(url.as_str()).await.unwrap();
            Self { loaded: false, dialect, pool }
        } else {
            let url_result = Url::parse(&url);
            if url_result.is_err() {
                panic!("Data source URL is invalid.");
            }
            let mut url_without_db = url_result.unwrap();
            let database_name = url_without_db.path()[1..].to_string();
            if dialect == SQLDialect::PostgreSQL {
                url_without_db.set_path("/postgres");
            } else {
                url_without_db.set_path("/");
            }
            let mut pool: AnyPool = AnyPool::connect(url_without_db.as_str()).await.unwrap();
            SQLMigration::create_database_if_needed(dialect, &mut pool, &database_name, reset_database).await;
            let pool: AnyPool = AnyPool::connect(url.as_str()).await.unwrap();
            Self { loaded: false, dialect, pool }
        }
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
                    values.push((column_name, val.to_string(self.dialect)));
                }
            } else if let Some(_property) = model.property(key) {
                let val: Value = object.get_property(key).await.unwrap();
                values.push((key, val.to_string(self.dialect)));
            }
        }
        let value_refs: Vec<(&str, &str)> = values.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let stmt = SQL::insert_into(model.table_name()).values(value_refs).returning(auto_keys).to_string(self.dialect);
        if self.dialect == SQLDialect::PostgreSQL {
            match self.pool.fetch_one(&*stmt).await {
                Ok(result) => {
                    let value = Execution::row_to_value(model, object.graph(), &result, self.dialect);
                    for (k, v) in value.as_hashmap().unwrap() {
                        object.set_value(k, v.clone())?;
                    }
                    Ok(())
                }
                Err(err) => {
                    println!("{:?}", err);
                    Err(Self::handle_err_result(self, err))
                }
            }
        } else {
            match self.pool.execute(&*stmt).await {
                Ok(result) => {
                    for key in auto_keys {
                        object.set_value(key, Value::I64(result.last_insert_id().unwrap()))?;
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
        let identifier = object.identifier();
        let r#where = Query::where_from_identifier(object, self.dialect);
        if !value_refs.is_empty() {
            let stmt = SQL::update(model.table_name()).values(value_refs).r#where(&r#where).to_string(self.dialect);
            let result = self.pool.execute(stmt.as_str()).await;
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

    fn handle_err_result(&self, err: sqlx::Error) -> Error {
        let message = err.as_database_error().unwrap().message();
        if self.dialect == SQLDialect::MySQL {
            // mysql
            let regex = Regex::new("Duplicate entry (.+) for key '(.+)'").unwrap();
            if let Some(captures) = regex.captures(message) {
                let keys = captures.get(2).unwrap().as_str().split(".").collect::<Vec<&str>>();
                let key = keys.last().unwrap();
                Error::unique_value_duplicated(key)
            } else {
                Error::unknown_database_write_error()
            }
        } else if self.dialect == SQLDialect::PostgreSQL {
            // duplicate key value violates unique constraint \"users_email_key\"
            let regex = Regex::new("^duplicate key value violates unique constraint").unwrap();
            if regex.is_match(message) {
                Error::unique_value_duplicated_reason("", message)
            } else {
                Error::unknown_database_write_error()
            }
        } else {
            Error::unknown_database_write_error()
        }
    }
}

#[async_trait]
impl Connector for SQLConnector {
    fn default_database_type(&self, field_type: &FieldType) -> DatabaseType {
        field_type.to_database_type(self.dialect)
    }

    async fn is_loaded(&self) -> bool {
        self.loaded
    }

    async fn load(&mut self, models: &Vec<Model>) -> Result<()> {
        Ok(())
    }

    async fn migrate(&mut self, models: &Vec<Model>, reset_database: bool) -> Result<()> {
        // if self.dialect != SQLDialect::SQLite {
        //     SQLMigration::create_database_if_needed(dialect, &mut pool, &database_name, reset_database).await;
        // }
        SQLMigration::migrate(self.dialect, &mut self.pool, models).await;
        Ok(())
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
        if object.inner.is_new.load(Ordering::SeqCst) {
            return Err(Error::object_is_not_saved());
        }
        let model = object.model();
        let r#where = Query::where_from_identifier(object, self.dialect);
        let stmt = SQL::delete_from(model.table_name()).r#where(r#where).to_string(self.dialect);
        let result = self.pool.execute(stmt.as_str()).await;
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
