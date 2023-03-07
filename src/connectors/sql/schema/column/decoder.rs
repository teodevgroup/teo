use std::collections::{BTreeSet, HashSet};
use std::sync::Arc;
use maplit::{btreeset, hashset};
use quaint::prelude::{ResultRow, ResultSet};
use crate::connectors::sql::schema::column::SQLColumn;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::r#type::decoder::SQLTypeDecoder;
use crate::core::field::Field;
use crate::core::model::Model;
use crate::core::property::Property;

pub(crate) struct ColumnDecoder { }

impl ColumnDecoder {

    pub(crate) fn sqlite_add_and_remove<'a>(db: &'a HashSet<SQLColumn>, def: &'a HashSet<SQLColumn>) -> (Vec<&'a SQLColumn>, Vec<&'a SQLColumn>) {
        let mut to_add: Vec<&SQLColumn> = def.iter().collect();
        let mut to_remove: Vec<&SQLColumn> = vec![];
        for c in db {
            if !defs.contains(&c) {
                to_remove.push(c);
            } else {
                // remove from to add
                // to_add.in
            }
        }
        (to_add, to_remove)
    }

    pub(crate) fn need_to_alter_any_columns(db: &HashSet<SQLColumn>, def: &HashSet<SQLColumn>) -> bool {
        for column in db {
            if let Some(def_column) = def.iter().find(|c| { &c.name == &column.name}) {
                if def_column != column {
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn decode_model_columns(model: &Model) -> HashSet<SQLColumn> {
        let mut result = hashset!{};
        for field in model.fields() {
            result.insert(field.into());
        }
        for property in model.properties() {
            if property.cached {
                result.insert(property.into());
            }
        }
        result
    }

    pub(crate) fn decode_sqlite_columns(columns: ResultSet, indices: ResultSet, auto_increment: ResultSet) -> HashSet<SQLColumn> {
        let mut indices_iter = indices.into_iter();
        let mut result = hashset!{};
        for column in columns {
            let name = column.get("name").unwrap().as_str().unwrap();
            let r#type = column.get("type").unwrap().as_str().unwrap();
            let not_null = column.get("notnull").unwrap().as_bool().unwrap();
            let pk = column.get("pk").unwrap().as_bool().unwrap();
            let unique_row = indices_iter.find(|i| i.get("column_name").unwrap().as_str().unwrap() == name);
            let unique_key = if unique_row.is_some() {
                unique_row.unwrap().get("unique").unwrap().as_bool().unwrap()
            } else {
                false
            };
            result.insert(SQLColumn {
                name: name.to_string(),
                r#type: SQLTypeDecoder::decode(&r#type, SQLDialect::SQLite),
                not_null,
                auto_increment: pk && !auto_increment.is_empty(),
                default: None,
                primary_key: pk,
                unique_key,
            });
        }
        result
    }

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
