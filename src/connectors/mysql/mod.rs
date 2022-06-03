use url::Url;
use sea_query::{MySqlQueryBuilder, MysqlQueryBuilder};
use serde_json::{Map, Value as JsonValue};
use sqlx::MySqlPool;
use async_trait::async_trait;
use crate::connectors::sql_shared::table_create_statement;
use crate::core::builders::graph_builder::GraphBuilder;
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::database_type::DatabaseType;
use crate::core::field_type::FieldType;
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::error::ActionError;


#[derive(Debug)]
pub(crate) struct MySQLConnector {
    pool: MySqlPool,
    database_name: String,
}

impl MySQLConnector {
    pub async fn new(pool: MySqlPool, database_name: String, models: &Vec<Model>) -> MySQLConnector {
        for model in models {
            let stmt_string = table_create_statement(model).to_string(MysqlQueryBuilder);
            println!("{}", stmt_string);
            sqlx::query(&stmt_string).execute(&pool).await;
        }
        MySQLConnector { pool, database_name }
    }
}

#[async_trait]
impl Connector for MySQLConnector {

    async fn save_object(&self, object: &Object) -> Result<(), ActionError> {
        todo!()
    }

    async fn delete_object(&self, object: &Object) -> Result<(), ActionError> {
        todo!()
    }

    async fn find_unique(&self, graph: &'static Graph, model: &'static Model, finder:  &Map<String, JsonValue>) -> Result<Object, ActionError> {
        todo!()
    }

    async fn find_first(&self, graph: &'static Graph, model: &'static Model, finder: &Map<String, JsonValue>) -> Result<Object, ActionError> {
        todo!()
    }

    async fn find_many(&self, graph: &'static Graph, model: &'static Model, finder: &Map<String, JsonValue>) -> Result<Vec<Object>, ActionError> {
        todo!()
    }

    async fn count(&self, graph: &'static Graph, model: &'static Model, finder: &Map<String, JsonValue>) -> Result<usize, ActionError> {
        todo!()
    }
}

unsafe impl Send for MySQLConnector {}
unsafe impl Sync for MySQLConnector {}

#[derive(Debug)]
pub(crate) struct MySQLConnectorBuilder {
    url: &'static str
}

impl MySQLConnectorBuilder {
    pub(crate) fn new(url: &'static str) -> MySQLConnectorBuilder {
        MySQLConnectorBuilder { url }
    }
}

#[async_trait]
impl ConnectorBuilder for MySQLConnectorBuilder {
    fn inferred_database_type(&self, field_type: &FieldType) -> DatabaseType {
        match field_type {
            FieldType::Undefined => DatabaseType::Undefined,
            FieldType::ObjectId => DatabaseType::Undefined,
            FieldType::Bool => DatabaseType::Bool,
            FieldType::I8 => DatabaseType::TinyInt(false),
            FieldType::I16 => DatabaseType::SmallInt(false),
            FieldType::I32 => DatabaseType::Int(false),
            FieldType::I64 => DatabaseType::BigInt(false),
            FieldType::I128 => DatabaseType::BigInt(false),
            FieldType::U8 => DatabaseType::TinyInt(true),
            FieldType::U16 => DatabaseType::SmallInt(true),
            FieldType::U32 => DatabaseType::Int(true),
            FieldType::U64 => DatabaseType::BigInt(true),
            FieldType::U128 => DatabaseType::BigInt(true),
            FieldType::F32 => DatabaseType::Real,
            FieldType::F64 => DatabaseType::Double,
            FieldType::String => DatabaseType::VarChar(191, None, None),
            FieldType::Date => DatabaseType::Date,
            FieldType::DateTime => DatabaseType::DateTime(3),
            FieldType::Enum(_) => DatabaseType::Undefined,
            FieldType::Vec(_) => DatabaseType::Undefined,
            FieldType::Map(_) => DatabaseType::Undefined,
            FieldType::Object(_) => DatabaseType::Undefined,
        }
    }

    async fn build_connector(&self, models: &Vec<Model>, reset_database: bool) -> Box<dyn Connector> {
        let url = Url::parse(self.url);
        match url {
            Ok(mut url) => {
                let database_name = url.path()[1..].to_string();
                url.set_path("/");
                let string_url = url.to_string();
                let pool = MySqlPool::connect(&string_url).await.unwrap();
                if reset_database {
                    sqlx::query(&format!("DROP DATABASE IF EXISTS {database_name}")).execute(&pool).await;
                }
                sqlx::query(&format!("CREATE DATABASE IF NOT EXISTS {database_name}")).execute(&pool).await;
                sqlx::query(&format!("USE DATABASE {database_name}")).execute(&pool).await;
                Box::new(MySQLConnector::new(pool, database_name, models).await)
            }
            Err(err) => {
                panic!("Database URL is invalid.")
            }
        }
    }
}

pub trait MySQLConnectorHelpers {
    fn mysql(&mut self, url: &'static str);
}

impl MySQLConnectorHelpers for GraphBuilder {
    fn mysql(&mut self, url: &'static str) {
        self.connector_builder = Some(Box::new(MySQLConnectorBuilder::new(url)))
    }
}
