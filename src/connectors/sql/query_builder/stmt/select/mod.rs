use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;

pub mod r#where;

pub struct SQLSelectStatement<'a> {
    pub(crate) columns: Option<&'a Vec<&'a str>>,
    pub(crate) from: &'a str,
    pub(crate) r#where: Option<String>,
    pub(crate) order_by: Option<String>,
    pub(crate) limit: Option<(u64, u64)>,
}

impl<'a> SQLSelectStatement<'a> {

    pub fn r#where(&mut self, r#where: String) -> &mut Self {
        self.r#where = Some(r#where);
        self
    }

    pub fn order_by(&mut self, order_by: String) -> &mut Self {
        self.order_by = Some(order_by);
        self
    }

    pub fn limit(&mut self, limit: u64, skip: u64) -> &mut Self {
        self.limit = Some((limit, skip));
        self
    }
}

impl<'a> ToSQLString for SQLSelectStatement<'a> {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let columns = if self.columns.is_none() { "*".to_owned() } else { self.columns.unwrap().join(", ") };
        let r#where = if let Some(r#where) = &self.r#where {
            " WHERE ".to_owned() + r#where
        } else {
            "".to_owned()
        };
        let order_by = if let Some(order_by) = &self.order_by {
            " ORDER BY ".to_owned() + order_by
        } else {
            "".to_owned()
        };
        let limit = if let Some(limit) = &self.limit {
            format!(" LIMIT {},{}", limit.1, limit.0)
        } else {
            "".to_owned()
        };
        format!("SELECT {columns} from {}{}{}{}", self.from, r#where, order_by, limit)
    }
}
