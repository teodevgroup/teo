use std::fmt::{Debug, Write};
use std::rc::Rc;
use serde_json::{Value as JsonValue};
use async_trait::async_trait;
use sea_query::{ColumnDef, GenericBuilder, Iden, IntoTableRef, MysqlQueryBuilder, MySqlQueryBuilder, PostgresQueryBuilder, QueryBuilder, SchemaBuilder, SeaRc, Table, TableCreateStatement, TableRef};
use sqlx::{Pool, Database, MySqlPool, PgPool, Executor, MySql, Postgres};
use url::Url;
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::object::Object;
use crate::core::builders::graph_builder::GraphBuilder;
use crate::core::field::{Optionality};
use crate::core::field_type::FieldType;
use crate::core::model::Model;
use crate::error::ActionError;


pub(crate) fn table_create_statement(model: &Model) -> TableCreateStatement {
    let mut table_create_statement = Table::create();
    table_create_statement
        .table(SeaIden::new(model.table_name().to_string()))
        .if_not_exists();
    for field in model.fields() {
        let mut def = ColumnDef::new(SeaIden::new(field.name.to_string()));
        field.r#type.install_column_type(&mut def);
        match field.optionality {
            Optionality::Required => {
                def.not_null();
            }
            Optionality::Optional => {

            }
        }
        if field.primary {
            def.primary_key();
        }
        if field.auto_increment {
            def.auto_increment();
        }
        table_create_statement.col(&mut def);
    }
    table_create_statement
}

pub(crate) trait ColumnType {
    fn install_column_type<'a>(&self, def: &'a mut ColumnDef) -> &'a mut ColumnDef;
}

impl ColumnType for FieldType {
    fn install_column_type<'a>(&self, def: &'a mut ColumnDef) -> &'a mut ColumnDef {
        match self {
            FieldType::Undefined => {
                panic!("Column type is undefined.")
            }
            FieldType::ObjectId => {
                def.string()
            }
            FieldType::Bool => {
                def.boolean()
            }
            FieldType::I8 => {
                def.tiny_integer()
            }
            FieldType::I16 => {
                def.small_integer()
            }
            FieldType::I32 => {
                def.integer()
            }
            FieldType::I64 => {
                def.big_integer()
            }
            FieldType::I128 => {
                def.big_integer()
            }
            FieldType::U8 => {
                def.tiny_unsigned()
            }
            FieldType::U16 => {
                def.small_unsigned()
            }
            FieldType::U32 => {
                def.unsigned()
            }
            FieldType::U64 => {
                def.big_unsigned()
            }
            FieldType::U128 => {
                def.big_unsigned()
            }
            FieldType::F32 => {
                def.float()
            }
            FieldType::F64 => {
                def.double()
            }
            FieldType::String => {
                def.string()
            }
            FieldType::Date => {
                def.date()
            }
            FieldType::DateTime => {
                def.date_time()
            }
            FieldType::Enum(enum_name) => {
                def.string()
            }
            FieldType::Vec(_) => {
                def.string()
            }
            FieldType::Map(_) => {
                def.string()
            }
            FieldType::Object(_) => {
                def.string()
            }
        }
    }
}

pub(crate) struct SeaIden {
    name: String
}

impl SeaIden {
    pub(crate) fn new(name: String) -> SeaIden {
        SeaIden { name }
    }
}

impl Iden for SeaIden {
    fn prepare(&self, s: &mut dyn Write, q: char) {
        s.write_char(q);
        s.write_str(&self.name);
        s.write_char(q);
    }

    fn quoted(&self, q: char) -> String {
        q.to_string() + &self.name.to_string() + &q.to_string()
    }

    fn to_string(&self) -> String {
        self.name.to_string()
    }

    fn unquoted(&self, s: &mut dyn Write) {
        s.write_str(&self.name);
    }
}

unsafe impl Send for SeaIden {}
unsafe impl Sync for SeaIden {}
