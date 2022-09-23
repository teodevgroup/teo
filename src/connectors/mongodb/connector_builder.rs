use async_trait::async_trait;
use crate::connectors::mongodb::connector::MongoDBConnector;
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::db_type::DatabaseType;
use crate::core::field::r#type::FieldType;
use crate::core::model::Model;


#[derive(Debug)]
pub(crate) struct MongoDBConnectorBuilder {
    url: String
}

impl MongoDBConnectorBuilder {
    pub(crate) fn new(url: String) -> MongoDBConnectorBuilder {
        MongoDBConnectorBuilder { url }
    }
}

#[async_trait]
impl ConnectorBuilder for MongoDBConnectorBuilder {
    fn inferred_database_type(&self, field_type: &FieldType) -> DatabaseType {
        match field_type {
            FieldType::Undefined => DatabaseType::Undefined,
            FieldType::ObjectId => DatabaseType::ObjectId,
            FieldType::Bool => DatabaseType::Bool,
            FieldType::I8 => DatabaseType::Int32,
            FieldType::I16 => DatabaseType::Int32,
            FieldType::I32 => DatabaseType::Int32,
            FieldType::I64 => DatabaseType::Int64,
            FieldType::I128 => DatabaseType::Int64,
            FieldType::U8 => DatabaseType::Int32,
            FieldType::U16 => DatabaseType::Int32,
            FieldType::U32 => DatabaseType::Int64,
            FieldType::U64 => DatabaseType::Int64,
            FieldType::U128 => DatabaseType::Int64,
            FieldType::F32 => DatabaseType::Double,
            FieldType::F64 => DatabaseType::Double,
            FieldType::Decimal => DatabaseType::Decimal(None, None),
            FieldType::String => DatabaseType::String,
            FieldType::Date => DatabaseType::DateTime(3),
            FieldType::DateTime => DatabaseType::DateTime(3),
            FieldType::Enum(_) => DatabaseType::Undefined,
            FieldType::Vec(_) => DatabaseType::Undefined,
            FieldType::Map(_) => DatabaseType::Undefined,
            FieldType::Object(_) => DatabaseType::Undefined,
        }
    }

    async fn build_connector(&self, models: &Vec<Model>, reset_database: bool) -> Box<dyn Connector> {
        Box::new(MongoDBConnector::new(self.url.clone(), models, reset_database).await)
    }
}
