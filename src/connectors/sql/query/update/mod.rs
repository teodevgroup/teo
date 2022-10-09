use crate::connectors::sql::dialect::SQLDialect;
use crate::connectors::sql::to_sql_string::ToSQLString;

pub struct SQLUpdateStatement<'a> {
    pub(crate) table: &'a str,
    pub(crate) values: Vec<(&'a str, &'a str)>,
    pub(crate) r#where: &'a str,
}

impl<'a> SQLUpdateStatement<'a> {
    pub fn value(&mut self, pair: (&'a str, &'a str)) -> &mut Self {
        self.values.push(pair);
        self
    }

    pub fn values(&mut self, pairs: Vec<(&'a str, &'a str)>) -> &mut Self {
        self.values.extend(pairs);
        self
    }

    pub fn r#where(&mut self, r#where: &'a str) -> &mut Self {
        self.r#where = r#where;
        self
    }
}

impl<'a> ToSQLString for SQLUpdateStatement<'a> {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let mut exprs: Vec<String> = vec![];
        for (k, v) in self.values.iter() {
            exprs.push(format!("{} = {}", k, v));
        }
        let r#where = if self.r#where.is_empty() {
            "".to_owned()
        } else {
            " WHERE ".to_owned() + self.r#where
        };
        format!("UPDATE `{}` SET {}{};", self.table, exprs.join(","), r#where)
    }
}
