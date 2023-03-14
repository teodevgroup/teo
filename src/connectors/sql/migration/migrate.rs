use std::collections::HashSet;
use std::fs;
use itertools::Itertools;
use maplit::hashset;
use quaint_forked::pooled::{PooledConnection, Quaint};
use quaint_forked::prelude::Queryable;
use quaint_forked::ast::Query;
use crate::connectors::sql::migration::sql::{sqlite_auto_increment_query, sqlite_list_indices_query};
use super::super::url::url_utils;
use crate::connectors::sql::schema::column::decoder::{ColumnDecoder, ColumnManipulation};
use crate::connectors::sql::stmts::create::table::SQLCreateTableStatement;
use crate::connectors::sql::stmts::SQL;
use crate::connectors::sql::schema::column::SQLColumn;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::model::Model;
use crate::connectors::sql::schema::value::encode::ToSQLString;
use crate::core::field::Sort;
use crate::core::model::index::{ModelIndex, ModelIndexItem, ModelIndexType};
use crate::core::pipeline::ctx::Ctx;
use crate::prelude::Value;

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

    pub(crate) async fn table_has_records(dialect: SQLDialect, conn: &PooledConnection, table_name: &str) -> bool {
        let escape = dialect.escape();
        let sql = format!("select * from {escape}{table_name}{escape} limit 1");
        !conn.query(Query::from(sql)).await.unwrap().is_empty()
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
                Self::create_table(dialect, &conn, model).await;
            } else {
                // remove from list
                let index = db_tables.clone().iter().find_position(|x| *x == table_name).unwrap().0;
                db_tables.remove(index);
                // start migrate for this table
                let model_columns = ColumnDecoder::decode_model_columns(model);
                let db_columns = Self::db_columns(&conn, dialect, table_name).await;
                let need_to_alter_any_column = ColumnDecoder::need_to_alter_any_columns(&db_columns, &model_columns);
                if need_to_alter_any_column && dialect == SQLDialect::SQLite {
                    panic!("SQLite doesn't support column altering");
                }
                let table_has_records = Self::table_has_records(dialect, &conn, table_name).await;
                let db_indices = Self::db_indices(dialect, &conn, model).await;
                let model_indices = Self::normalized_model_indices(model.indices(), dialect, table_name);
                // here update columns and indices
                let manipulations = ColumnDecoder::manipulations(&db_columns, &model_columns, &db_indices, &model_indices, model);
                if table_has_records && manipulations.iter().find(|m| m.is_add_column_non_null()).is_some() && model.allows_drop_when_migrate() {
                    Self::drop_table(dialect, &conn, table_name).await;
                    Self::create_table(dialect, &conn, model).await;
                } else {
                    for m in manipulations.iter() {
                        match m {
                            ColumnManipulation::CreateIndex(index) => {

                            }
                            ColumnManipulation::DropIndex(index) => {

                            }
                            ColumnManipulation::AddColumn(column, action, default) => {
                                if column.not_null() && default.is_none() {
                                    // if any records, just raise here
                                    let has_records = Self::table_has_records(dialect, &conn, table_name).await;
                                    if has_records {
                                        panic!("Cannot add new non null column `{}', table `{}' has records. Consider add a default value or drop the table.", column.name(), table_name)
                                    }
                                }
                                let mut c = column.clone().clone();
                                if default.is_some() {
                                    c.set_default(Some(default.as_ref().unwrap().to_string(dialect)));
                                }
                                let stmt = SQL::alter_table(table_name).add(c).to_string(dialect);
                                conn.execute(Query::from(stmt)).await.unwrap();
                                if let Some(action)= action {
                                    let ctx = Ctx::initial_state_with_value(Value::Null);
                                    action.process(ctx).await.unwrap();
                                }
                            }
                            ColumnManipulation::AlterColumn(old_column, new_column, _action) => {
                                if dialect != SQLDialect::PostgreSQL {
                                    let alter = SQL::alter_table(table_name).modify(new_column.clone().clone()).to_string(dialect);
                                    conn.execute(Query::from(alter)).await.unwrap();
                                } else {
                                    let clauses = Self::psql_alter_clauses(table_name, *old_column, *new_column);
                                    for clause in clauses {
                                        conn.execute(Query::from(clause)).await.unwrap();
                                    }
                                }
                            }
                            ColumnManipulation::RemoveColumn(name, action) => {
                                if let Some(action)= action {
                                    let ctx = Ctx::initial_state_with_value(Value::Null);
                                    action.process(ctx).await.unwrap();
                                }
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
        }
        // drop tables
        for table in db_tables {
            Self::drop_table(dialect, &conn, &table).await;
        }
    }

    async fn drop_table(dialect: SQLDialect, conn: &PooledConnection, table: &str) {
        let escape = dialect.escape();
        let sql = format!("DROP TABLE {escape}{table}{escape}");
        conn.execute(Query::from(sql)).await.unwrap();
    }

    async fn create_table(dialect: SQLDialect, conn: &PooledConnection, model: &Model) {
        // create table
        let stmt = SQLCreateTableStatement::from(model).to_string(dialect);
        conn.execute(Query::from(stmt)).await.unwrap();
        // create indices
        for index in model.indices() {
            // primary is created when creating table
            if index.r#type().is_primary() { continue }
            let stmt = index.to_sql_create(dialect, model.table_name());
            conn.execute(Query::from(stmt)).await.unwrap();
        }
    }

    fn psql_alter_clauses(table: &str, old_column: &SQLColumn, new_column: &SQLColumn) -> Vec<String> {
        let mut result = vec![];
        let name = new_column.name();
        let escape = SQLDialect::PostgreSQL.escape();
        if old_column.r#type() != new_column.r#type() {
            result.push(format!("ALTER TABLE {escape}{table}{escape} ALTER COLUMN {escape}{name}{escape} TYPE {}", new_column.r#type().to_string(SQLDialect::PostgreSQL)));
        }
        if old_column.default().is_none() && new_column.default().is_some() {
            result.push(format!("ALTER TABLE {escape}{table}{escape} ALTER COLUMN {escape}{name}{escape} SET DEFAULT {}", new_column.default().unwrap()));
        } else if old_column.default().is_some() && new_column.default().is_none() {
            result.push(format!("ALTER TABLE {escape}{table}{escape} ALTER COLUMN {escape}{name}{escape} DROP DEFAULT"));
        } else if old_column.default().is_some() && new_column.default().is_some() {
            if old_column.default() == new_column.default() {
                result.push(format!("ALTER TABLE {escape}{table}{escape} ALTER COLUMN {escape}{name}{escape} SET DEFAULT {}", new_column.default().unwrap()));
            }
        }
        result
    }

    fn normalized_model_indices(indices: &Vec<ModelIndex>, dialect: SQLDialect, table_name: &str) -> HashSet<ModelIndex> {
        indices.iter().map(|index| {
            let mut index = index.clone();
            let sql_name_cow = index.sql_name(table_name);
            let sql_name = sql_name_cow.as_ref().to_owned();
            index.set_name(sql_name);
            index
        }).collect()
    }

    async fn db_indices(dialect: SQLDialect, conn: &PooledConnection, model: &Model) -> HashSet<ModelIndex> {
        match dialect {
            SQLDialect::PostgreSQL => Self::psql_db_indices(conn, model).await,
            SQLDialect::MySQL => Self::mysql_db_indices(conn, model).await,
            SQLDialect::SQLite => Self::sqlite_db_indices(conn, model).await,
            _ => unreachable!(),
        }
    }

    async fn mysql_db_indices(conn: &PooledConnection, model: &Model) -> HashSet<ModelIndex> {
        let table_name = model.table_name();
        let sql = format!("SHOW INDEX FROM `{}`", table_name);
        let result_set = conn.query(Query::from(sql)).await.unwrap();
        let mut indices = vec![];
        for row in result_set {
            let index_name = row.get("Key_name").unwrap().as_str().unwrap();
            let column_name = row.get("Column_name").unwrap().as_str().unwrap();
            let order = Sort::from_mysql_str(row.get("Collation").unwrap().as_str().unwrap()).unwrap();
            if let Some(position) = indices.iter().position(|m: &ModelIndex| m.name().unwrap() == index_name) {
                let model_index = indices.get_mut(position).unwrap();
                let item = ModelIndexItem::new(column_name, order, None);
                model_index.append_item(item);
            } else {
                let is_unique = !row.get("Non_unique").unwrap().as_bool().unwrap();
                let item = ModelIndexItem::new(column_name, order, None);
                indices.push(ModelIndex::new(
                    if index_name == "PRIMARY" { ModelIndexType::Primary } else if is_unique { ModelIndexType::Unique } else { ModelIndexType::Index },
                    Some(index_name),
                    vec![item],
                ))
            }
        }
        indices.into_iter().collect()
    }

    async fn psql_db_indices(conn: &PooledConnection, model: &Model) -> HashSet<ModelIndex> {
        let table_name = model.table_name();
        let sql = format!(r#"SELECT     irel.relname                           AS index_name,
           a.attname                              AS column_name,
           i.indisunique                          AS is_unique,
           i.indisprimary                         AS is_primary,
           1 + Array_position(i.indkey, a.attnum) AS column_position,
           CASE o.OPTION
                                 & 1
                      WHEN 1 THEN 'DESC'
                      ELSE 'ASC'
           END      AS order
FROM       pg_index AS i
join       pg_class AS trel
ON         trel.oid = i.indrelid
join       pg_namespace AS tnsp
ON         trel.relnamespace = tnsp.oid
join       pg_class AS irel
ON         irel.oid = i.indexrelid
cross join lateral unnest (i.indkey) WITH ordinality    AS c (colnum, ordinality)
left join  lateral unnest (i.indoption) WITH ordinality AS o (OPTION, ordinality)
ON         c.ordinality = o.ordinality
join       pg_attribute AS a
ON         trel.oid = a.attrelid
AND        a.attnum = c.colnum
WHERE      tnsp.nspname='public'
AND        trel.relname='{table_name}'
GROUP BY   tnsp.nspname,
           trel.relname,
           irel.relname,
           i.indisunique,
           i.indisprimary,
           a.attname,
           array_position(i.indkey, a.attnum),
           o.OPTION ORDER BY column_position
"#);
        let result_set = conn.query(Query::from(sql)).await.unwrap();
        let mut indices = vec![];
        for row in result_set {
            let index_name = row.get("index_name").unwrap().as_str().unwrap();
            let column_name = row.get("column_name").unwrap().as_str().unwrap();
            let order = Sort::from_str(row.get("order").unwrap().as_str().unwrap()).unwrap();
            if let Some(position) = indices.iter().position(|m: &ModelIndex| m.name().unwrap() == index_name) {
                let model_index = indices.get_mut(position).unwrap();
                let item = ModelIndexItem::new(column_name, order, None);
                model_index.append_item(item);
            } else {
                let is_unique = row.get("is_unique").unwrap().as_bool().unwrap();
                let is_primary = row.get("is_primary").unwrap().as_bool().unwrap();
                let item = ModelIndexItem::new(column_name, order, None);
                indices.push(ModelIndex::new(
                    if is_primary { ModelIndexType::Primary } else if is_unique { ModelIndexType::Unique} else { ModelIndexType::Index },
                    Some(index_name),
                    vec![item],
                ))
            }
        }
        indices.into_iter().collect()
    }

    async fn sqlite_db_indices(conn: &PooledConnection, model: &Model) -> HashSet<ModelIndex> {
        let table_name = model.table_name();
        let sql = format!(r#"SELECT
    il.name as index_name,
    ii.name as column_name,
    CASE il.origin when 'pk' then 1 else 0 END as is_primary,
    CASE il.[unique] when 1 then 0 else 1 END as non_unique,
    il.[unique] as is_unique,
    il.partial,
    il.seq as seq,
    ii.desc as `desc`
FROM sqlite_master AS m,
    pragma_index_list(m.name) AS il,
    pragma_index_xinfo(il.name) AS ii
WHERE
    m.type = 'table'
    and m.tbl_name = '{}'
    and ii.name is not null
GROUP BY
    m.tbl_name,
    il.name,
    ii.name,
    il.origin,
    il.partial,
    il.seq,
    ii.`desc`
ORDER BY 1,6"#, table_name);
        let result_set = conn.query(Query::from(sql)).await.unwrap();
        let mut indices = vec![];
        for row in result_set {
            let index_name = row.get("index_name").unwrap().as_str().unwrap();
            let column_name = row.get("column_name").unwrap().as_str().unwrap();
            let order = Sort::from_desc_bool(row.get("desc").unwrap().as_bool().unwrap());
            if let Some(position) = indices.iter().position(|m: &ModelIndex| m.name().unwrap() == index_name) {
                let model_index = indices.get_mut(position).unwrap();
                let item = ModelIndexItem::new(column_name, order, None);
                model_index.append_item(item);
            } else {
                let is_unique = row.get("is_unique").unwrap().as_bool().unwrap();
                let is_primary = row.get("is_primary").unwrap().as_bool().unwrap();
                let item = ModelIndexItem::new(column_name, order, None);
                indices.push(ModelIndex::new(
                    if is_primary { ModelIndexType::Primary } else if is_unique { ModelIndexType::Unique} else { ModelIndexType::Index },
                    Some(index_name),
                    vec![item],
                ))
            }
        }
        indices.into_iter().collect()
    }
}
