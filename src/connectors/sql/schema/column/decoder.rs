use std::collections::{HashSet};
use std::sync::Arc;
use itertools::Itertools;
use maplit::{hashset};
use quaint::pooled::PooledConnection;
use quaint::prelude::{Query, Queryable, ResultRow, ResultSet};
use crate::connectors::sql::migration::sql::psql_is_auto_increment;
use crate::connectors::sql::schema::column::SQLColumn;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::r#type::decoder::SQLTypeDecoder;
use crate::core::field::Field;
use crate::core::model::Model;
use crate::core::pipeline::Pipeline;
use crate::core::property::Property;

pub(crate) enum ColumnManipulation<'a> {
    AddColumn(&'a SQLColumn, Option<Pipeline>),
    RemoveColumn(String, Option<Pipeline>),
    RenameColumn{ old: String, new: String },
    AlterColumn(&'a SQLColumn, Option<Pipeline>),
}

impl<'a> ColumnManipulation<'a> {

    pub(crate) fn get_field(&'a self, model: &'a Model) -> Option<&Field> {
        match self {
            ColumnManipulation::AddColumn(c, _) => model.field(c.name()),
            ColumnManipulation::RemoveColumn(c, _) => model.dropped_field(c.as_str()),
            ColumnManipulation::RenameColumn {old: _, new} => model.field(new.as_str()),
            ColumnManipulation::AlterColumn(c, _) => model.field(c.name()),
        }
    }

    pub(crate) fn priority(&self, model: &Model) -> usize {
        self.get_field(model).map(|f| f.migration().map(|m| m.priority.unwrap_or(0))).unwrap_or(Some(0)).unwrap_or(0)
    }
}

pub(crate) struct ColumnDecoder { }

impl ColumnDecoder {

    pub(crate) fn manipulations<'a>(db: &'a HashSet<SQLColumn>, def: &'a HashSet<SQLColumn>, model: &Model) -> Vec<ColumnManipulation<'a>> {
        let mut to_add: Vec<&SQLColumn> = def.iter().collect();
        let mut to_remove: Vec<&SQLColumn> = vec![];
        let mut to_rename: Vec<(String, String)> = vec![];
        let mut to_alter: Vec<&SQLColumn> = vec![];
        // analyse add and remove
        for c in db {
            if let Some(dc) = def.iter().find(|dc| dc.name() == c.name()) {
                // remove from to add
                let index = to_add.iter().position(|x| x.name() == c.name()).unwrap();
                to_add.remove(index);
                // maybe alter
                if c != dc {
                    to_alter.push(dc);
                }
            } else {
                to_remove.push(c);
            }
        }
        // analyse rename
        for c in to_add.clone() {
            if let Some(field) = model.field(c.name()) {
                if let Some(migration) = field.migration() {
                    for name in &migration.renamed {
                        if let Some((remove_index, remove_column)) = to_remove.clone().iter().find_position(|c| c.name() == name.as_str()) {
                            to_remove.remove(remove_index);
                            to_rename.push((remove_column.name().to_owned(), c.name().to_owned()));
                            let to_add_index = to_add.iter().position(|i| *i == c).unwrap();
                            to_add.remove(to_add_index);
                        }

                    }
                }
            }
            // TODO: for cached property, too
        }
        // collect
        let mut result = vec![];
        for c in to_add {
            let action = if let Some(field) = model.field(c.name()) {
                field.migration().map(|m| m.action.clone()).flatten()
            } else { None };
            result.push(ColumnManipulation::AddColumn(c, action));
        }
        for c in to_remove {
            let action = if let Some(field) = model.dropped_field(c.name()) {
                field.migration().map(|m| m.action.clone()).flatten()
            } else { None };
            result.push(ColumnManipulation::RemoveColumn(c.name().to_owned(), action));
        }
        for c in to_alter {
            let action = if let Some(field) = model.field(c.name()) {
                field.migration().map(|m| m.action.clone()).flatten()
            } else { None };
            result.push(ColumnManipulation::AlterColumn(c, action));
        }
        for c in to_rename {
            result.push(ColumnManipulation::RenameColumn { old: c.0, new: c.1 })
        }
        // sort
        result.sort_by(|a, b| {
            a.priority(model).cmp(&b.priority(model))
        });
        result
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
        let indices_iter: Vec<ResultRow> = indices.into_iter().collect();
        let mut result = hashset!{};
        for column in columns {
            let name = column.get("name").unwrap().as_str().unwrap();
            let r#type = column.get("type").unwrap().as_str().unwrap();
            let not_null = column.get("notnull").unwrap().as_bool().unwrap();
            let pk = column.get("pk").unwrap().as_bool().unwrap();
            let unique_row = indices_iter.iter().find(|i| {
                i.get("column_name").unwrap().as_str().unwrap() == name
            });
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

    async fn psql_primary_field_name(conn: &PooledConnection, table_name: &str) -> Vec<String> {
        let sql = format!("SELECT a.attname
FROM   pg_index i
JOIN   pg_attribute a ON a.attrelid = i.indrelid
                     AND a.attnum = ANY(i.indkey)
WHERE  i.indrelid = '{}'::regclass
AND    i.indisprimary", table_name);
        let result = conn.query(Query::from(sql)).await.unwrap();
        result.into_iter().map(|r| {
            r.get("attname").unwrap().to_string().unwrap()
        }).collect()
    }

    async fn psql_is_unique(conn: &PooledConnection, table_name: &str, column_name: &str) -> bool {
        let sql = format!("SELECT *
            FROM INFORMATION_SCHEMA.TABLE_CONSTRAINTS tc
        inner join INFORMATION_SCHEMA.CONSTRAINT_COLUMN_USAGE cu
        on cu.CONSTRAINT_NAME = tc.CONSTRAINT_NAME
        where
        tc.CONSTRAINT_TYPE = 'UNIQUE'
        and tc.TABLE_NAME = '{}'
        and cu.COLUMN_NAME = '{}'", table_name, column_name);
        !conn.query(Query::from(sql)).await.unwrap().is_empty()
    }

    async fn psql_is_auto_increment(conn: &PooledConnection, table_name: &str, column_name: &str) -> bool {
        !conn.query(Query::from(psql_is_auto_increment(table_name, column_name))).await.unwrap().is_empty()
    }

    pub(crate) async fn decode(row: ResultRow, dialect: SQLDialect, conn: &PooledConnection, table_name: &str) -> SQLColumn {
        if dialect == SQLDialect::MySQL {
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
        } else if dialect == SQLDialect::PostgreSQL { // postgres
            let primary_names = Self::psql_primary_field_name(conn, table_name).await;
            let column_name: String = row.get("column_name").unwrap().to_string().unwrap();
            let nullable_text: String = row.get("is_nullable").unwrap().to_string().unwrap();
            let nullable: bool = nullable_text == "YES";
            let data_type: String = row.get("data_type").unwrap().to_string().unwrap();
            SQLColumn {
                name: column_name.clone(),
                r#type: SQLTypeDecoder::decode(&data_type, dialect),
                not_null: !nullable,
                default: None,
                primary_key: primary_names.contains(&column_name),
                unique_key: Self::psql_is_unique(conn, table_name, &column_name).await,
                auto_increment: Self::psql_is_auto_increment(conn, table_name, &column_name).await,
            }
        } else {
            unreachable!()
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
