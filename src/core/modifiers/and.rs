use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::stage::Stage;
use crate::core::value::Value;


#[derive(Debug, Clone)]
pub struct AndModifier {
    pipeline: Pipeline
}

impl AndModifier {
    pub fn new<F: Fn(&mut Pipeline)>(build: F) -> Self {
        let mut pipeline = Pipeline::new();
        build(&mut pipeline);
        return AndModifier { pipeline };
    }
}

#[async_trait]
impl Modifier for AndModifier {

    fn name(&self) -> &'static str {
        "and"
    }

    async fn call(&self, stage: Stage, object: &Object) -> Stage {
        if let Some(_) = &stage.value() {
            return self.pipeline._process(stage.clone(), object).await;
        } else {
            return stage;
        }
    }
}
