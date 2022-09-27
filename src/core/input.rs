use serde_json::{Value as JsonValue};
use crate::core::value::Value;


pub(crate) enum AtomicUpdateType {
    Increment(Value),
    Decrement(Value),
    Multiply(Value),
    Divide(Value),
    Push(Value),
}

pub enum RelationInputType {

    // both create and update

    Create(JsonValue),
    Set(JsonValue),
    Connect(JsonValue),
    // where, create
    ConnectOrCreate(JsonValue, JsonValue),

    // update only

    Disconnect(JsonValue),
    Update(JsonValue),
    // create, update
    Upsert(JsonValue, JsonValue),
    Delete(JsonValue),
}

pub(crate) enum Input {
    SetValue(Value),
    AtomicUpdate(AtomicUpdateType),
    RelationInput(RelationInputType),
}

unsafe impl Send for AtomicUpdateType { }
unsafe impl Sync for AtomicUpdateType { }
