use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::model::index::ModelIndexItem;

pub(crate) mod decoder;

// impl ToSQLString for ModelIndexItem {
//     fn to_string(&self, _dialect: SQLDialect) -> String {
//         let name = self.field_name();
//         let ordering = match self.ordering {
//             SQLIndexOrdering::Asc => " ASC",
//             SQLIndexOrdering::Desc => " DESC",
//         };
//         format!("{name}{ordering}")
//     }
// }
