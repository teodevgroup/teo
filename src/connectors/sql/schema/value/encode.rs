use bigdecimal::BigDecimal;
use chrono::{NaiveDate, Utc, DateTime, SecondsFormat};
use itertools::Itertools;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::prelude::Value;

pub trait ToSQLString {
    fn to_string(&self, dialect: SQLDialect) -> String;
}

pub trait TypeOrNull {
    fn or_null(&self, optional: bool) -> String;
}

impl TypeOrNull for &str {
    fn or_null(&self, optional: bool) -> String {
        self.to_string() + if optional { " or null" } else { "" }
    }
}

pub(crate) trait ValueToSQLString {
    fn to_sql_string<'a>(&self, r#type: &FieldType, optional: bool, dialect: SQLDialect) -> String;
    fn to_sql_string_array_arg<'a>(&self, r#type: &FieldType, optional: bool, dialect: SQLDialect) -> String;
}

impl ValueToSQLString for Value {
    fn to_sql_string<'a>(&self, r#type: &FieldType, optional: bool, dialect: SQLDialect) -> String {
        if optional {
            if self.is_null() {
                return "NULL".to_owned()
            }
        }
        match r#type {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => panic!("SQL doesn't support `ObjectId`."),
            FieldType::String => ToSQLInputDialect::to_sql_input(&self.as_str().unwrap(), dialect),
            FieldType::Bool => self.as_bool().unwrap().to_sql_input(),
            FieldType::I32 | FieldType::I64 |
            FieldType::F32 | FieldType::F64 => if let Some(val) = self.as_f64() {
                val.to_string()
            } else if let Some(val) = self.as_i64() {
                val.to_string()
            } else {
                panic!("Uncoded number.")
            }
            FieldType::Enum(_) => ToSQLInputDialect::to_sql_input(&self.as_str().unwrap(), dialect),
            FieldType::Vec(element_field) => {
                let val = self.as_vec().unwrap();
                let mut result: Vec<String> = vec![];
                for (_i, v) in val.iter().enumerate() {
                    result.push(v.to_sql_string(element_field.field_type(), element_field.is_optional(), dialect));
                }
                result.join(", ").wrap_in_array()
            }
            FieldType::Date => self.as_date().unwrap().to_string().to_sql_input(dialect),
            FieldType::DateTime => self.as_datetime().unwrap().to_string().to_sql_input(dialect),
            FieldType::Decimal => self.as_decimal().unwrap().to_string().to_sql_input(dialect),
            _ => { panic!() }
        }
    }

    fn to_sql_string_array_arg<'a>(&self, r#type: &FieldType, optional: bool, dialect: SQLDialect) -> String {
        if optional {
            if self.is_null() {
                return "NULL".to_owned()
            }
        }
        match r#type {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => panic!("SQL doesn't support `ObjectId`."),
            FieldType::String => ToSQLInputDialect::to_sql_input(&self.as_str().unwrap(), dialect),
            FieldType::Bool => self.as_bool().unwrap().to_sql_input(),
            FieldType::I32 | FieldType::I64 |
            FieldType::F32 | FieldType::F64 => if let Some(val) = self.as_f64() {
                val.to_string()
            } else if let Some(val) = self.as_i64() {
                val.to_string()
            } else {
                panic!("Uncoded number.")
            }
            FieldType::Enum(_) => ToSQLInputDialect::to_sql_input(&self.as_str().unwrap(), dialect),
            FieldType::Vec(element_field) => {
                let val = self.as_vec().unwrap();
                let mut result: Vec<String> = vec![];
                for (_i, v) in val.iter().enumerate() {
                    result.push(v.to_sql_string_array_arg(element_field.field_type(), element_field.is_optional(), dialect));
                }
                result.join(",").wrap_in_array()
            }
            FieldType::Date => self.as_date().unwrap().to_string(),
            FieldType::DateTime => self.as_datetime().unwrap().to_string(),
            FieldType::Decimal => self.as_decimal().unwrap().to_string(),
            _ => { panic!() }
        }
    }
}

impl ValueToSQLString for &Value {
    fn to_sql_string<'a>(&self, r#type: &FieldType, optional: bool, dialect: SQLDialect) -> String {
        (*self).to_sql_string(r#type, optional, dialect)
    }

    fn to_sql_string_array_arg<'a>(&self, r#type: &FieldType, optional: bool, dialect: SQLDialect) -> String {
        (*self).to_sql_string_array_arg(r#type, optional, dialect)
    }
}

impl ToSQLString for &Value {
    fn to_string(&self, dialect: SQLDialect) -> String {
        match self {
            Value::Null => "NULL".to_owned(),
            Value::String(string) => string.to_sql_input(dialect),
            Value::I32(i) => i.to_string(),
            Value::I64(i) => i.to_string(),
            Value::F32(i) => i.to_string(),
            Value::F64(i) => i.to_string(),
            Value::Bool(b) => b.to_sql_input(),
            Value::Date(d) => d.to_sql_input(dialect),
            Value::DateTime(d) => d.to_sql_input(dialect),
            Value::Decimal(d) => d.to_sql_input(dialect),
            Value::Vec(values) => format!("array[{}]", values.iter().map(|v| ToSQLString::to_string(&v, dialect)).join(",")),
            Value::RawEnumChoice(string, _) => string.to_sql_input(dialect),
            _ => panic!("unhandled value: {:?}", self),
        }
    }
}

pub(crate) trait PSQLArrayToSQLString {
    fn to_string_with_ft(&self, dialect: SQLDialect, field_type: &FieldType) -> String;
}

