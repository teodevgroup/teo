use std::{borrow::Cow, str::FromStr};

use mysql_async::{Conn, Error, Row, prelude::Queryable};
use crate::{connection::AsyncConnection, migration::{AsyncMigration, ColumnDef, EnumDef, IndexColumnDef, IndexDef, TableDef}, types::{Schema, SortOrder}};
use teo_column_type::mysql;

impl AsyncConnection for Conn {

    type Err = Error;

    async fn migrate<S>(&mut self) -> Result<(), Self::Err> where S: Schema {
        AsyncMigration::migrate::<S>(self).await
    }
}

impl AsyncMigration for Conn {

    type Err = Error;

    type ColumnType = mysql::ColumnType;

    #[inline]
    async fn execute_without_params(&mut self, q: &str) -> Result<(), Self::Err> {
        self.exec_drop(q, ()).await
    }

    #[inline]
    fn ident_quote_char() -> &'static str {
        "`"
    }

    #[inline]
    fn string_quote_char() -> &'static str {
        "'"
    }

    async fn exist_enum_names(&mut self) -> Result<Vec<String>, Self::Err> {
        Ok(Vec::new())
    }

    fn enum_create_statement(&self, _enum_def: &EnumDef) -> String {
        unreachable!()
    }

    fn enum_drop_statement(&self, _enum_name: &str) -> String {
        unreachable!()
    }

    fn add_enum_variant_statement(&self, _enum_name: &str, _variant_name: &str) -> String {
        unreachable!()
    }

    async fn exist_enum_def(&mut self, _enum_name: &'static str) -> Result<EnumDef, Self::Err> {
        unreachable!()
    }

    async fn exist_table_names(&mut self) -> Result<Vec<String>, Self::Err> {
        let statement = "show tables";
        let rows: Vec<Row> = self.query(statement).await?;
        let mut table_names = Vec::new();
        for row in rows {
            let name: String = row.get_opt(0).unwrap().unwrap();
            table_names.push(name);
        }
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

    fn column_statement(&self, column_def: &ColumnDef<Self::ColumnType>) -> String {
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
    fn defined_table_defs<S>(&self) -> Vec<TableDef<mysql::ColumnType>> where S: Schema {
        S::mysql_table_defs()
    }

    async fn exist_table_def(&mut self, table_name: &str) -> Result<TableDef<Self::ColumnType>, Self::Err> {
        let columns_statement = format!("describe `{table_name}`");
        let column_rows: Vec<Row> = self.query(&columns_statement).await?;
        let mut columns = vec![];
        for row in &column_rows {
            let name: String = row.get_opt("Field").unwrap().unwrap();
            let ty: String = row.get_opt("Type").unwrap().unwrap();
            let nullable: String = row.get_opt("Null").unwrap().unwrap();
            columns.push(ColumnDef {
                name: Cow::Owned(name),
                ty: mysql::ColumnType::from_str(&ty).unwrap(),
                nullable: nullable == "YES",
                default: None
            });
        }
        let index_statement = format!("show indexes from `{table_name}`");
        let index_rows: Vec<Row> = self.query(&index_statement).await?;
        let mut indexes = vec![];
        for row in &index_rows {
            let index_name: String = row.get_opt("Key_name").unwrap().unwrap();
            let column_name: String = row.get_opt("Column_name").unwrap().unwrap();
            let order: String = row.get_opt("Collation").unwrap().unwrap();
            let _non_unique: bool = row.get_opt("Non_unique").unwrap().unwrap();
            let column = IndexColumnDef {
                name: Cow::Owned(column_name),
                order: if &order == "D" { SortOrder::Desc } else { SortOrder::Asc }
            };
            if let Some(position) = indexes.iter().position(|i: &IndexDef| &i.name == &index_name) {
                indexes.get_mut(position).unwrap().columns.push(column);
            } else {
                indexes.push(IndexDef {
                    name: Cow::Owned(index_name.clone()),
                    columns: vec![column]
                })
            }
        }
        Ok(TableDef {
            name: Cow::Owned(table_name.to_string()),
            columns,
            indexes,
        })
    }
}
