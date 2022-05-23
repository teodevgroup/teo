use std::borrow::BorrowMut;
use std::fmt::{Debug, Formatter, Write};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use serde_json::{Value as JsonValue};
use async_trait::async_trait;
use sea_query::{ColumnDef, Iden, IntoTableRef, MysqlQueryBuilder, SeaRc, Table, TableRef};
use sqlx::{Pool, Database};
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::graph::{Graph};
use crate::core::object::Object;
use crate::core::builders::GraphBuilder;
use crate::core::field::{Availability, Type};
use crate::core::model::Model;
use crate::error::ActionError;


#[derive(Debug)]
pub struct SqlxConnector<DB> where DB: Database {
    pool: Pool<DB>
}

impl<DB> SqlxConnector<DB> where DB: Database {
    pub(crate) async fn new(pool: Pool<DB>, models: &Vec<Model>) -> SqlxConnector<DB> {
        for model in models {
            let mut table_create_statement = Table::create();
            table_create_statement
                .table(SeaIden::new(model.table_name().to_string()))
                .if_not_exists();
            for field in model.fields() {
                let mut def = ColumnDef::new(SeaIden::new(field.name.to_string()));
                field.r#type.install_column_type(&mut def);
                match field.availability {
                    Availability::Required => {
                        def.not_null();
                    }
                    Availability::Optional => {

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
            println!("{}", table_create_statement.to_string(MysqlQueryBuilder))
        }
        SqlxConnector { pool }
    }
}

#[async_trait]
impl<DB> Connector for SqlxConnector<DB> where DB: Database {

    async fn drop_database(&self) {

    }

    async fn save_object(&self, object: &Object) -> Result<(), ActionError> {
        Ok(())
    }

    async fn delete_object(&self, object: &Object) {
        todo!()
    }

    async fn find_unique(&self, model: &Model, finder: JsonValue) -> Option<Object> {
        todo!()
    }

    async fn find_one(&self, model: &Model, finder: JsonValue) -> Option<Object> {
        todo!()
    }

    async fn find_many(&self, model: &Model, finder: JsonValue) -> Vec<Object> {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct SqlxConnectorBuilder<DB> where DB: Database {
    pool: Pool<DB>
}

impl<DB> SqlxConnectorBuilder<DB> where DB: Database {
    pub(crate) fn new(pool: Pool<DB>) -> SqlxConnectorBuilder<DB> {
        SqlxConnectorBuilder { pool }
    }
}

#[async_trait]
impl<DB> ConnectorBuilder for SqlxConnectorBuilder<DB> where DB: Database {
    async fn build_connector(&self, models: &Vec<Model>) -> Box<dyn Connector> {
        Box::new(SqlxConnector::new(self.pool.clone(), models).await)
    }
}

pub trait SqlxConnectorHelpers {
    fn sqlx<DB>(&mut self, pool: Pool<DB>) where DB: Database;
}

impl SqlxConnectorHelpers for GraphBuilder {

    fn sqlx<DB>(&mut self, pool: Pool<DB>) where DB: Database {
        self.connector_builder = Some(Box::new(SqlxConnectorBuilder::new(pool)))
    }
}

unsafe impl<DB> Sync for SqlxConnector<DB> where DB: Database {}

pub(crate) trait ColumnType {
    fn install_column_type<'a>(&self, def: &'a mut ColumnDef) -> &'a mut ColumnDef;
}

impl ColumnType for Type {
    fn install_column_type<'a>(&self, def: &'a mut ColumnDef) -> &'a mut ColumnDef {
        match self {
            Type::Undefined => {
                panic!("Column type is undefined.")
            }
            Type::ObjectId => {
                def.string()
            }
            Type::Bool => {
                def.boolean()
            }
            Type::I8 => {
                def.tiny_integer()
            }
            Type::I16 => {
                def.small_integer()
            }
            Type::I32 => {
                def.integer()
            }
            Type::I64 => {
                def.big_integer()
            }
            Type::I128 => {
                def.big_integer()
            }
            Type::U8 => {
                def.tiny_unsigned()
            }
            Type::U16 => {
                def.small_unsigned()
            }
            Type::U32 => {
                def.unsigned()
            }
            Type::U64 => {
                def.big_unsigned()
            }
            Type::U128 => {
                def.big_unsigned()
            }
            Type::F32 => {
                def.float()
            }
            Type::F64 => {
                def.double()
            }
            Type::String => {
                def.string()
            }
            Type::Date => {
                def.date()
            }
            Type::DateTime => {
                def.date_time()
            }
            Type::Enum(enum_name) => {
                def.string()
            }
            Type::Vec(_) => {
                def.string()
            }
            Type::Map(_) => {
                def.string()
            }
            Type::Object(_) => {
                def.string()
            }
        }
    }
}

struct SeaIden {
    name: String
}

impl SeaIden {
    fn new(name: String) -> SeaIden {
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

// impl IntoTableRef for SeaIden {
//     fn into_table_ref(self) -> TableRef {
//         TableRef::Table(SeaRc::new(self))
//     }
// }
