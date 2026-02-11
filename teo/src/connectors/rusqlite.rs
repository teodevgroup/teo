use std::{borrow::Cow, str::FromStr};

use rusqlite::{Connection, Error, Params, Rows};
use teo_column_type::sqlite;
use crate::{connection::SyncConnection, migration::{ColumnDef, IndexColumnDef, IndexDef, SyncMigration, TableDef}, types::{Schema, SortOrder}};

impl SyncConnection for Connection {

    type Err = Error;

    fn migrate<S>(&self) -> Result<(), Self::Err> where S: Schema {
        SyncMigration::migrate::<S>(self)
    }
}

impl SyncMigration for Connection {

    type Err = Error;

    type ColumnType = sqlite::ColumnType;

    fn execute_without_params(&self, q: &str) -> Result<(), Self::Err> {
        self.execute(q, ()).map(|_| ())
    }

    fn ident_quote_char() -> &'static str {
        "\""
    }

    fn string_quote_char() -> &'static str {
        "'"
    }

    fn exist_enum_names(&self) -> Result<Vec<String>, Self::Err> {
        Ok(Vec::new())
    }

    fn enum_create_statement(&self, _enum_def: &crate::migration::EnumDef) -> String {
        unreachable!()
    }

    fn enum_drop_statement(&self, _enum_name: &str) -> String {
        unreachable!()
    }

    fn add_enum_variant_statement(&self, _enum_name: &str, _variant_name: &str) -> String {
        unreachable!()
    }

    fn exist_enum_def(&self, _enum_name: &'static str) -> Result<crate::migration::EnumDef, Self::Err> {
        unreachable!()
    }

    fn exist_table_names(&self) -> Result<Vec<String>, Self::Err> {
        let mut statement = self.prepare("select name from sqlite_master where type='table'")?;
        let rows = statement.query_map((), |row| {
            let name: String = row.get(0)?;
            Ok(name)
        })?;
        let table_names = rows.filter_map(Result::ok).filter_map(|name| {
            if name.starts_with("_") {
                None
            } else {
                Some(name)
            }
        }).collect();
        Ok(table_names)
    }

    fn create_table_statement(&self, table_def: &TableDef<Self::ColumnType>) -> String {
        let columns: Vec<String> = table_def.columns.iter().map(|c| self.column_statement(c)).collect();
        let columns_joined = columns.join(",");
        format!(r#"create table if not exists {}{}{}({})"#,
            Self::ident_quote_char(),
            table_def.name,
            Self::ident_quote_char(),
            columns_joined)
    }

    fn column_statement(&self, column_def: &crate::migration::ColumnDef<Self::ColumnType>) -> String {
        let not_null = if column_def.nullable { "" } else { " not null" };
        let default = if let Some(default) = &column_def.default { format!(" default {}", default) } else { "".to_owned() };
        format!(r#"{}{}{} {}{}{}"#,
            Self::ident_quote_char(),
            column_def.name,
            Self::ident_quote_char(),
            column_def.ty.to_string(),
            not_null,
            default)
    }

    #[inline]
    fn defined_table_defs<S>(&self) -> Vec<TableDef<sqlite::ColumnType>> where S: Schema {
        S::sqlite_table_defs()
    }

    fn exist_table_def(&self, table_name: &'static str) -> Result<TableDef<Self::ColumnType>, Self::Err> {
        let column_sql = format!("pragma table_info({}{}{})",
            Self::ident_quote_char(),
            table_name,
            Self::ident_quote_char());
        let mut column_statement = self.prepare(&column_sql)?;
        let mut columns = column_statement.query_map((), |row| {
            let name: String = row.get("name")?;
            let ty: String = row.get("type")?;
            let notnull: bool = row.get("notnull")?;
            Ok(ColumnDef {
                name: Cow::Owned(name),
                ty: sqlite::ColumnType::from_str(&ty).unwrap(),
                nullable: !notnull,
                default: None,
            })
        })?.filter_map(Result::ok).collect();
        let indexes_sql = format!("select * from sqlite_master where type = 'index' and tbl_name = '{}' and not name like 'sqlite%'", table_name);
        let mut indexes_statement = self.prepare(&indexes_sql)?;
        let indexes_rows = indexes_statement.query_map((), |row| {
            let name: String = row.get("name")?;
            let sql: String = row.get("sql")?;
            Ok((name, sql))
        })?;
        let indexes: Vec<IndexDef> = indexes_rows.filter_map(Result::ok).filter_map(|(name, sql)| {
            let columns = if let Some(start) = sql.find('(') && let Some(end) = sql.find(')') {
                let cols_def = &sql[start + 1..end];
                cols_def.split(",").map(|col| {
                    let trimmed = col.trim();
                    let mut name_and_order = trimmed.split(" ");
                    let name = name_and_order.nth(0).unwrap().trim();
                    let order = name_and_order.nth(1).unwrap().trim();
                    IndexColumnDef {
                        name: Cow::Owned(name.to_string()),
                        order: SortOrder::from_str(order).unwrap()
                    }
                }).collect()
            } else {
                vec![]
            };
            Some(IndexDef {
                name: Cow::Owned(name.to_string()),
                columns
            })
        }).collect();
        Ok(TableDef {
            name: table_name,
            columns,
            indexes,
        })
    }
}
