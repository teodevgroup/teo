use crate::connectors::sql::to_sql_string::ToSQLString;

pub struct SQLInsertIntoStatement<'a> {
    pub(crate) table: &'a str,
    pub(crate) values: Vec<(&'a str, &'a str)>,
}

impl<'a> SQLInsertIntoStatement<'a> {
    pub fn value(&mut self, pair: (&'a str, &'a str)) -> &mut Self {
        self.values.push(pair);
        self
    }

    pub fn values(&mut self, pairs: Vec<(&'a str, &'a str)>) -> &mut Self {
        self.values.extend(pairs);
        self
    }
}

impl<'a> ToSQLString for SQLInsertIntoStatement<'a> {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let mut keys: Vec<&str> = vec![];
        let mut values: Vec<&str> = vec![];
        for (k, v) in self.values.iter() {
            keys.push(k);
            values.push(v);
        }
        format!("INSERT INTO `{}`({}) VALUES({});", self.table, keys.join(","), values.join(","))
    }
}
