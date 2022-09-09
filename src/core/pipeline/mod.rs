use std::sync::{Arc};
use crate::core::modifier::Modifier;
use crate::core::pipeline::stage::Stage;
use crate::core::object::Object;

pub mod builder;
pub mod stage;

#[derive(Debug)]
pub struct Pipeline {
    pub modifiers: Vec<Arc<dyn Modifier>>
}

impl Pipeline {

    pub(crate) fn has_any_modifier(&self) -> bool {
        self.modifiers.len() > 0
    }

    pub(crate) async fn process(&self, mut stage: Stage, object: &Object) -> Stage {
        for modifier in &self.modifiers {
            stage = modifier.call(stage.clone(), object).await;
            match stage {
                Stage::Invalid(s) => {
                    return Stage::Invalid(s)
                }
                Stage::Value(v) => {
                    stage = Stage::Value(v);
                }
                Stage::ConditionTrue(v) => {
                    stage = Stage::ConditionTrue(v);
                }
                Stage::ConditionFalse(v) => {
                    stage = Stage::ConditionFalse(v);
                }
            }
        }
        return stage;
    }
}

impl Clone for Pipeline {
    fn clone(&self) -> Self {
        Pipeline { modifiers: self.modifiers.clone() }
    }
}

unsafe impl Send for Pipeline {}
unsafe impl Sync for Pipeline {}
