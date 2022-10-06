use crate::core::tson::Value;

pub(crate) enum AtomicUpdateType {
    Increment(Value),
    Decrement(Value),
    Multiply(Value),
    Divide(Value),
    Push(Value),
}

pub enum RelationInputType {
    // both create and update
    Create(Value),
    Set(Value),
    Connect(Value),
    ConnectOrCreate { r#where: Value, create: Value },

    // update only
    Disconnect(Value),
    Update(Value),
    Upsert { r#where: Value, create: Value,  update: Value },
    Delete(Value),
}

pub(crate) enum Input {
    SetValue(Value),
    AtomicUpdate(AtomicUpdateType),
    RelationInput(RelationInputType),
}

unsafe impl Send for AtomicUpdateType { }
unsafe impl Sync for AtomicUpdateType { }
