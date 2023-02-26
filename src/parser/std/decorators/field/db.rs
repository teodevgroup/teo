use maplit::hashmap;
use crate::core::database::name::DatabaseName;
use crate::parser::ast::accessible::Container;

pub(crate) fn db_container(database_name: DatabaseName) -> Container {
    match database_name {
        DatabaseName::MySQL => {
            Container { objects: hashmap!{} }
        }
        DatabaseName::PostgreSQL => {
            Container { objects: hashmap!{} }
        }
        #[cfg(feature = "data-source-sqlite")]
        DatabaseName::SQLite => {
            Container { objects: hashmap!{} }
        }
        DatabaseName::MongoDB => {
            Container { objects: hashmap!{} }
        }
    }
}
