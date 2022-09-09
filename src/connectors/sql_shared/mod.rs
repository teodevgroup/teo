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
use crate::core::field::r#type::FieldType;
use crate::core::model::Model;
use crate::core::error::ActionError;

pub(crate) fn table_create_statement(model: &Model) -> SQLCreateTableStatement {
    let mut stmt = SQL::create().table(model.table_name());
    stmt.if_not_exists();
    for field in model.fields() {
        stmt.column(field.into());
    }
    stmt
}
