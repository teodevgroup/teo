use std::fmt::Debug;
use std::sync::Arc;
use async_trait::async_trait;
use crate::core::action::Action;
use crate::core::database::name::DatabaseName;
use crate::core::initiator::Initiator;
use crate::core::database::r#type::DatabaseType;
use crate::core::field::r#type::FieldType;
use crate::core::graph::Graph;
use crate::core::model::model::Model;
use crate::core::object::Object;
use crate::core::result::Result;
use crate::prelude::Value;

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

#[async_trait]
pub(crate) trait SaveSession: Debug + Send + Sync { }

#[async_trait]
pub(crate) trait Connector: Send + Sync {

    // Migration

    async fn migrate(&self, models: Vec<&Model>, reset_database: bool) -> Result<()>;

    // Purge

    async fn purge(&self, graph: &Graph) -> Result<()>;

    // Raw query

    async fn query_raw(&self, query: &Value) -> Result<Value>;

    // Object manipulation

    async fn save_object(&self, object: &Object, session: Arc<dyn SaveSession>) -> Result<()>;

    async fn delete_object(&self, object: &Object, session: Arc<dyn SaveSession>) -> Result<()>;

    async fn find_unique(&self, graph: &Graph, model: &Model, finder: &Value, mutation_mode: bool, action: Action, action_source: Initiator) -> Result<Option<Object>>;

    async fn find_many(&self, graph: &Graph, model: &Model, finder: &Value, mutation_mode: bool, action: Action, action_source: Initiator) -> Result<Vec<Object>>;

    async fn count(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<usize>;

    async fn aggregate(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<Value>;

    async fn group_by(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<Value>;

    // Save session

    fn new_save_session(&self) -> Arc<dyn SaveSession>;
}
