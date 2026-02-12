use std::{borrow::Cow, str::FromStr};
use teo_column_type::postgres;
use tokio_postgres::{Client, Error};
use crate::{connection::AsyncConnection, migration::{AsyncMigration, ColumnDef, EnumDef, IndexColumnDef, IndexDef, TableDef}, types::{Schema, SortOrder}};

impl AsyncConnection for Client {

    type Err = Error;

    async fn migrate<S>(&mut self) -> Result<(), Self::Err> where S: Schema {
        AsyncMigration::migrate::<S>(self).await
    }
}

impl AsyncMigration for Client {

    type Err = Error;

    type ColumnType = postgres::ColumnType;

    #[inline]
    async fn execute_without_params(&mut self, q: &str) -> Result<(), Self::Err> {
        self.execute(q, &[]).await.map(|_| ())
    }

    #[inline]
    fn ident_quote_char() -> &'static str {
        "\""
    }

    #[inline]
    fn string_quote_char() -> &'static str {
        "'"
    }

    async fn exist_enum_names(&mut self) -> Result<Vec<String>, Self::Err> {
        let statement = r#"select distinct pg_type.typname as enum_type from pg_type join pg_enum on pg_enum.enumtypid = pg_type.oid"#;
        let rows = self.query(statement, &[]).await?;
        let mut enum_names = vec![];
        for row in rows {
            let enum_name = row.try_get::<&str, String>("enum_type")?;
            if !enum_name.starts_with("_") {
                enum_names.push(enum_name);
            }
        }
        Ok(enum_names)
    }

    fn enum_create_statement(&self, enum_def: &crate::migration::EnumDef) -> String {
        let variants: Vec<String> = enum_def.variants.iter().map(|v| format!("{}{}{}", Self::string_quote_char(), v, Self::string_quote_char())).collect();
        format!(r#"create type {}{}{} as enum({})"#,
            Self::ident_quote_char(),
            enum_def.name,
            Self::ident_quote_char(),
            variants.join(","))
    }

    fn enum_drop_statement(&self, enum_name: &str) -> String {
        format!(r#"drop type if exists {}{}{}"#, Self::ident_quote_char(), enum_name, Self::ident_quote_char())
    }

    fn add_enum_variant_statement(&self, enum_name: &str, variant_name: &str) -> String {
        format!(r#"alter type {}{}{} add value {}{}{}"#,
            Self::ident_quote_char(),
            enum_name,
            Self::ident_quote_char(),
            Self::string_quote_char(),
            variant_name,
            Self::string_quote_char())
    }

    async fn exist_enum_def(&mut self, enum_name: &'static str) -> Result<EnumDef, Self::Err> {
        let statement = format!("select pg_enum.enumlabel as variant from pg_type join pg_enum on pg_enum.enumtypid = pg_type.oid where pg_type.typname = '{}'", enum_name);
        let rows = self.query(&statement, &[]).await?;
        let mut variants = vec![];
        for row in rows {
            let variant: String = row.try_get("variant")?;
            variants.push(Cow::Owned(variant));
        }
        Ok(EnumDef {
            name: enum_name,
            variants,
        })
    }

    async fn exist_table_names(&mut self) -> Result<Vec<String>, Self::Err> {
        let statement = "select tablename from pg_catalog.pg_tables where schemaname != 'pg_catalog' and schemaname != 'information_schema'";
        let rows = self.query(
            statement,
            &[],
        ).await?;
        let mut table_names = vec![];
        for row in rows {
            let table_name = row.try_get::<&str, String>("tablename")?;
            table_names.push(table_name);
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
    fn defined_table_defs<S>(&self) -> Vec<TableDef<postgres::ColumnType>> where S: Schema {
        S::postgres_table_defs()
    }

    async fn exist_table_def(&mut self, table_name: &'static str) -> Result<TableDef<Self::ColumnType>, Self::Err> {
        let columns_statement = format!("select * from information_schema.columns where table_name = '{table_name}'");
        let column_rows = self.query(
            &columns_statement,
            &[]
        ).await?;
        let mut columns = vec![];
        for row in &column_rows {
            let name: String = row.try_get("column_name")?;
            let column_default: Option<String> = row.try_get("column_default")?;
            let is_nullable: String = row.try_get("is_nullable")?;
            let udt_name: String = row.try_get("udt_name")?;
            columns.push(ColumnDef {
                name: Cow::Owned(name),
                ty: postgres::ColumnType::from_str(&udt_name).unwrap(),
                nullable: is_nullable == "YES",
                default: column_default.map(Cow::Owned)
            });
        }
        let indexes_statement = format!("select * from pg_indexes where tablename = '{table_name}' and not indexname like '%_pkey'");
        let index_rows = self.query(
            &indexes_statement,
            &[]
        ).await?;
        let mut indexes = vec![];
        for index_row in &index_rows {
            let def: String = index_row.try_get("indexdef")?;
            let cols = if let Some(start) = def.find('(') && let Some(end) = def.find(')') {
                let cols_def = &def[start + 1..end];
                cols_def.split(",").map(|s| {
                    let trimmed = s.trim();
                    if trimmed.contains(" ") {
                        let mut name_and_order = trimmed.split(" ");
                        let name = name_and_order.nth(0).unwrap().trim();
                        let order = name_and_order.nth(1).unwrap().trim();
                        let sort_order = if order == "DESC" { SortOrder::Desc } else { SortOrder::Asc };
                        let clear_name = if name.starts_with("\"") && name.ends_with("\"") {
                            &name[1..name.len() - 1]
                        } else {
                            name
                        };
                        IndexColumnDef { name: Cow::Owned(clear_name.to_string()), order: sort_order }
                    } else {
                        if trimmed.starts_with("\"") && trimmed.ends_with("\"") {
                            IndexColumnDef {
                                name: Cow::Owned(trimmed[1..trimmed.len() - 1].to_string()),
                                order: SortOrder::Asc,
                            }
                        } else {
                            IndexColumnDef {
                                name: Cow::Owned(trimmed.to_string()),
                                order: SortOrder::Asc,
                            }
                        }
                    }
                }).collect()
            } else {
                vec![]
            };
            indexes.push(IndexDef {
                name: Cow::Owned(index_row.get("indexname")),
                columns: cols,
            });
        }
        Ok(TableDef {
            name: table_name,
            columns,
            indexes,
        })
    }
}
