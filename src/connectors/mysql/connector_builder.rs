use url::Url;
use async_trait::async_trait;
use mysql_async::Pool;
use mysql_async::prelude::{Query};
use crate::connectors::mysql::connector::MySQLConnector;
use crate::connectors::sql_shared::sql::{SQL, SQLDialect, ToSQLString};
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::database_type::DatabaseType;
use crate::core::field_type::FieldType;
use crate::core::model::Model;


#[derive(Debug)]
pub(crate) struct MySQLConnectorBuilder {
    url: String
}

impl MySQLConnectorBuilder {
    pub(crate) fn new(url: String) -> MySQLConnectorBuilder {
        MySQLConnectorBuilder { url }
    }


}

#[async_trait]
impl ConnectorBuilder for MySQLConnectorBuilder {
    fn inferred_database_type(&self, field_type: &FieldType) -> DatabaseType {
        match field_type {
            FieldType::Undefined => DatabaseType::Undefined,
            FieldType::ObjectId => DatabaseType::Undefined,
            FieldType::Bool => DatabaseType::Bool,
            FieldType::I8 => DatabaseType::TinyInt(false),
            FieldType::I16 => DatabaseType::SmallInt(false),
            FieldType::I32 => DatabaseType::Int(false),
            FieldType::I64 => DatabaseType::BigInt(false),
            FieldType::I128 => DatabaseType::BigInt(false),
            FieldType::U8 => DatabaseType::TinyInt(true),
            FieldType::U16 => DatabaseType::SmallInt(true),
            FieldType::U32 => DatabaseType::Int(true),
            FieldType::U64 => DatabaseType::BigInt(true),
            FieldType::U128 => DatabaseType::BigInt(true),
            FieldType::F32 => DatabaseType::Real,
            FieldType::F64 => DatabaseType::Double,
            FieldType::String => DatabaseType::VarChar(191, None, None),
            FieldType::Date => DatabaseType::Date,
            FieldType::DateTime => DatabaseType::DateTime(3),
            FieldType::Enum(_) => DatabaseType::Undefined,
            FieldType::Vec(_) => DatabaseType::Undefined,
            FieldType::Map(_) => DatabaseType::Undefined,
            FieldType::Object(_) => DatabaseType::Undefined,
        }
    }

    async fn build_connector(&self, models: &Vec<Model>, reset_database: bool) -> Box<dyn Connector> {
        let url_result = Url::parse(&self.url);
        if url_result.is_err() {
            panic!("MySQL URL is invalid.");
        }
        let mut url = url_result.unwrap();
        let database_name = url.path()[1..].to_string();
        url.set_path("/");
        let pool = Pool::new(url.as_str());
        let mut conn = pool.get_conn().await.unwrap();
        // drop database if needed
        if reset_database {
            let stmt = SQL::drop().database(&database_name).
                if_exists().to_string(SQLDialect::MySQL);
            stmt.ignore(&mut conn).await.unwrap();
        }
        // use database
        let stmt = SQL::create().database(&database_name).if_not_exists().to_string(SQLDialect::MySQL);
        stmt.ignore(&mut conn).await.unwrap();
        let stmt = SQL::r#use().database(&database_name).to_string(SQLDialect::MySQL);
        stmt.ignore(&mut conn).await.unwrap();
        Box::new(MySQLConnector::new(pool, database_name.clone(), models).await)
    }
}
