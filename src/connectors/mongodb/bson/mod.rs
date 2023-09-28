use bson::Bson;
use bson::datetime::{DateTime as BsonDateTime};
use chrono::{NaiveDateTime, NaiveTime, TimeZone, Utc};
use crate::prelude::Value;

pub(crate) mod coder;

pub(crate) fn teon_value_to_bson(value: &Value) -> Bson {
    match value {
        Value::Null => Bson::Null,
        Value::ObjectId(oid) => Bson::ObjectId(oid),
        Value::Bool(b) => Bson::Boolean(b),
        Value::I32(i) => Bson::Int32(i),
        Value::I64(i) => Bson::Int64(i),
        Value::F32(f) => Bson::Double(f as f64),
        Value::F64(f) => Bson::Double(f as f64),
        Value::Decimal(_d) => panic!("Decimal is not implemented by MongoDB."),
        Value::String(s) => Bson::String(s),
        Value::Date(val) => Bson::DateTime(BsonDateTime::from(Utc.from_utc_datetime(&NaiveDateTime::new(val, NaiveTime::default())))),
        Value::DateTime(val) => Bson::DateTime(BsonDateTime::from(val)),
        Value::Vec(val) => Bson::Array(val.iter().map(|i| { i.into() }).collect()),
        Value::HashMap(val) => Bson::Document(val.iter().map(|(k, v)| (k.clone(), v.into())).collect()),
        Value::BTreeMap(val) => Bson::Document(val.iter().map(|(k, v)| (k.clone(), v.into())).collect()),
        Value::IndexMap(val) => Bson::Document(val.iter().map(|(k, v)| (k.clone(), v.into())).collect()),
        _ => panic!("Cannot convert to bson.")
    }
}
