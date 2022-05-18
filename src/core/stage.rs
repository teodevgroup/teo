use crate::core::value::Value;


#[derive(Debug, Clone)]
pub enum Stage {
    Value(Value),
    Invalid(String),
    ConditionTrue(Value),
    ConditionFalse(Value),
}

impl Stage {
    pub fn value(&self) -> Option<Value> {
        return match self {
            Stage::Value(v) => { Some(v.clone()) }
            Stage::ConditionTrue(v) => { Some(v.clone()) }
            Stage::ConditionFalse(v) => { Some(v.clone()) }
            _ => { None }
        }
    }
}

unsafe impl Send for Stage {}
unsafe impl Sync for Stage {}
