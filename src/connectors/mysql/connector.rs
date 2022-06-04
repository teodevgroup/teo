use url::Url;
use serde_json::{Map, Value as JsonValue};
use async_trait::async_trait;
use mysql_async::{Conn, Pool, Row, Value};
use mysql_async::prelude::{Query, Queryable};
use crate::connectors::mysql::mysql_column::MySQLColumn;
use crate::connectors::sql_shared::sql::{SQL, SQLColumnDef, SQLDialect, ToSQLString};
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
    conn: Conn,
    database_name: String,
}

impl MySQLConnector {
    pub async fn new(pool: Pool, mut conn: Conn, database_name: String, models: &Vec<Model>) -> MySQLConnector {
        for model in models {
            let name = model.table_name();
            let show_table = SQL::show().tables().like(name).to_string(SQLDialect::MySQL);
            let result: Option<String> = show_table.first(&mut conn).await.unwrap();
            let table_exists = result.is_some();
            if table_exists {
                // migrate
                Self::table_migrate(&mut conn, model).await;
            } else {
                // create table
                let stmt_string = table_create_statement(model).to_string(SQLDialect::MySQL);
                println!("EXECUTE SQL: {}", stmt_string);
                stmt_string.ignore(&mut conn).await.unwrap();
            }
        }
        MySQLConnector { pool, conn, database_name }
    }

    pub async fn table_migrate(conn: &mut Conn, model: &Model) {
        let table_name = model.table_name();
        let desc = SQL::describe(table_name).to_string(SQLDialect::MySQL);
        let mut reviewed_columns: Vec<String> = Vec::new();
        let columns: Vec<Row> = desc.fetch(&mut *conn).await.unwrap();
        for column in &columns {
            let db_column: MySQLColumn = column.into();
            let schema_field = model.field(&db_column.field);
            if schema_field.is_none() {
                // remove this column
                let stmt = SQL::alter_table(table_name).drop_column(db_column.field.clone()).to_string(SQLDialect::MySQL);
                let _ = stmt.ignore(&mut *conn).await;
            }
            let sql_column_def: SQLColumnDef = schema_field.unwrap().into();
            let schema_column: MySQLColumn = (&sql_column_def).into();
            if schema_column != db_column {
                // this column is different, alter it
                let alter = SQL::alter_table(table_name).modify(sql_column_def).to_string(SQLDialect::MySQL);
                let _ = alter.ignore(&mut *conn).await;
            }
            reviewed_columns.push(db_column.field.clone());
        }
        for field in &model.fields_vec {
            if !reviewed_columns.contains(&field.column_name()) {
                let sql_column_def: SQLColumnDef = field.into();
                let add = SQL::alter_table(table_name).add(sql_column_def).to_string(SQLDialect::MySQL);
                let _ = add.ignore(&mut *conn).await;
            }
        }
        // then indices / unique / primary
        let show_index = SQL::show().index_from(table_name).to_string(SQLDialect::MySQL);
        let indices: Vec<Row> = show_index.fetch(&mut *conn).await.unwrap();
        for _index in &indices {

        }

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
