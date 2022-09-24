use crate::connectors::sql::query_builder::dialect::SQLDialect;

pub trait ToSQLString {
    fn to_string(&self, dialect: SQLDialect) -> String;
}
