use url::Url;
use serde_json::{Map, Value as JsonValue};
use async_trait::async_trait;
use tokio_postgres::{Client, NoTls};
use crate::connectors::sql_shared::sql::{SQL, SQLDialect, ToSQLString};
use crate::connectors::sql_shared::table_create_statement;
use crate::core::builders::graph_builder::GraphBuilder;
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::database_type::DatabaseType;
use crate::core::field_type::FieldType;
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::error::ActionError;


#[derive(Debug)]
pub(crate) struct PostgresConnector {
    client: Client,
    database_name: String,
}

impl PostgresConnector {
    pub async fn new(client: Client, database_name: String, models: &Vec<Model>) -> PostgresConnector {
        for model in models {
            let stmt_string = table_create_statement(model).to_string(SQLDialect::PostgreSQL);
            client.execute(&stmt_string, &[]).await.unwrap();
        }
        PostgresConnector { client, database_name }
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

    async fn find_unique(&self, graph: &Graph, model: &Model, finder:  &Map<String, JsonValue>) -> Result<Object, ActionError> {
        todo!()
    }

    async fn find_first(&self, graph: &Graph, model: &Model, finder: &Map<String, JsonValue>) -> Result<Object, ActionError> {
        todo!()
    }

    async fn find_many(&self, graph: &Graph, model: &Model, finder: &Map<String, JsonValue>) -> Result<Vec<Object>, ActionError> {
        todo!()
    }

    async fn count(&self, graph: &Graph, model: &Model, finder: &Map<String, JsonValue>) -> Result<usize, ActionError> {
        todo!()
    }
}

unsafe impl Send for PostgresConnector {}
unsafe impl Sync for PostgresConnector {}

#[derive(Debug)]
pub(crate) struct PostgresConnectorBuilder {
    url: String
}

impl PostgresConnectorBuilder {
    pub(crate) fn new(url: String) -> PostgresConnectorBuilder {
        PostgresConnectorBuilder { url }
    }
}

#[async_trait]
impl ConnectorBuilder for PostgresConnectorBuilder {
    fn inferred_database_type(&self, field_type: &FieldType) -> DatabaseType {
        todo!()
    }

    async fn build_connector(&self, models: &Vec<Model>, reset_database: bool) -> Box<dyn Connector> {
        let mut url = Url::parse(&self.url).unwrap();
        let database_name = url.path()[1..].to_string();
        url.set_path("/");
        let (client, connection) = 
            tokio_postgres::connect(url.as_str(), NoTls).await.unwrap();
        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        if reset_database {
            let stmt = SQL::drop().database(&database_name).if_exists().to_string(SQLDialect::PostgreSQL);
            let _ = client.execute(stmt.as_str(), &[]).await.unwrap();
        }
        let stmt = SQL::create().database(&database_name).if_not_exists().to_string(SQLDialect::PostgreSQL);
        let _ = client.execute(stmt.as_str(), &[]).await.unwrap();
        let stmt = SQL::r#use().database(&database_name).to_string(SQLDialect::PostgreSQL);
        let _ = client.execute(stmt.as_str(), &[]).await.unwrap();
        Box::new(PostgresConnector::new(client, database_name.clone(), models).await)
    }
}
