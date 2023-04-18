use crate::connectors::sql::stmts::create::table::SQLCreateTableStatement;
use crate::connectors::sql::stmts::SQL;
use crate::core::model::model::Model;

impl From<&Model> for SQLCreateTableStatement {
    fn from(model: &Model) -> Self {
        let mut stmt = SQL::create().table(model.table_name());
        stmt.if_not_exists();
        for field in model.fields() {
            stmt.column(field.into());
        }
        for property in model.properties() {
            if property.cached {
                stmt.column(property.into());
            }
        }
        if model.primary_field_names().len() > 1 {
            stmt.primary(model.primary_index().clone());
        }
        stmt
    }
}
