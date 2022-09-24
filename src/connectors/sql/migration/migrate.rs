use sqlx::{AnyPool, Connection, Database, Executor, MySqlPool, Pool, Row};
use crate::connectors::sql::query_builder::{SQL, SQLDialect, table_create_statement, ToSQLString};
use crate::core::model::Model;

pub async fn migrate(dialect: SQLDialect, pool: &mut AnyPool, models: &Vec<Model>) {
    // compare each table and do migration
    for model in models {
        let name = model.table_name();
        let show_table = SQL::show().tables().like(name).to_string(dialect);
        let result = pool.fetch_one(&*show_table).await;
        if result.is_err() {
            // table not exist, create table
            let stmt_string = table_create_statement(model).to_string(dialect);
            println!("EXECUTE SQL: {}", stmt_string);
            pool.execute(&*stmt_string).await.unwrap();
        } else {
            // table exist, migrate
            // do nothing for now

        }
    }
}
