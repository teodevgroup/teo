use std::sync::Arc;
use serde_json::{Value as JsonValue};
use crate::core::json_pipeline::items::into::IntoItem;
use crate::core::json_pipeline::items::set::SetItem;
use crate::core::json_pipeline::items::set_default::SetDefaultItem;
use crate::core::json_pipeline::{JsonPipeline, JsonPipelineItem};
use crate::core::json_pipeline::items::out::OutItem;
use crate::core::json_pipeline::items::map::MapItem;
use crate::core::json_pipeline::items::top::TopItem;

#[derive(Debug)]
pub struct JsonPipelineBuilder {
    pub(crate) items: Vec<Arc<dyn JsonPipelineItem>>
}

impl JsonPipelineBuilder {

    pub(crate) fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn set_default(&mut self, key: impl Into<String>, json_value: JsonValue) -> &mut Self {
        self.items.push(Arc::new(SetDefaultItem::new(key, json_value)));
        self
    }

    pub fn set(&mut self, key: impl Into<String>, json_value: JsonValue) -> &mut Self {
        self.items.push(Arc::new(SetItem::new(key, json_value)));
        self
    }

    pub fn into(&mut self, key: impl Into<String>) -> &mut Self {
        self.items.push(Arc::new(IntoItem::new(key)));
        self
    }

    pub fn out(&mut self) -> &mut Self {
        self.items.push(Arc::new(OutItem::new(key)));
        self
    }

    pub fn top(&mut self) -> &mut Self {
        self.items.push(Arc::new(TopItem::new(key)));
        self
    }

    pub fn map<F: Fn(&mut JsonPipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = JsonPipelineBuilder::new();
        build(&mut builder);
        self.items.push(Arc::new(MapItem::new(builder.build())));
        self
    }

    pub fn custom<F: Fn(&mut JsonPipelineBuilder)>(&mut self, build: F) -> &mut Self {
        self
    }

    pub(crate) fn build(&self) -> JsonPipeline {
        JsonPipeline { items: self.items.clone() }
    }
}
