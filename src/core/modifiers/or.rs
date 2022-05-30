use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::object::Object;
use crate::core::pipeline::Pipeline;
use crate::core::stage::Stage;
use crate::core::value::Value;


#[derive(Debug, Clone)]
pub struct OrModifier {
    pipeline: Pipeline
}

impl OrModifier {
    pub fn new<F: Fn(&mut Pipeline)>(build: F) -> Self {
        let mut pipeline = Pipeline::new();
        build(&mut pipeline);
        return OrModifier { pipeline };
    }
}

#[async_trait]
impl Modifier for OrModifier {

    fn name(&self) -> &'static str {
        "or"
    }

    async fn call(&self, stage: Stage, object: &Object) -> Stage {
        if let Some(_) = &stage.value() {
            return stage
        }
        return self.pipeline._process(stage.clone(), object).await
    }
}
