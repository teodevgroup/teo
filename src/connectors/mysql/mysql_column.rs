use mysql_async::{Row, Value};
use crate::connectors::sql_shared::sql::{SQLColumnDef, SQLDialect};


#[derive(PartialEq)]
pub struct MySQLColumn {
    pub(crate) field: String,
    pub(crate) field_type: String,
    pub(crate) null: String,
    pub(crate) key: String,
    pub(crate) default: Option<String>,
    pub(crate) extra: String
}

trait ValueHelpers {
    fn to_string(&self) -> String;
}

impl ValueHelpers for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Bytes(val) => String::from_utf8(val.clone()).unwrap(),
            _ => panic!()
        }
    }
}

impl From<&Row> for MySQLColumn {
    fn from(row: &Row) -> Self {
        let field = (&row["Field"]).to_string();
        let field_type = (&row["Type"]).to_string();
        let null = (&row["Null"]).to_string();
        let key = (&row["Key"]).to_string();
        let default = None;
        let extra = (&row["Extra"]).to_string();
        MySQLColumn { field, field_type, null, key, default, extra }
    }
}

impl From<&SQLColumnDef> for MySQLColumn {
    fn from(def: &SQLColumnDef) -> Self {
        let field = def.name.clone();
        let field_type = def.column_type.to_string(SQLDialect::MySQL).to_lowercase();
        let null = if def.not_null { "NO" } else { "YES" }.to_string();
        let key = if def.primary_key { "PRI" } else { "" }.to_string();
        let default = None;
        let extra = if def.auto_increment { "auto_increment" } else { "" }.to_string();
        MySQLColumn { field, field_type, null, key, default, extra }
    }
}
