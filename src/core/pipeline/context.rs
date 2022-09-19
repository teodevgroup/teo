use crate::core::object::Object;
use crate::core::value::Value;

pub(crate) enum Validity {
    Valid,
    Invalid(String)
}

pub(crate) enum Stage {
    Normal,
    ConditionTrue,
    ConditionFalse,
}

pub(crate) struct Context {
    value: Value,
    object: Object,
    key_path: Object,
    identity: Option<Object>,
    validity: Validity,
    stage: Stage,
}
