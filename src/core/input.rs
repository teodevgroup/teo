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
    // where, create
    ConnectOrCreate(Value, Value),

    // update only

    Disconnect(Value),
    Update(Value),
    // create, update
    Upsert(Value, Value),
    Delete(Value),
}

pub(crate) enum Input {
    SetValue(Value),
    AtomicUpdate(AtomicUpdateType),
    RelationInput(RelationInputType),
}

unsafe impl Send for AtomicUpdateType { }
unsafe impl Sync for AtomicUpdateType { }
