use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::to_sql_string::ToSQLString;

pub enum WhereClause {
    And(Vec<String>),
    Or(Vec<String>),
    Not(String),
}

pub trait ToWrappedSQLString {
    fn to_wrapped_string(&self, dialect: SQLDialect) -> String;
}

impl ToWrappedSQLString for WhereClause {
    fn to_wrapped_string(&self, dialect: SQLDialect) -> String {
        "(".to_owned() + &self.to_string(dialect) + ")"
    }
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
