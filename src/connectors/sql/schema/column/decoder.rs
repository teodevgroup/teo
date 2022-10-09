
use std::sync::Arc;
use sqlx::any::AnyRow;
use sqlx::Row;
use crate::connectors::sql::column::SQLColumn;
use crate::connectors::sql::dialect::SQLDialect;
use crate::connectors::sql::query_builder::structs::column::SQLColumn;
use crate::connectors::sql::to_sql_string::ToSQLString;
use crate::core::field::Field;
use crate::core::property::Property;

pub(crate) struct ColumnDecoder { }

impl ColumnDecoder {
    fn decode(row: AnyRow, dialect: SQLDialect) -> SQLColumn {
        let field: String = row.get("Field");
        let field_type_in_string: String = row.get("Type");
        let null_in_string: String = row.get("Null");
        let null = &null_in_string == "YES";
        let key: String = row.get("Key");
        let extra: String = row.get("Extra");
        let auto_increment = extra.contains("auto_increment");
        let primary = &key == "PRI";
        let unique = extra.contains("unique");
        SQLColumn {
            name: field,

            not_null: !null,
            auto_increment,
            default: None,
            primary_key: primary,
            unique_key: unique,
        }
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
        SQLColumn::from(field.as_ref())
    }
}

impl From<&Property> for SQLColumn {
    fn from(property: &Property) -> Self {
        let mut column = SQLColumn::new(&property.name);
        column.column_type(property.database_type.clone());
        if property.is_required() {
            column.not_null();
        }
        column
    }
}

impl From<&Arc<Property>> for SQLColumn {
    fn from(property: &Arc<Property>) -> Self {
        SQLColumn::from(property.as_ref())
    }
}
