use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SQLIndexType {
    Primary,
    Index,
    Unique
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SQLIndexOrdering {
    Asc,
    Desc
}

#[derive(PartialEq, Debug, Clone)]
pub struct SQLIndexColumn {
    pub(crate) name: String,
    pub(crate) ordering: SQLIndexOrdering,
    pub(crate) length: Option<u16>,
}

impl SQLIndexColumn {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), ordering: SQLIndexOrdering::Asc, length: None }
    }

    pub fn asc(&mut self) -> &mut Self {
        self.ordering = SQLIndexOrdering::Asc;
        self
    }

    pub fn desc(&mut self) -> &mut Self {
        self.ordering = SQLIndexOrdering::Desc;
        self
    }
}

impl ToSQLString for SQLIndexColumn {
    fn to_string(&self, _dialect: SQLDialect) -> String {
        let name = &self.name;
        let ordering = match self.ordering {
            SQLIndexOrdering::Asc => " ASC",
            SQLIndexOrdering::Desc => " DESC",
        };
        format!("{name}{ordering}")
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct SQLIndex {
    pub(crate) name: String,
    pub(crate) index_type: SQLIndexType,
    pub(crate) columns: Vec<SQLIndexColumn>
}
