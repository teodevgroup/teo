use crate::core::pipeline::context::stage::Stage::*;

#[derive(Clone)]
pub(crate) enum Stage {
    Default,
    ConditionTrue,
    ConditionFalse,
}

impl Stage {

    pub(crate) fn is_condition_default(&self) -> bool {
        match self {
            Default => true,
            _ => false
        }
    }

    pub(crate) fn is_condition_true(&self) -> bool {
        match self {
            ConditionTrue => true,
            _ => false
        }
    }

    pub(crate) fn is_condition_false(&self) -> bool {
        match self {
            ConditionFalse => true,
            _ => false
        }
    }
}
