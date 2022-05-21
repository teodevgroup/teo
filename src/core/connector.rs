use std::fmt::Debug;
use async_trait::async_trait;
use serde_json::{Value as JsonValue};
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::graph::{Graph};
use crate::error::ActionError;


#[async_trait]
pub(crate) trait Connector: Debug + Send + Sync {

    async fn drop_database(&self);

    async fn save_object(&self, object: &Object) -> Result<(), ActionError>;

    async fn delete_object(&self, object: &Object);

    async fn find_unique(&self, model: &Model, finder: JsonValue) -> Option<Object>;

    async fn find_one(&self, model: &Model, finder: JsonValue) -> Option<Object>;

    async fn find_many(&self, model: &Model, finder: JsonValue) -> Vec<Object>;
}

#[async_trait]
pub(crate) trait ConnectorBuilder: Debug + Send + Sync {

    async fn build_connector(&self, models: &Vec<Model>) -> Box<dyn Connector>;
}
