use crate::core::value::Value;


pub enum RelationInput {
    Create(Value),
    Set(Value),
    Connect(Value),
    // where, create
    ConnectOrCreate(Value, Value),
    Disconnect(Value),
    Update(Value),
    // create, update
    Upsert(Value, Value),
    Delete(Value),
}