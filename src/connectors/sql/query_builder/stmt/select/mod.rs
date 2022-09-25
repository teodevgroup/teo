use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;

pub mod r#where;

pub struct SQLSelectStatement<'a> {
    pub(crate) columns: Option<&'a Vec<&'a str>>,
    pub(crate) from: &'a str,
    pub(crate) r#where: Option<&'a Vec<(&'a str, &'a str, &'a str)>>,
}

impl<'a> SQLSelectStatement<'a> {

    pub fn r#where(&mut self, pairs: &'a Vec<(&'a str, &'a str, &'a str)>) -> &mut Self {
        self.r#where = Some(pairs);
        self
    }
}

impl<'a> ToSQLString for SQLSelectStatement<'a> {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let columns = if self.columns.is_none() { "*" } else { self.columns.unwrap().join(", ") };
        let r#where = if self.r#where.is_none() {
            "".to_owned()
        } else {
            "WHERE ".to_owned() + &*self.r#where.unwrap().iter().map(|p| format!("{} {} {}", p.0, p.1, p.2)).collect().join(", ")
        };
        format!("SELECT {columns} from {}{}", self.from, r#where)
    }
}
