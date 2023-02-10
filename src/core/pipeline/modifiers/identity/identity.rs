use async_trait::async_trait;
use crate::core::action::source::ActionSource;
use crate::core::pipeline::modifier::Modifier;
use crate::core::teon::Value;
use crate::core::pipeline::context::Context;
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
impl Modifier for IdentityModifier {

    fn name(&self) -> &'static str {
        "identity"
    }

    async fn call<'a>(&self, context: Context<'a>) -> Context<'a> {
        match context.object.as_ref().unwrap().action_source() {
            ActionSource::Identity(user) => {
                let user = match user {
                    Some(u) => Value::Object(u.clone()),
                    None => Value::Null,
                };
                self.pipeline.process(context.alter_value(user)).await
            }
            _ => context
        }
    }
}
