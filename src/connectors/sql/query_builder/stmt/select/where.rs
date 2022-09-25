use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;

pub enum WhereClause {
    And(Vec<String>),
    Or(Vec<String>),
    Not(String),
}

impl ToSQLString for WhereClause {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        match self {
            WhereClause::And(items) => items.join(" AND "),
            WhereClause::Or(items) => items.join(" OR "),
            WhereClause::Not(item) => format!("NOT {item}"),
        }
    }
}

pub struct WhereItem<'a>(&'a str, &'a str, &'a str);

impl<'a> ToSQLString for WhereItem<'a> {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        format!("{} {} {}", self.0, self.1, self.2)
    }
}
