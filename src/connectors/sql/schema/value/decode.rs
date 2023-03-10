use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::field::r#type::FieldType;
use crate::core::teon::Value;
use chrono::{NaiveDate, DateTime, Utc};
use quaint::prelude::ResultRow;

pub(crate) struct RowDecoder { }

impl RowDecoder {

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
            return Value::number_from_f32(row.get(column_name).unwrap().as_f32().unwrap(), r#type);
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
