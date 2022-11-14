use bson::Bson;
use bson::datetime::{DateTime as BsonDateTime};
use crate::prelude::Value;

pub(crate) mod decoder;

impl Into<Bson> for Value {
    fn into(self) -> Bson {
        match self {
            Value::Null => Bson::Null,
            Value::ObjectId(oid) => Bson::ObjectId(oid),
            Value::Bool(b) => Bson::Boolean(b),
            Value::I8(i) => Bson::Int32(i as i32),
            Value::I16(i) => Bson::Int32(i as i32),
            Value::I32(i) => Bson::Int32(i),
            Value::I64(i) => Bson::Int64(i),
            Value::I128(i) => Bson::Int64(i as i64),
            Value::U8(u) => Bson::Int32(u as i32),
            Value::U16(u) => Bson::Int32(u as i32),
            Value::U32(u) => Bson::Int64(u as i64),
            Value::U64(u) => Bson::Int64(u as i64),
            Value::U128(u) => Bson::Int64(u as i64),
            Value::F32(f) => Bson::Double(f as f64),
            Value::F64(f) => Bson::Double(f as f64),
            Value::Decimal(d) => panic!("Decimal is not implemented by MongoDB."),
            Value::String(s) => Bson::String(s),
            Value::Date(val) => Bson::DateTime(BsonDateTime::parse_rfc3339_str(val.format("%Y-%m-%d").to_string()).unwrap()),
            Value::DateTime(val) => Bson::DateTime(BsonDateTime::from(val)),
            Value::Vec(val) => Bson::Array(val.iter().map(|i| { i.into() }).collect()),
            Value::HashMap(val) => Bson::Document(val.iter().map(|(k, v)| (k.clone(), v.into())).collect()),
            Value::BTreeMap(val) => Bson::Document(val.iter().map(|(k, v)| (k.clone(), v.into())).collect()),
            Value::IndexMap(val) => Bson::Document(val.iter().map(|(k, v)| (k.clone(), v.into())).collect()),
            Value::Object(_) => panic!("Save embedded object is not implemented."),
            _ => panic!("Cannot convert to bson.")
        }
    }
}
