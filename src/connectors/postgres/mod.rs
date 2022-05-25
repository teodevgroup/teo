use url::Url;
use sea_query::PostgresQueryBuilder;
use serde_json::{Value as JsonValue};
use sqlx::PgPool;
use async_trait::async_trait;
use crate::connectors::sql_shared::table_create_statement;
use crate::core::builders::GraphBuilder;
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::error::ActionError;


#[derive(Debug)]
pub(crate) struct PostgresConnector {
    pool: PgPool,
    database_name: String,
}

impl PostgresConnector {
    pub async fn new(pool: PgPool, database_name: String, models: &Vec<Model>) -> PostgresConnector {
        for model in models {
            let stmt_string = table_create_statement(model).to_string(PostgresQueryBuilder);
            sqlx::query(&stmt_string).execute(&pool).await;
        }
        PostgresConnector { pool, database_name }
    }
}

#[async_trait]
impl Connector for PostgresConnector {

    async fn save_object(&self, object: &Object) -> Result<(), ActionError> {
        todo!()
    }

    async fn delete_object(&self, object: &Object) -> Result<(), ActionError> {
        todo!()
    }

    async fn find_unique(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<Object, ActionError> {
        todo!()
    }

    async fn find_first(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<Object, ActionError> {
        todo!()
    }

    async fn find_many(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<Vec<Object>, ActionError> {
        todo!()
    }

    async fn count(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<usize, ActionError> {
        todo!()
    }
}

unsafe impl Send for PostgresConnector {}
unsafe impl Sync for PostgresConnector {}

#[derive(Debug)]
pub(crate) struct PostgresConnectorBuilder {
    url: &'static str
}

impl PostgresConnectorBuilder {
    pub(crate) fn new(url: &'static str) -> PostgresConnectorBuilder {
        PostgresConnectorBuilder { url }
    }
}

#[async_trait]
impl ConnectorBuilder for PostgresConnectorBuilder {
    async fn build_connector(&self, models: &Vec<Model>, reset_database: bool) -> Box<dyn Connector> {
        let url = Url::parse(self.url);
        match url {
            Ok(mut url) => {
                let database_name = url.path()[1..].to_string();
                url.set_path("/");
                let string_url = url.to_string();
                let pool = PgPool::connect(&string_url).await.unwrap();
                if reset_database {
                    sqlx::query(&format!("DROP DATABASE IF EXISTS {database_name}")).execute(&pool).await;
                }
                sqlx::query(&format!("CREATE DATABASE IF NOT EXISTS {database_name}")).execute(&pool).await;
                sqlx::query(&format!("USE DATABASE {database_name}")).execute(&pool).await;
                Box::new(PostgresConnector::new(pool, database_name, models).await)
            }
            Err(err) => {
                panic!("Database URL is invalid.")
            }
        }
    }
}


pub trait PostgresConnectorHelpers {
    fn postgres(&mut self, url: &'static str);
}

impl PostgresConnectorHelpers for GraphBuilder {
    fn postgres(&mut self, url: &'static str) {
        self.connector_builder = Some(Box::new(PostgresConnectorBuilder::new(url)))
    }
}
