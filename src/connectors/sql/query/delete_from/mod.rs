use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::to_sql_string::ToSQLString;

pub struct SQLDeleteFromStatement<'a> {
    pub(crate) from: &'a str,
    pub(crate) r#where: Option<String>,
}

impl<'a> SQLDeleteFromStatement<'a> {

    pub fn r#where(&mut self, r#where: String) -> &mut Self {
        self.r#where = Some(r#where);
        self
    }
}

impl<'a> ToSQLString for SQLDeleteFromStatement<'a> {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let r#where = if let Some(r#where) = &self.r#where {
            " WHERE ".to_owned() + r#where
        } else {
            "".to_owned()
        };
        format!("DELETE FROM {}{}", self.from, r#where)
    }
}
