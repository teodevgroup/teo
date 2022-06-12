use crate::core::value::Value;


pub(crate) enum AtomicUpdateType {
    Increment(Value),
    Decrement(Value),
    Multiply(Value),
    Divide(Value),
    Push(Value),
}

pub(crate) enum FieldInput {
    SetValue(Value),
    AtomicUpdate(AtomicUpdateType)
}
