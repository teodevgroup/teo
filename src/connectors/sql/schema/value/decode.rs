use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::field::r#type::FieldType;
use crate::core::teon::Value;
use chrono::{NaiveDate, DateTime, Utc};
use indexmap::IndexMap;
use quaint_forked::prelude::{ResultRow, ResultSet};

pub(crate) struct RowDecoder { }

impl RowDecoder {

    pub(crate) fn decode_raw(value: &quaint_forked::Value) -> Value {
        match value {
            quaint_forked::Value::Float(f) => {
                match f {
                    Some(f) => Value::F32(*f),
                    None => Value::Null,
                }
            }
            quaint_forked::Value::Double(d) => {
                match d {
                    Some(d) => Value::F64(*d),
                    None => Value::Null,
                }
            }
            quaint_forked::Value::Text(t) => {
                match t {
                    Some(d) => Value::String(d.as_ref().to_owned()),
                    None => Value::Null,
                }
            }
            quaint_forked::Value::Boolean(b) => {
                match b {
                    Some(d) => Value::Bool(*d),
                    None => Value::Null,
                }
            }
            quaint_forked::Value::Date(d) => {
                match d {
                    Some(d) => Value::Date(d.clone()),
                    None => Value::Null,
                }
            }
            quaint_forked::Value::DateTime(d) => {
                match d {
                    Some(d) => Value::DateTime(d.clone()),
                    None => Value::Null,
                }
            }
            quaint_forked::Value::Int32(i) => {
                match i {
                    Some(i) => Value::I32(*i),
                    None => Value::Null,
                }
            }
            quaint_forked::Value::Int64(i) => {
                match i {
                    Some(i) => Value::I64(*i),
                    None => Value::Null,
                }
            }
            quaint_forked::Value::Numeric(d) => {
                match d {
                    Some(i) => Value::Decimal(i.clone()),
                    None => Value::Null,
                }
            }
            _ => unreachable!()
        }
    }

    pub(crate) fn decode_raw_result_set(set: ResultSet) -> Value {
        let columns = set.columns().clone();
        let results: Vec<Value> = set.into_iter().map(|row| {
            let mut map: IndexMap<String, Value> = IndexMap::new();
            for column in columns.iter() {
                let val = row.get(column).unwrap();
                map.insert(column.to_owned(), Self::decode_raw(val));
            }
            Value::IndexMap(map)
        }).collect();
        Value::Vec(results)
    }

    pub(crate) fn decode_serial(optional: bool, row: &ResultRow, column_name: &str) -> Value {
        let try_value = row.get(column_name);
        if try_value.is_none() && optional {
            Value::Null
        } else {
            if try_value.is_none() {
                Value::Null
            } else {
                let val = try_value.unwrap();
                if val.is_i32() {
                    Value::I32(val.as_i32().unwrap())
                } else {
                    Value::I64(val.as_i64().unwrap())
                }
            }
        }
    }

    pub(crate) fn decode(r#type: &FieldType, optional: bool, row: &ResultRow, column_name: &str, dialect: SQLDialect) -> Value {
        let result = row.get(column_name);
        if optional {
            if result.is_none() {
                return Value::Null;
            }
        }
        if r#type.is_bool() {
            return Value::Bool(row.get(column_name).unwrap().as_bool().unwrap())
        }
        if r#type.is_string() {
            return Value::String(row.get(column_name).unwrap().as_str().unwrap().to_owned())
        }
        if r#type.is_int32() {
            return Value::I32(row.get(column_name).unwrap().as_i32().unwrap().to_owned())
        }
        if r#type.is_int64() {
            return Value::I64(row.get(column_name).unwrap().as_i64().unwrap().to_owned())
        }
        if r#type.is_float32() {
            let value = row.get(column_name).unwrap();
            if let Some(f64_val) = row.get(column_name).unwrap().as_f64() {
                return Value::number_from_f64(f64_val, r#type);
            } else if let Some(f32_val) = row.get(column_name).unwrap().as_f32() {
                return Value::number_from_f32(f32_val, r#type);
            } else {
                unreachable!()
            }
        }
        if r#type.is_float64() {
            return Value::number_from_f64(row.get(column_name).unwrap().as_f64().unwrap(), r#type);
        }
        if r#type.is_date() {
            if dialect == SQLDialect::PostgreSQL {
                let naive_date = row.get(column_name).unwrap().as_date().unwrap();
                return Value::Date(naive_date);
            } else if dialect == SQLDialect::SQLite {
                let timestamp: String = row.get(column_name).unwrap().as_str().unwrap().to_owned();
                let naive_date = NaiveDate::parse_from_str(&timestamp, "%Y-%m-%d").unwrap();
                return Value::Date(naive_date);
            } else if dialect == SQLDialect::MySQL {
                let datetime = row.get(column_name).unwrap().as_datetime().unwrap();
                let naive_date = datetime.date_naive();
                return Value::Date(naive_date);
            } else {
                let naive_date = row.get(column_name).unwrap().as_date().unwrap();
                return Value::Date(naive_date);
            }
        }
        if r#type.is_datetime() {
            if dialect == SQLDialect::PostgreSQL {
                let datetime: DateTime<Utc> = row.get(column_name).unwrap().as_datetime().unwrap();
                return Value::DateTime(datetime);
            } else if dialect == SQLDialect::SQLite {
                let timestamp: String = row.get(column_name).unwrap().as_str().unwrap().to_owned();
                return Value::DateTime(DateTime::parse_from_rfc3339(&timestamp).unwrap().with_timezone(&Utc));
            } else {
                let datetime: DateTime<Utc> = row.get(column_name).unwrap().as_datetime().unwrap();
                return Value::DateTime(datetime);
            }
        }

        panic!("Unhandled database when decoding type.")
    }
}
