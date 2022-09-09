use serde_json::{Value as JsonValue};
use crate::core::object::Object;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum JsonPipelineContextStage {
    Value,
    ConditionTrue,
    ConditionFalse,
}

#[derive(Debug)]
pub(crate) struct JsonPipelineContext {
    value: Option<JsonValue>,
    location: Vec<String>,
    object: JsonValue,
    stage: JsonPipelineContextStage,
    identity: Option<Object>,
}

impl JsonPipelineContext {
    pub(crate) fn new(initial: JsonValue, identity: Option<Object>) -> Self {
        Self {
            value: Some(initial.clone()),
            location: vec![],
            object: initial,
            stage: JsonPipelineContextStage::Value,
            identity
        }
    }

    pub(crate) fn construct(value: Option<JsonValue>, location: Vec<String>, object: JsonValue, stage: JsonPipelineContextStage, identity: Option<Object>) -> Self {
        Self {
            value, location, object, stage, identity
        }
    }

    pub(crate) fn value(&self) -> Option<&JsonValue> {
        self.value.as_ref()
    }

    pub(crate) fn location(&self) -> &Vec<String> {
        &self.location
    }

    pub(crate) fn object(&self) -> &JsonValue {
        &self.object
    }

    pub(crate) fn stage(&self) -> JsonPipelineContextStage {
        self.stage
    }

    pub(crate) fn identity(&self) -> Option<&Object> {
        self.identity.as_ref()
    }
}
