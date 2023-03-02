use std::sync::Arc;
use quaint::prelude::ResultRow;
use crate::connectors::sql::schema::column::SQLColumn;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::r#type::decoder::SQLTypeDecoder;
use crate::core::field::Field;
use crate::core::property::Property;

pub(crate) struct ColumnDecoder { }

impl ColumnDecoder {
    pub(crate) fn decode(row: ResultRow, dialect: SQLDialect) -> SQLColumn {
        let field: String = row.get("Field").unwrap().to_string().unwrap();
        let field_type_in_string: String = row.get("Type").unwrap().to_string().unwrap();
        let null_in_string: String = row.get("Null").unwrap().to_string().unwrap();
        let null = &null_in_string == "YES";
        let key: String = row.get("Key").unwrap().to_string().unwrap();
        let extra: String = row.get("Extra").unwrap().to_string().unwrap();
        let auto_increment = extra.contains("auto_increment");
        let primary = &key == "PRI";
        let unique = &key == "UNI";
        SQLColumn {
            name: field,
            r#type: SQLTypeDecoder::decode(&field_type_in_string, dialect),
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
        SQLColumn::new(field.column_name().to_owned(), field.database_type().clone(), field.is_required(), field.auto_increment, None, field.primary, field.index.is_some() && field.index.as_ref().unwrap().is_unique())
    }
}

impl From<&Arc<Field>> for SQLColumn {
    fn from(field: &Arc<Field>) -> Self {
        SQLColumn::from(field.as_ref())
    }
}

impl From<&Property> for SQLColumn {
    fn from(property: &Property) -> Self {
        SQLColumn::new(property.name.clone(), property.database_type().clone(), property.is_required(), false, None, false, false)
    }
}

impl From<&Arc<Property>> for SQLColumn {
    fn from(property: &Arc<Property>) -> Self {
        SQLColumn::from(property.as_ref())
    }
}
