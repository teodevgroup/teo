use chrono::{Date, DateTime, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use sqlx::any::{AnyRow, AnyValueRef};
use sqlx::{Row, ValueRef};
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::field::r#type::FieldType;
use crate::core::tson::Value;

pub(crate) struct RowDecoder { }

impl RowDecoder {

    pub(crate) fn decode_serial(optional: bool, row: &AnyRow, column_name: &str) -> Value {
        if optional {
            let any_value: AnyValueRef = row.try_get_raw(column_name).unwrap();
            if any_value.is_null() {
                return Value::Null;
            }
        }
        Value::I32(row.get(column_name))
    }

    pub(crate) fn decode(r#type: &FieldType, optional: bool, row: &AnyRow, column_name: &str, dialect: SQLDialect) -> Value {
        if optional {
            let any_value: AnyValueRef = row.try_get_raw(column_name).unwrap();
            if any_value.is_null() {
                return Value::Null;
            }
        }
        if r#type.is_bool() {
            return Value::Bool(row.get(column_name))
        }
        if r#type.is_string() {
            return Value::String(row.get(column_name))
        }
        if r#type.is_int() {
            if dialect == SQLDialect::MySQL {
                if r#type.is_sint() {
                    return Value::number_from_i64(row.get(column_name), r#type);
                }
                if r#type.is_uint() {
                    return Value::number_from_u64(row.get(column_name), r#type);
                }
                panic!("Unhandled database when decoding type.")
            } else {
                match r#type {
                    FieldType::I8 | FieldType::I16 | FieldType::U8 => return Value::number_from_i16(row.get(column_name), r#type),
                    FieldType::I32 | FieldType::U16 => return Value::number_from_i32(row.get(column_name), r#type),
                    FieldType::I64 | FieldType::U32 | FieldType::U64 => return Value::number_from_i64(row.get(column_name), r#type),
                    _ => panic!(""),
                }
            }
        }
        if r#type.is_float() {
            return Value::number_from_f64(row.get(column_name), r#type);
        }
        // #[cfg(feature = "data-source-mysql")]
        // if r#type.is_decimal() {
        //     return Value::Decimal(row.get(column_name));
        // }
        #[cfg(not(feature = "data-source-mssql"))]
        if r#type.is_date() {
            if dialect == SQLDialect::PostgreSQL {
                let timestamp: NaiveDateTime = row.get(column_name);
                let naive_date = timestamp.date();
                let date: Date<Utc> = Date::from_utc(naive_date, Utc);
                return Value::Date(date);
            } else {
                let naive_date: NaiveDate = row.get(column_name);
                let date: Date<Utc> = Date::from_utc(naive_date, Utc);
                return Value::Date(date);
            }
        }
        #[cfg(not(feature = "data-source-mssql"))]
        if r#type.is_datetime() {
            if dialect == SQLDialect::PostgreSQL {
                let timestamp: NaiveDateTime = row.get(column_name);
                let datetime: DateTime<Utc> = DateTime::from_utc(timestamp, Utc);
                return Value::DateTime(datetime);
            } else {
                let datetime: DateTime<Utc> = row.get(column_name);
                return Value::DateTime(datetime);
            }
        }
        panic!("Unhandled database when decoding type.")
    }
}
