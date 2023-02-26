use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn float_constructor(args: Vec<Argument>) -> Value {
    let b = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap();
    Value::F64(b.parse().unwrap())
}
