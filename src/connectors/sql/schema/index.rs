// impl ToSQLString for SQLIndexColumn {
//     fn to_string(&self, _dialect: SQLDialect) -> String {
//         let name = &self.name;
//         let ordering = match self.ordering {
//             SQLIndexOrdering::Asc => " ASC",
//             SQLIndexOrdering::Desc => " DESC",
//         };
//         format!("{name}{ordering}")
//     }
// }
