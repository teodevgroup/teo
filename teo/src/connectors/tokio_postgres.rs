use std::borrow::Cow;

use teo_column_type::postgres;
use tokio_postgres::{Client, Error};
use crate::{connection::AsyncConnection, migration::{AsyncMigration, EnumDef}, types::Schema};

impl AsyncConnection for Client {

    type Err = Error;

    async fn migrate<S>(&self) -> Result<(), Self::Err> where S: Schema {
        AsyncMigration::migrate::<S>(self).await
    }
}

impl AsyncMigration for Client {

    type Err = Error;

    type ColumnType = postgres::ColumnType;

    #[inline]
    async fn execute_without_params(&self, q: &str) -> Result<(), Self::Err> {
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

    async fn exist_enum_names(&self) -> Result<Vec<String>, Self::Err> {
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

    async fn exist_enum_def(&self, enum_name: &'static str) -> Result<EnumDef, Self::Err> {
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

    async fn exist_table_names(&self) -> Result<Vec<String>, Self::Err> {
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

    #[inline]
    fn defined_table_defs<S>(&self) -> Vec<crate::migration::TableDef<postgres::ColumnType>> where S: Schema {
        S::postgres_table_defs()
    }
}