fn field_type_to_psql(field_type: &FieldType) -> &'static str {
    match field_type {
        FieldType::Decimal => "decimal",
        FieldType::I32 | FieldType::I64 => "integer",
        FieldType::F32 | FieldType::F64 => "double precision",
        FieldType::String => "text",
        FieldType::Bool => "boolean",
        FieldType::Date => "date",
        FieldType::DateTime => "timestamp",
        _ => unreachable!(),
    }
}

impl PSQLArrayToSQLString for Value {
    fn to_string_with_ft(&self, dialect: SQLDialect, field_type: &FieldType) -> String {
        match self {
            Value::Vec(values) => if values.is_empty() {
                format!("array[]::{}[]", field_type_to_psql(field_type.element_field().unwrap().field_type()))
            } else {
                format!("array[{}]", values.iter().map(|v| {
                    ToSQLString::to_string(&v, dialect)
                }).join(","))
            },
            _ => ToSQLString::to_string(&self, dialect),
        }
    }
}

pub trait ToWrapped {
    fn to_wrapped(&self) -> String;
}

impl ToWrapped for String {
    fn to_wrapped(&self) -> String {
        "(".to_owned() + self + ")"
    }
}

pub trait ToSQLInput {
    fn to_sql_input(&self) -> String;
}

pub trait ToSQLInputDialect {
    fn to_sql_input(&self, dialect: SQLDialect) -> String;
}

impl ToSQLInputDialect for String {
    fn to_sql_input(&self, dialect: SQLDialect) -> String {
        let mut result = String::with_capacity(self.len() + 2);
        result.push('\'');
        for ch in self.chars() {
            match ch {
                '\'' => if dialect.is_mysql() {
                    result.push_str("\\'");
                } else {
                    result.push_str("''");
                },
                _ => result.push(ch)
            }
        }
        result.push('\'');
        result
    }
}


impl ToSQLInputDialect for &str {
    fn to_sql_input(&self, dialect: SQLDialect) -> String {
        let mut result = String::with_capacity(self.len() + 2);
        result.push('\'');
        for ch in self.chars() {
            match ch {
                '\'' => if dialect.is_mysql() {
                    result.push_str("\\'");
                } else {
                    result.push_str("''");
                },
                _ => result.push(ch)
            }
        }
        result.push('\'');
        result
    }
}


impl ToSQLInput for bool {
    fn to_sql_input(&self) -> String {
        if *self { "TRUE".to_owned() } else { "FALSE".to_owned() }
    }
}

impl ToSQLInputDialect for BigDecimal {
    fn to_sql_input(&self, dialect: SQLDialect) -> String {
        let result = self.normalized().to_string();
        if dialect == SQLDialect::PostgreSQL {
            result + "::numeric"
        } else {
            result
        }
    }
}

impl ToSQLInputDialect for NaiveDate {
    fn to_sql_input(&self, dialect: SQLDialect) -> String {
        let result = self.format("%Y-%m-%d").to_string().to_sql_input(dialect);
        if dialect == SQLDialect::PostgreSQL {
            result + "::date"
        } else {
            result
        }
    }
}

impl ToSQLInputDialect for DateTime<Utc> {
    fn to_sql_input(&self, dialect: SQLDialect) -> String {
        if dialect == SQLDialect::SQLite {
            self.to_rfc3339_opts(SecondsFormat::Millis, true).to_sql_input(dialect)
        } else {
            let result = self.format("%Y-%m-%d %H:%M:%S.%3f").to_string().to_sql_input(dialect);
            if dialect == SQLDialect::PostgreSQL {
                result + "::timestamp"
            } else {
                result
            }
        }
    }
}

pub trait IfIMode {
    fn to_i_mode(&self, i_mode: bool) -> String;
}

impl IfIMode for &str {
    fn to_i_mode(&self, i_mode: bool) -> String {
        if i_mode {
            "LOWER(".to_owned() + self + ")"
        } else {
            self.to_string()
        }
    }
}

impl IfIMode for String {
    fn to_i_mode(&self, i_mode: bool) -> String {
        self.as_str().to_i_mode(i_mode)
    }
}

pub trait ToLike {
    fn to_like(&self, left: bool, right: bool) -> String;
}

impl ToLike for &str {
    fn to_like(&self, left: bool, right: bool) -> String {
        let mut retval = "".to_owned();
        retval.push(self.chars().nth(0).unwrap());
        if left {
            retval.push('%');
        }
        retval += &self[1..self.len() - 1];
        if right {
            retval.push('%');
        }
        retval.push(self.chars().nth(self.len() - 1).unwrap());
        retval
    }
}

impl ToLike for String {
    fn to_like(&self, left: bool, right: bool) -> String {
        self.as_str().to_like(left, right)
    }
}

pub trait WrapInArray {
    fn wrap_in_array(&self) -> String;
}

impl WrapInArray for &str {
    fn wrap_in_array(&self) -> String {
        "\'{".to_owned() + self + "}\'"
    }
}

impl WrapInArray for String {
    fn wrap_in_array(&self) -> String {
        self.as_str().wrap_in_array()
    }
}

pub trait SQLEscape {
    fn escape(&self, dialect: SQLDialect) -> String;
}

impl SQLEscape for &str {
    fn escape(&self, dialect: SQLDialect) -> String {
        match dialect {
            SQLDialect::MySQL => format!("`{}`", self),
            SQLDialect::PostgreSQL => format!("\"{}\"", self),
            _ => format!("`{}`", self),
        }
    }
}
