use std::collections::HashMap;
use std::{fmt, ops};
use std::fmt::{Display, Write};
use super::Value;

// Code from this file is inspired from serde json
// https://github.com/serde-rs/json/blob/master/src/value/index.rs

pub(crate) trait Index {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value>;
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value>;
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value;
}

impl Index for usize {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Vec(vec) => vec.get(*self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v {
            Value::Vec(vec) => vec.get_mut(*self),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        match v {
            Value::Vec(vec) => {
                let len = vec.len();
                vec.get_mut(*self).unwrap_or_else(|| {
                    panic!(
                        "cannot access index {} of Tson array of length {}",
                        self, len
                    )
                })
            }
            _ => panic!("cannot access index {} of Tson {}", self, Type(v)),
        }
    }
}

impl Index for str {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::HashMap(map) => map.get(self),
            Value::BTreeMap(map) => map.get(self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v {
            Value::HashMap(map) => map.get_mut(self),
            Value::BTreeMap(map) => map.get_mut(self),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        if let Value::Null = v {
            *v = Value::HashMap(HashMap::new());
        }
        match v {
            Value::HashMap(map) => map.entry(self.to_owned()).or_insert(Value::Null),
            Value::BTreeMap(map) => map.entry(self.to_owned()).or_insert(Value::Null),
            _ => panic!("cannot access key {:?} in JSON {}", self, Type(v)),
        }
    }
}

impl Index for String {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        self[..].index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        self[..].index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        self[..].index_or_insert(v)
    }
}

impl<'a, T> Index for &'a T where T: ?Sized + Index, {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        (**self).index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        (**self).index_into_mut(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        (**self).index_or_insert(v)
    }
}

/// Used in panic messages.
struct Type<'a>(&'a Value);

impl<'a> Display for Type<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Value::Null => formatter.write_str("null"),
            Value::Bool(_) => formatter.write_str("bool"),
            #[cfg(feature = "data-source-mongodb")]
            Value::ObjectId(_) => formatter.write_str("object id"),
            Value::I8(_) => formatter.write_str("i8"),
            Value::I16(_) => formatter.write_str("i16"),
            Value::I32(_) => formatter.write_str("i32"),
            Value::I64(_) => formatter.write_str("i64"),
            Value::I128(_) => formatter.write_str("i128"),
            Value::U8(_) => formatter.write_str("u8"),
            Value::U16(_) => formatter.write_str("u16"),
            Value::U32(_) => formatter.write_str("u32"),
            Value::U64(_) => formatter.write_str("u64"),
            Value::U128(_) => formatter.write_str("u128"),
            Value::F32(_) => formatter.write_str("f32"),
            Value::F64(_) => formatter.write_str("f64"),
            Value::String(_) => formatter.write_str("string"),
            Value::Vec(_) => formatter.write_str("vec"),
            Value::HashMap(_) => formatter.write_str("hash map"),
            Value::HashSet(_) => formatter.write_str("hash set"),
            Value::Decimal(_) => formatter.write_str("decimal"),
            Value::Date(_) => formatter.write_str("date"),
            Value::DateTime(_) => formatter.write_str("date time"),
            Value::BTreeMap(_) => formatter.write_str("btree map"),
            Value::BTreeSet(_) => formatter.write_str("btree set"),
            Value::Object(_) => formatter.write_str("object"),
        }
    }
}

impl<I> ops::Index<I> for Value where I: Index {
    type Output = Value;
    fn index(&self, index: I) -> &Value {
        static NULL: Value = Value::Null;
        index.index_into(self).unwrap_or(&NULL)
    }
}

impl<I> ops::IndexMut<I> for Value where I: Index {
    fn index_mut(&mut self, index: I) -> &mut Value {
        index.index_or_insert(self)
    }
}
