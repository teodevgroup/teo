use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn datetime_constructor(args: Vec<Argument>) -> Value {
    let b = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap();
    Value::DateTime(DateTime::parse_from_rfc3339(b).unwrap().with_timezone(&Utc))
}
