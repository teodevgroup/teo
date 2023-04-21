use crate::core::database::name::DatabaseName;
use crate::core::database::r#type::DatabaseType;
use crate::core::field::r#type::FieldType;

#[derive(Debug)]
pub(crate) struct ConnectorConf {
    pub(crate) provider: DatabaseName,
    pub(crate) url: &'static str,
}

impl ConnectorConf {
    pub(crate) fn default_database_type(&self, field_type: &FieldType) -> DatabaseType {
        self.provider.default_database_type(field_type)
    }
}