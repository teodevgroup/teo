pub mod sql;

use std::fmt::{Debug, Write};
use std::rc::Rc;
use serde_json::{Value as JsonValue};
use async_trait::async_trait;
use url::Url;
use crate::connectors::sql_shared::sql::{SQL, SQLColumnDef, SQLCreateTableStatement};
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::object::Object;
use crate::core::builders::graph_builder::GraphBuilder;
use crate::core::field::{Optionality};
use crate::core::field_type::FieldType;
use crate::core::model::Model;
use crate::error::ActionError;


pub(crate) fn table_create_statement(model: &Model) -> SQLCreateTableStatement {
    let mut stmt = SQL::create().table(model.table_name().clone());
    stmt.if_not_exists();
    for field in model.fields() {
        let mut column = SQLColumnDef::new(field.column_name());
        column.column_type(field.database_type.clone());
        match field.optionality {
            Optionality::Required => {
                column.not_null();
            }
            Optionality::Optional => {}
        }
        if field.primary {
            column.primary_key();
        }
        if field.auto_increment {
            column.auto_increment();
        }
        stmt.column(column);
    }
    stmt
}

//
// impl ColumnType for FieldType {
//     fn install_column_type<'a>(&self, def: &'a mut ColumnDef) -> &'a mut ColumnDef {
//         match self {
//             FieldType::Undefined => {
//                 panic!("Column type is undefined.")
//             }
//             FieldType::ObjectId => {
//                 def.string()
//             }
//             FieldType::Bool => {
//                 def.boolean()
//             }
//             FieldType::I8 => {
//                 def.tiny_integer()
//             }
//             FieldType::I16 => {
//                 def.small_integer()
//             }
//             FieldType::I32 => {
//                 def.integer()
//             }
//             FieldType::I64 => {
//                 def.big_integer()
//             }
//             FieldType::I128 => {
//                 def.big_integer()
//             }
//             FieldType::U8 => {
//                 def.tiny_unsigned()
//             }
//             FieldType::U16 => {
//                 def.small_unsigned()
//             }
//             FieldType::U32 => {
//                 def.unsigned()
//             }
//             FieldType::U64 => {
//                 def.big_unsigned()
//             }
//             FieldType::U128 => {
//                 def.big_unsigned()
//             }
//             FieldType::F32 => {
//                 def.float()
//             }
//             FieldType::F64 => {
//                 def.double()
//             }
//             FieldType::String => {
//                 def.string()
//             }
//             FieldType::Date => {
//                 def.date()
//             }
//             FieldType::DateTime => {
//                 def.date_time()
//             }
//             FieldType::Enum(enum_name) => {
//                 def.string()
//             }
//             FieldType::Vec(_) => {
//                 def.string()
//             }
//             FieldType::Map(_) => {
//                 def.string()
//             }
//             FieldType::Object(_) => {
//                 def.string()
//             }
//         }
//     }
// }
