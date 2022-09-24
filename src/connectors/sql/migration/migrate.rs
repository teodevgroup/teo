use sqlx::{AnyPool, Connection, Database, Executor, MySqlPool, Pool};
use crate::connectors::sql::query_builder::{SQL, SQLDialect, ToSQLString};
use crate::core::model::Model;

pub async fn migrate(dialect: SQLDialect, pool: &mut AnyPool, db_name: String, models: &Vec<Model>, reset_database: bool) {
    // drop database if needed
    if reset_database {
        let stmt = SQL::drop().database(&db_name).
            if_exists().to_string(dialect);
        let result = pool.execute(&*stmt).await;
        println!("see drop database result {:?}", result);
    }
    // use database
    let stmt = SQL::create().database(&db_name).if_not_exists().to_string(SQLDialect::MySQL);
    // stmt.ignore(&mut conn).await.unwrap();
    let stmt = SQL::r#use().database(&db_name).to_string(SQLDialect::MySQL);
    // stmt.ignore(&mut conn).await.unwrap();
}
