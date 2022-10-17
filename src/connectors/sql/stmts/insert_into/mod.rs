use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::value::encode::ToSQLString;

pub struct SQLInsertIntoStatement<'a> {
    pub(crate) table: &'a str,
    pub(crate) values: Vec<(&'a str, &'a str)>,
    pub(crate) returning: Vec<String>,
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

    pub fn returning(&mut self, keys: &Vec<String>) -> &mut Self {
        self.returning = keys.clone();
        self
    }
}

impl<'a> ToSQLString for SQLInsertIntoStatement<'a> {
    fn to_string(&self, dialect: SQLDialect) -> String {
        let mut keys: Vec<&str> = vec![];
        let mut values: Vec<&str> = vec![];
        for (k, v) in self.values.iter() {
            keys.push(k);
            values.push(v);
        }
        if dialect == SQLDialect::PostgreSQL {
            format!("INSERT INTO {}({}) VALUES({}){};", self.table, keys.iter().map(|k| format!("\"{}\"", k)).collect::<Vec<String>>().join(","), values.join(","), if self.returning.is_empty() {
                "".to_owned()
            } else {
                "  RETURNING ".to_owned() + &self.returning.join(",")
            })
        } else {
            format!("INSERT INTO `{}`({}) VALUES({});", self.table, keys.join(","), values.join(","))
        }
    }
}
