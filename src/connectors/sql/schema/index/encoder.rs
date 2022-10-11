use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::value::encode::ToSQLString;
use crate::core::field::Sort;
use crate::core::model::index::ModelIndexItem;

impl ToSQLString for ModelIndexItem {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let name = self.field_name();
        let ordering = match self.sort() {
            Sort::Asc => " ASC",
            Sort::Desc => " DESC",
        };
        format!("{name}{ordering}")
    }
}
