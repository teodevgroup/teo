use crate::core::action::r#type::ActionType;

pub mod r#type;
pub mod builder;

#[derive(Clone)]
struct JsonPipeline { }

#[derive(Clone)]
pub(crate) struct Action {
    transformers: Vec<JsonPipeline>,
}

impl Action {
    pub(crate) fn new() -> Self {
        Action { transformers: vec![] }
    }
}
