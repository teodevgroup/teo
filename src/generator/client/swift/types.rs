use std::borrow::Cow;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};

pub(crate) fn field_type_to_swift_output_type(field_type: &FieldType) -> Cow<str> {
    match field_type {
        FieldType::ObjectId => Cow::Borrowed("String"),
        FieldType::Bool => Cow::Borrowed("Bool"),
        FieldType::I32 => Cow::Borrowed("Int32"),
        FieldType::I64 => Cow::Borrowed("Int64"),
        FieldType::F32 => Cow::Borrowed("Float"),
        FieldType::F64 => Cow::Borrowed("Double"),
        FieldType::Decimal => Cow::Borrowed("Decimal"),
        FieldType::String => Cow::Borrowed("String"),
        FieldType::Date => Cow::Borrowed("String"),
        FieldType::DateTime => Cow::Borrowed("Date"),
        FieldType::Enum(enum_def) => Cow::Borrowed(enum_def.name()),
        FieldType::Vec(inner) => Cow::Owned("[".to_owned() + field_type_to_swift_output_type(inner.field_type()).as_ref() + if inner.is_optional() { "?" } else { "" } + "]"),
        FieldType::HashMap(_) => unreachable!(),
        FieldType::BTreeMap(_) => unreachable!(),
        FieldType::Object(_) => unreachable!(),
    }
}

pub(crate) fn field_type_to_swift_vec(field_type: &str) -> Cow<str> {
    Cow::Owned("[".to_owned() + field_type + "]")
}
