use crate::prelude::Value;

pub(crate) fn encode_value_to_sql_input(value: Value) -> String {
    match value {
        Value::Null => "NULL".to_owned(),
        Value::String(string) => escape_string(&string),
        Value::I8(i) => i.to_string(),
        Value::I16(i) => i.to_string(),
        Value::I32(i) => i.to_string(),
        Value::I64(i) => i.to_string(),
        Value::I128(i) => i.to_string(),
        Value::U8(i) => i.to_string(),
        Value::U16(i) => i.to_string(),
        Value::U32(i) => i.to_string(),
        Value::U64(i) => i.to_string(),
        Value::U128(i) => i.to_string(),
        Value::F32(i) => i.to_string(),
        Value::F64(i) => i.to_string(),
        Value::Bool(b) => if b { "TRUE".to_owned() } else { "FALSE".to_owned() },
        _ => panic!("unhandled"),
    }
}

fn escape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 2);
    result.push('\'');
    for ch in s.chars() {
        match ch {
            '\'' => result.push_str("\\'"),
            _ => result.push(ch)
        }
    }
    result.push('\'');
    result
}
