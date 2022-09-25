use crate::connectors::sql::query_builder::stmt::create::table::SQLCreateTableStatement;
use crate::connectors::sql::query_builder::stmt::SQL;
use crate::core::model::Model;

impl From<&Model> for SQLCreateTableStatement {
    fn from(model: &Model) -> Self {
        let mut stmt = SQL::create().table(model.table_name());
        stmt.if_not_exists();
        for field in model.fields() {
            stmt.column(field.into());
        }
        stmt
    }
}
