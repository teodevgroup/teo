use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::stage::Stage;
use crate::core::value::Value;


#[derive(Debug, Clone)]
pub struct DoModifier {
    pipeline: Pipeline
}

impl DoModifier {
    pub fn new<F: Fn(&mut Pipeline)>(build: F) -> Self {
        let mut pipeline = Pipeline::new();
        build(&mut pipeline);
        return DoModifier { pipeline };
    }
}

#[async_trait]
impl Modifier for DoModifier {

    fn name(&self) -> &'static str {
        "do"
    }

    async fn call(&self, stage: Stage, object: &Object) -> Stage {
        self.pipeline._process(stage.clone(), object).await
    }
}
