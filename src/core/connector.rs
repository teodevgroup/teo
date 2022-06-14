use std::fmt::Debug;
use std::sync::Arc;
use async_trait::async_trait;
use serde_json::{Map, Value as JsonValue};
use crate::core::database_type::DatabaseType;
use crate::core::field_type::FieldType;
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::save_session::SaveSession;
use crate::error::ActionError;


#[async_trait]
pub(crate) trait Connector: Debug + Send + Sync {

    async fn save_object(&self, object: &Object) -> Result<(), ActionError>;

    async fn delete_object(&self, object: &Object) -> Result<(), ActionError>;

    async fn find_unique(&self, graph: &Graph, model: &Model, finder: &Map<String, JsonValue>) -> Result<Object, ActionError>;

    async fn find_first(&self, graph: &Graph, model: &Model, finder: &Map<String, JsonValue>) -> Result<Object, ActionError>;

    async fn find_many(&self, graph: &Graph, model: &Model, finder: &Map<String, JsonValue>) -> Result<Vec<Object>, ActionError>;

    async fn count(&self, graph: &Graph, model: &Model, finder: &Map<String, JsonValue>) -> Result<usize, ActionError>;

    fn new_save_session(&self) -> Arc<dyn SaveSession>;
}

#[async_trait]
pub(crate) trait ConnectorBuilder: Debug + Send + Sync {

    fn inferred_database_type(&self, field_type: &FieldType) -> DatabaseType;

    async fn build_connector(&self, models: &Vec<Model>, reset_database: bool) -> Box<dyn Connector>;
}
