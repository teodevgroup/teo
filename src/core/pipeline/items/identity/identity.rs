use async_trait::async_trait;
use crate::core::action::source::ActionSource;
use crate::core::pipeline::item::Item;
use crate::core::teon::Value;
use crate::core::pipeline::ctx::Ctx;
use crate::core::pipeline::Pipeline;

#[derive(Debug, Clone)]
pub struct IdentityModifier {
    pipeline: Pipeline
}

impl IdentityModifier {
    pub fn new(pipeline: Pipeline) -> Self {
        Self { pipeline }
    }
}

#[async_trait]
impl Item for IdentityModifier {
    async fn call<'a>(&self, context: Ctx<'a>) -> Ctx<'a> {
        match context.object.as_ref().unwrap().action_source() {
            ActionSource::Identity(user) => {
                let user = match user {
                    Some(u) => Value::Object(u.clone()),
                    None => Value::Null,
                };
                self.pipeline.process(context.with_value(user)).await
            }
            _ => context
        }
    }
}
