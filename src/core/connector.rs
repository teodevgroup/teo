use std::fmt::Debug;
use std::sync::{Arc};
use async_trait::async_trait;
use serde_json::{Value as JsonValue};
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::graph::Graph;


#[async_trait]
pub trait Connector: Debug + Send + Sync {

    async fn connect(self: Arc<Self>);

    async fn sync_graph(self: Arc<Self>, graph: &Graph);

    async fn save_object(self: Arc<Self>, object: Arc<Object>);

    async fn delete_object(self: Arc<Self>, object: Arc<Object>);

    async fn find_unique(self: Arc<Self>, model: &Model, finder: JsonValue) -> Arc<Object>;

    async fn find_one(self: Arc<Self>, model: &Model, finder: JsonValue) -> Arc<Object>;

    async fn find_many(self: Arc<Self>, model: &Model, finder: JsonValue) -> Vec<Arc<Object>>;
}
