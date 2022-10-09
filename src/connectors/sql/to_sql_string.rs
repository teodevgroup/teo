use crate::connectors::sql::dialect::SQLDialect;

pub trait ToSQLString {
    fn to_string(&self, dialect: SQLDialect) -> String;
}
