use std::fmt::Debug;
use std::sync::{Arc};
use async_trait::async_trait;
use serde_json::{Value as JsonValue};
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::graph::{Graph};
use crate::error::ActionError;


#[async_trait]
pub(crate) trait Connector: Debug + Send + Sync {

    async fn drop_database(&self);

    async fn sync_graph(&self, graph: &Graph);

    async fn save_object(&self, object: &Object) -> Result<(), ActionError>;

    async fn delete_object(&self, object: &Object);

    async fn find_unique(&self, model: &Model, finder: JsonValue) -> Option<Object>;

    async fn find_one(&self, model: &Model, finder: JsonValue) -> Option<Object>;

    async fn find_many(&self, model: &Model, finder: JsonValue) -> Vec<Object>;
}
