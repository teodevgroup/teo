use url::Url;
use serde_json::{Map, Value as JsonValue};
use async_trait::async_trait;
use mysql_async::Pool;
use mysql_async::prelude::{Query, Queryable};
use crate::connectors::sql_shared::sql::{SQL, SQLDialect, ToSQLString};
use crate::connectors::sql_shared::table_create_statement;
use crate::core::builders::graph_builder::GraphBuilder;
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::database_type::DatabaseType;
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::error::ActionError;


#[derive(Debug)]
pub(crate) struct MySQLConnector {
    pool: Pool,
    database_name: String,
}

impl MySQLConnector {
    pub async fn new(pool: Pool, database_name: String, models: &Vec<Model>) -> MySQLConnector {
        for model in models {
            let stmt_string = table_create_statement(model).to_string(SQLDialect::MySQL);
            println!("{}", stmt_string);
            //sqlx::query(&stmt_string).execute(&pool).await;
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
