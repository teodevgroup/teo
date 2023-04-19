use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::value::encode::ToSQLString;

pub(crate) struct SQLInsertIntoStatement<'a> {
    pub(crate) table: &'a str,
    pub(crate) values: Vec<(&'a str, &'a str)>,
    pub(crate) returning: Vec<String>,
}

impl<'a> SQLInsertIntoStatement<'a> {
    pub(crate) fn value(&mut self, pair: (&'a str, &'a str)) -> &mut Self {
        self.values.push(pair);
        self
    }

    pub(crate) fn values(&mut self, pairs: Vec<(&'a str, &'a str)>) -> &mut Self {
        self.values.extend(pairs);
        self
    }

    pub(crate) fn returning<S>(&mut self, keys: &Vec<S>) -> &mut Self where S: Clone + Into<String> {
        self.returning = keys.iter().map(|k| k.clone().into()).collect();
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
            format!("INSERT INTO `{}`({}) VALUES({});", self.table, keys.iter().map(|k| format!("`{k}`")).collect::<Vec<String>>().join(","), values.join(","))
        }
    }
}
