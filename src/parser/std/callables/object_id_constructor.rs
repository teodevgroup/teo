use bson::oid::ObjectId;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn object_id_constructor(args: Vec<Argument>) -> Value {
    let b = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap();
    return Value::ObjectId(ObjectId::parse_str(b).unwrap())
}
