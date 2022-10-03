use std::sync::Arc;
use sqlx::any::AnyRow;
use sqlx::Row;
use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::structs::column::SQLColumn;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;
use crate::core::field::Field;

#[derive(PartialEq, Debug)]
pub struct MySQLColumn {
    pub(crate) field: String,
    pub(crate) field_type: String,
    pub(crate) null: String,
    pub(crate) key: String,
    pub(crate) default: Option<String>,
    pub(crate) extra: String
}

pub trait ValueHelpers {
    fn to_string(&self) -> String;
    fn to_i64(&self) -> i64;
    fn to_u64(&self) -> u64;
}

impl From<AnyRow> for MySQLColumn {
    fn from(row: AnyRow) -> Self {
        let field: String = row.get("Field");
        let field_type: String = row.get("Type");
        let null = row.get("Null");
        let key = row.get("Key");
        let default = None;
        let extra = row.get("Extra");
        MySQLColumn { field, field_type, null, key, default, extra }
    }
}

impl From<&SQLColumn> for MySQLColumn {
    fn from(def: &SQLColumn) -> Self {
        let field = def.name.clone();
        let field_type = def.column_type.to_string(SQLDialect::MySQL).to_lowercase();
        let null = if def.not_null { "NO" } else { "YES" }.to_string();
        let key = if def.primary_key { "PRI" } else { "" }.to_string();
        let default = None;
        let extra = if def.auto_increment { "auto_increment" } else { "" }.to_string();
        MySQLColumn { field, field_type, null, key, default, extra }
    }
}

impl From<&Field> for SQLColumn {
    fn from(field: &Field) -> Self {
        let mut column = SQLColumn::new(field.column_name());
        column.column_type(field.database_type.clone());
        if field.is_required() {
            column.not_null();
        }
        if field.primary {
            column.primary_key();
        }
        if field.auto_increment {
            column.auto_increment();
        }
        column
    }
}

impl From<&Arc<Field>> for SQLColumn {
    fn from(field: &Arc<Field>) -> Self {
        let mut column = SQLColumn::new(field.column_name());
        column.column_type(field.database_type.clone());
        if field.is_required() {
            column.not_null();
        }
        if field.primary {
            column.primary_key();
        }
        if field.auto_increment {
            column.auto_increment();
        }
        column
    }
}
