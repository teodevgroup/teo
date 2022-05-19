use std::fmt::Debug;
use std::sync::{Arc};
use async_trait::async_trait;
use serde_json::{Value as JsonValue};
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::graph::{Graph, GraphInner};


#[async_trait]
pub(crate) trait Connector: Debug + Send + Sync {

    async fn connect(self: Arc<Self>);

    async fn disconnect(self: Arc<Self>);

    async fn sync_graph(self: Arc<Self>, graph: Arc<GraphInner>);

    async fn save_object(self: Arc<Self>, object: Object);

    async fn delete_object(self: Arc<Self>, object: Object);

    async fn find_unique(self: Arc<Self>, model: &Model, finder: JsonValue) -> Object;

    async fn find_one(self: Arc<Self>, model: &Model, finder: JsonValue) -> Object;

    async fn find_many(self: Arc<Self>, model: &Model, finder: JsonValue) -> Vec<Object>;
}
