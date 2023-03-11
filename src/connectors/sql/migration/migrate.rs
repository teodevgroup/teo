use std::collections::HashSet;
use std::fs;
use itertools::Itertools;
use maplit::hashset;
use quaint_forked::pooled::{PooledConnection, Quaint};
use quaint_forked::prelude::Queryable;
use quaint_forked::ast::Query;
use quaint_forked::ast::Comparable;
use crate::connectors::sql::migration::sql::{sqlite_auto_increment_query, sqlite_list_indices_query};
use super::super::url::url_utils;
use crate::connectors::sql::schema::column::decoder::{ColumnDecoder, ColumnManipulation};
use crate::connectors::sql::stmts::create::table::SQLCreateTableStatement;
use crate::connectors::sql::stmts::SQL;
use crate::connectors::sql::schema::column::SQLColumn;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::model::Model;
use crate::connectors::sql::schema::value::encode::ToSQLString;

pub(crate) struct SQLMigration { }

impl SQLMigration {

    // Create database

    pub(crate) async fn create_database_if_needed(dialect: SQLDialect, url: &str, reset: bool) {
        match dialect {
            SQLDialect::SQLite => Self::create_sqlite_database_if_needed(url, reset).await,
            _ => Self::create_server_database_if_needed(dialect, url, reset).await,
        }
    }

    pub(crate) async fn create_sqlite_database_if_needed(url: &str, reset: bool) {
        let url = url_utils::remove_scheme(url);
        if url_utils::is_memory_url(url) {
            return
        }
        let absolutized_url = url_utils::absolutized(url);
        if absolutized_url.exists() && reset {
            // delete the old one
            let _ = fs::remove_file(&absolutized_url);
        }
        if !absolutized_url.exists() || reset {
            // create a new one
            fs::File::create(absolutized_url).expect("SQLite database file create failed.");
        }
    }

    pub(crate) async fn create_server_database_if_needed(dialect: SQLDialect, url: &str, reset: bool) {
        let url = url_utils::normalized_url(dialect, url);
        let db_name = &url.path()[1..];
        let url_without_db = url_utils::remove_db_path(dialect, &url);
        let pool = Quaint::builder(url_without_db.as_str()).unwrap().build();
        let conn = pool.check_out().await.unwrap();
        // drop database if needed
        if reset {
            let stmt = SQL::drop().database(db_name).if_exists().to_string(dialect);
            conn.execute(Query::from(stmt)).await.unwrap();
        }
        // create database if needed
        if dialect == SQLDialect::PostgreSQL {
            let stmt = format!("select from pg_database where datname = '{}'", db_name);
            let result = conn.query(Query::from(stmt)).await.unwrap();
            if result.is_empty() {
                let stmt = SQL::create().database(db_name).to_string(dialect);
                conn.execute(Query::from(stmt)).await.unwrap();
            }
        } else {
            let stmt = SQL::create().database(db_name).if_not_exists().to_string(dialect);
            conn.execute(Query::from(stmt)).await.unwrap();
        }
        // use database
        if dialect == SQLDialect::PostgreSQL {
            let stmt = format!("SET search_path TO {db_name}");
            conn.execute(Query::from(stmt)).await.unwrap();
        } else {
            let stmt = SQL::r#use().database(db_name).to_string(dialect);
            conn.raw_cmd(&stmt).await.unwrap();
        }
    }

    // Migrate

    pub(crate) async fn is_table_exist(conn: &PooledConnection, dialect: SQLDialect, table_name: &str) -> bool {
        match dialect {
            SQLDialect::SQLite => !conn.query(
                Query::from(
                    quaint_forked::ast::Select::from_table("sqlite_master").column("name").and_where("type".equals("table")).and_where("name".equals(table_name))
                )
            ).await.unwrap().is_empty(),
            SQLDialect::PostgreSQL => {
                !conn.query(
                    Query::from(format!("SELECT table_name FROM information_schema.tables WHERE table_name = '{}'", table_name))
                ).await.unwrap().is_empty()
            },
            _ => !conn.query(
                Query::from(SQL::show().tables().like(table_name).to_string(dialect))
            ).await.unwrap().is_empty()
        }
    }

    pub(crate) async fn db_columns(conn: &PooledConnection, dialect: SQLDialect, table_name: &str) -> HashSet<SQLColumn> {
        match dialect {
            SQLDialect::SQLite => {
                let columns_result = conn.query(Query::from(format!("pragma table_info('{}')", table_name))).await.unwrap();
                let indices_result = conn.query(Query::from(sqlite_list_indices_query(table_name))).await.unwrap();
                let auto_increment_result = conn.query(Query::from(sqlite_auto_increment_query(table_name))).await.unwrap();
                let db_columns = ColumnDecoder::decode_sqlite_columns(columns_result, indices_result, auto_increment_result);
                db_columns
            }
            _ => {
                let mut results = hashset! {};
                let db_table_columns = conn.query(if dialect == SQLDialect::PostgreSQL {
                    let desc = format!("SELECT * FROM information_schema.columns where table_name = '{}'", table_name);
                    Query::from(desc)
                } else {
                    let desc = SQL::describe(table_name).to_string(dialect);
                    Query::from(desc)
                }).await.unwrap();
                for db_table_column in db_table_columns {
                    let db_column = ColumnDecoder::decode(db_table_column, dialect, conn, table_name).await;
                    results.insert(db_column);
                }
                results
            }
        }
    }

