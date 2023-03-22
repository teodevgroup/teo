use std::borrow::Cow;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::core::relation::Relation;

pub(crate) fn field_to_nodejs_api_type<T>(field: &T) -> String where T: FieldTypeOwner {
    let base = match field.field_type() {
        FieldType::String => "string".to_string(),
        FieldType::Date => "Date".to_string(),
        FieldType::DateTime => "Date".to_string(),
        FieldType::Bool => "boolean".to_string(),
        FieldType::I32 | FieldType::I64 | FieldType::F32 | FieldType::F64 => "number".to_string(),
        FieldType::Decimal => "Decimal".to_string(),
        FieldType::Vec(inner) => field_to_nodejs_api_type(inner.as_ref()) + "[]",
        FieldType::Object(name) => name.to_string(),
        FieldType::Enum(name) => name.to_string(),
        _ => unreachable!(),
    };
    return if field.is_optional() {
        base + " | null"
    } else {
        base
    }
}

pub(crate) fn relation_to_nodejs_api_type(relation: &Relation) -> String {
    if relation.is_vec() {
        relation.model().to_string() + "[]"
    } else {
        let result = relation.model().to_string();
        if relation.is_optional() {
            result + " | null"
        } else {
            result
        }
    }
}
