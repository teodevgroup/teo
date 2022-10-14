use chrono::{Date, DateTime, NaiveDate, Utc};
use sqlx::any::{AnyRow, AnyValueRef};
use sqlx::{Row, ValueRef};
use crate::core::field::r#type::FieldType;
use crate::core::tson::Value;

pub(crate) struct RowDecoder { }

impl RowDecoder {
    pub(crate) fn decode(r#type: &FieldType, optional: bool, row: &AnyRow, column_name: &str) -> Value {
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
        if r#type.is_sint() {
            return Value::number_from_i64(row.get(column_name), r#type);
        }
        if r#type.is_uint() {
            return Value::number_from_u64(row.get(column_name), r#type);
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
            let naive_date: NaiveDate = row.get(column_name);
            let date: Date<Utc> = Date::from_utc(naive_date, Utc);
            return Value::Date(date);
        }
        #[cfg(not(feature = "data-source-mssql"))]
        if r#type.is_datetime() {
            let datetime: DateTime<Utc> = row.get(column_name);
            return Value::DateTime(datetime);
        }
        panic!("Unhandled database when decoding type.")
    }
}