    pub(crate) async fn get_db_user_tables(dialect: SQLDialect, conn: &PooledConnection) -> Vec<String> {
        match dialect {
            SQLDialect::MySQL => {
                let sql = "SHOW TABLES";
                let db_result = conn.query(Query::from(sql)).await.unwrap();
                db_result.into_iter().map(|result| { result.into_single().unwrap().to_string().unwrap() }).collect()
            }
            SQLDialect::PostgreSQL => {
                let sql = "SELECT tablename FROM pg_catalog.pg_tables WHERE schemaname != 'pg_catalog' AND schemaname != 'information_schema'";
                let db_result = conn.query(Query::from(sql)).await.unwrap();
                db_result.into_iter().map(|result| { result.into_single().unwrap().to_string().unwrap() }).collect()
            }
            SQLDialect::SQLite => {
                let sql = "SELECT name FROM sqlite_master WHERE type in ('table') AND name not like 'sqlite?_%' escape '?'";
                let db_result = conn.query(Query::from(sql)).await.unwrap();
                db_result.into_iter().map(|result| { result.into_single().unwrap().to_string().unwrap() }).collect()
            }
            _ => unreachable!()
        }
    }

    pub(crate) async fn rename_table(dialect: SQLDialect, conn: &PooledConnection, old_name: &str, new_name: &str) {
        let escape = dialect.escape();
        let sql = format!("ALTER TABLE {escape}{old_name}{escape} RENAME TO {escape}{new_name}{escape}");
        conn.execute(Query::from(sql)).await.unwrap();
    }

    pub(crate) async fn migrate(dialect: SQLDialect, pool: &Quaint, models: &Vec<Model>) {
        let conn = pool.check_out().await.unwrap();
        let mut db_tables = Self::get_db_user_tables(dialect, &conn).await;
        // compare each table and do migration
        for model in models {
            if model.r#virtual() { continue }
            let table_name = model.table_name();
            if let Some(migration) = model.migration() {
                if !db_tables.iter().any(|x| x == table_name) {
                    for old_name in &migration.renamed {
                        if db_tables.contains(old_name) {
                            // rename
                            Self::rename_table(dialect, &conn, old_name.as_str(), table_name).await;
                            let index = db_tables.clone().iter().find_position(|v| *v == old_name).unwrap().0;
                            db_tables.remove(index);
                            db_tables.push(table_name.to_string());
                            break;
                        }
                    }
                }

            }
            let is_table_exist = db_tables.iter().any(|x| x == table_name);
            if !is_table_exist {
                // table not exist, create table
                let stmt = SQLCreateTableStatement::from(model).to_string(dialect);
                conn.execute(Query::from(stmt)).await.unwrap();
            } else {
                let index = db_tables.clone().iter().find_position(|x| *x == table_name).unwrap().0;
                db_tables.remove(index);

                let model_columns = ColumnDecoder::decode_model_columns(model);
                let db_columns = Self::db_columns(&conn, dialect, table_name).await;
                let need_to_alter_any_column = ColumnDecoder::need_to_alter_any_columns(&db_columns, &model_columns);
                if need_to_alter_any_column && dialect == SQLDialect::SQLite {
                    panic!("SQLite doesn't support column altering");
                }
                // here update indices
                // here update columns
                let manipulations = ColumnDecoder::manipulations(&db_columns, &model_columns, model);
                for m in manipulations.iter() {
                    match m {
                        ColumnManipulation::AddColumn(column, action, default) => {
                            let mut c = column.clone().clone();
                            if default.is_some() {
                                c.set_default(Some(default.as_ref().unwrap().to_string(dialect)));
                            }
                            let stmt = SQL::alter_table(table_name).add(c).to_string(dialect);
                            conn.execute(Query::from(stmt)).await.unwrap();
                        }
                        ColumnManipulation::AlterColumn(column, action) => {
                            let alter = SQL::alter_table(table_name).modify(column.clone().clone()).to_string(dialect);
                            conn.execute(Query::from(alter)).await.unwrap();
                        }
                        ColumnManipulation::RemoveColumn(name, action) => {
                            let stmt = SQL::alter_table(table_name).drop_column(name).to_string(dialect);
                            conn.execute(Query::from(stmt)).await.unwrap();
                        }
                        ColumnManipulation::RenameColumn { old, new } => {
                            let stmt = if dialect == SQLDialect::PostgreSQL {
                                format!("ALTER TABLE {} RENAME COLUMN '{}' TO '{}'", table_name, old, new)
                            } else {
                                format!("ALTER TABLE {} RENAME COLUMN `{}` TO `{}`", table_name, old, new)
                            };
                            conn.execute(Query::from(stmt)).await.unwrap();
                        }
                    }
                }
            }
        }
        // drop tables
        for table in db_tables {
            let escape = if dialect == SQLDialect::PostgreSQL { "\"" } else { "`" };
            let sql = format!("DROP TABLE {escape}{table}{escape}");
            conn.execute(Query::from(sql)).await.unwrap();
        }
    }
}
