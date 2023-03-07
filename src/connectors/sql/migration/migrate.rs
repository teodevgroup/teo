use std::fs;
use quaint::pooled::Quaint;
use quaint::prelude::Queryable;
use quaint::ast::Query;
use quaint::ast::Query::Select;
use quaint::ast::Comparable;
use url::Url;
use crate::connectors::sql::migration::sql::{sqlite_auto_increment_query, sqlite_list_indices_query};
use super::super::url::url_utils;
use crate::connectors::sql::schema::column::decoder::ColumnDecoder;
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
            println!("see url: {:?}", absolutized_url);
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
            let result = conn.query(Query::from(stmt)).await;
            if result.is_err() {
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

    pub(crate) async fn migrate(dialect: SQLDialect, pool: &Quaint, models: &Vec<Model>) {
        match dialect {
            SQLDialect::SQLite => Self::migrate_sqlite_database(pool, models).await,
            _ => Self::migrate_server_database(dialect, pool, models).await,
        }
    }

    pub(crate) async fn migrate_sqlite_database(pool: &Quaint, models: &Vec<Model>) {
        let dialect = SQLDialect::SQLite;
        let conn = pool.check_out().await.unwrap();
        // compare each table and do migration
        for model in models {
            if model.r#virtual() {
                continue
            }
            let table_name = model.table_name();
            let show_table = quaint::ast::Select::from_table("sqlite_master").column("name").and_where("type".equals("table")).and_where("name".equals(model.table_name()));
            let table_result = conn.query(Query::from(show_table)).await.unwrap();
            if table_result.is_empty() {
                // table not exist, create table
                let stmt = SQLCreateTableStatement::from(model).to_string(dialect);
                // println!("EXECUTE SQL for create table: {}", &stmt);
                conn.execute(Query::from(stmt)).await.unwrap();
            } else {
                let columns_result = conn.query(Query::from(format!("pragma table_info('{}')", table_name))).await.unwrap();
                let indices_result = conn.query(Query::from(sqlite_list_indices_query(table_name))).await.unwrap();
                let auto_increment_result = conn.query(Query::from(sqlite_auto_increment_query(table_name))).await.unwrap();
                let db_columns = ColumnDecoder::decode_sqlite_columns(columns_result, indices_result, auto_increment_result);
                let model_columns = ColumnDecoder::decode_model_columns(model);
                let need_to_alter_any_column = ColumnDecoder::need_to_alter_any_columns(&db_columns, &model_columns);
                if need_to_alter_any_column {
                    println!("need to alter any column");
                    unreachable!()
                } else {
                    let (columns_to_add, columns_to_remove) = ColumnDecoder::sqlite_add_and_remove(&db_columns, &model_columns);
                    // update indices here
                    for column in columns_to_add {
                        // add column
                        let stmt = SQL::alter_table(table_name).add(column.clone()).to_string(SQLDialect::SQLite);
                        conn.execute(Query::from(stmt)).await.unwrap();
                    }
                    for column in columns_to_remove {
                        // remove column
                        let stmt = SQL::alter_table(table_name).drop_column(column.name()).to_string(SQLDialect::SQLite);
                        conn.execute(Query::from(stmt)).await.unwrap();
                    }
                }
            }
        }
    }

    pub(crate) async fn migrate_server_database(dialect: SQLDialect, pool: &Quaint, models: &Vec<Model>) {
        let conn = pool.check_out().await.unwrap();
        for model in models {
            if model.r#virtual() {
                continue
            }
            let show_table = SQL::show().tables().like(model.table_name()).to_string(dialect);
            let table_result = conn.query(Query::from(show_table)).await.unwrap();
            if table_result.is_empty() {
                // table not exist, create table
                let stmt = SQLCreateTableStatement::from(model).to_string(dialect);
                // println!("EXECUTE SQL for create table: {}", &stmt);
                conn.execute(Query::from(stmt)).await.unwrap();
            } else {
                // table exist, migrate
                let table_name = model.table_name();
                let mut reviewed_columns: Vec<String> = Vec::new();
                let db_table_columns = conn.query(if dialect == SQLDialect::MySQL {
                    let desc = SQL::describe(table_name).to_string(dialect);
                    Query::from(desc)
                } else {
                    let desc = SQL::describe(table_name).to_string(dialect);
                    Query::from(desc)
                }).await.unwrap();
                for db_table_column in db_table_columns {
                    // println!("table column {:?}", db_table_column);
                    let db_column = ColumnDecoder::decode(db_table_column, dialect);
                    let schema_field = model.field_with_column_name(db_column.name());
                    if schema_field.is_none() {
                        // remove this column
                        let stmt = SQL::alter_table(table_name).drop_column(db_column.name()).to_string(dialect);
                        // println!("EXECUTE SQL for remove column: {}", &stmt);
                        conn.execute(Query::from(stmt)).await.unwrap();
                    } else {
                        // compare column definition
                        let schema_column: SQLColumn = schema_field.unwrap().into();
                        if schema_column != db_column {
                            // this column is different, alter it
                            let alter = SQL::alter_table(table_name).modify(schema_column).to_string(dialect);
                            // println!("EXECUTE SQL for alter column: {}", &alter);
                            conn.execute(Query::from(alter)).await.unwrap();
                        }
                        reviewed_columns.push(db_column.name().to_owned());
                    }
                }
                for field in model.fields() {
                    if !reviewed_columns.contains(&field.column_name().to_string()) {
                        let sql_column_def: SQLColumn = field.into();
                        // add this column
                        let add = SQL::alter_table(table_name).add(sql_column_def).to_string(dialect);
                        // println!("EXECUTE SQL for add column: {}", &add);
                        conn.execute(Query::from(add)).await.unwrap();
                    }
                }
            }
        }
    }
}
