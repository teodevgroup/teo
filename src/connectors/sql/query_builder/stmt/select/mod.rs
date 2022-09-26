use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;

pub mod r#where;

pub struct SQLSelectStatement<'a> {
    pub(crate) columns: Option<&'a Vec<&'a str>>,
    pub(crate) from: &'a str,
    pub(crate) r#where: Option<String>,
}

impl<'a> SQLSelectStatement<'a> {

    pub fn r#where(&mut self, r#where: String) -> &mut Self {
        self.r#where = Some(r#where);
        self
    }
}

impl<'a> ToSQLString for SQLSelectStatement<'a> {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let columns = if self.columns.is_none() { "*".to_owned() } else { self.columns.unwrap().join(", ") };
        let r#where = if let Some(r#where) = &self.r#where {
            " WHERE".to_owned() + r#where
        } else {
            "".to_owned()
        };
        format!("SELECT {columns} from {}{}", self.from, r#where)
    }
}
