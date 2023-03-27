use std::borrow::Cow;
use crate::core::action::{Action, AGGREGATE_HANDLER, COUNT_HANDLER, CREATE_HANDLER, CREATE_MANY_HANDLER, DELETE_HANDLER, DELETE_MANY_HANDLER, FIND_FIRST_HANDLER, FIND_MANY_HANDLER, FIND_UNIQUE_HANDLER, GROUP_BY_HANDLER, IDENTITY_HANDLER, SIGN_IN_HANDLER, UPDATE_HANDLER, UPDATE_MANY_HANDLER, UPSERT_HANDLER};
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

pub(crate) fn swift_action_result(action: Action, model: &str) -> Cow<str> {
    Cow::Owned(match action.to_u32() {
        FIND_UNIQUE_HANDLER => format!("Response<{model}>?"),
        FIND_FIRST_HANDLER => format!("Response<{model}>?"),
        FIND_MANY_HANDLER => format!("ResponseWithMeta<PagingInfo, [{model}]>"),
        CREATE_HANDLER => format!("Response<{model}>"),
        UPDATE_HANDLER => format!("Response<{model}>"),
        UPSERT_HANDLER => format!("Response<{model}>"),
        DELETE_HANDLER => format!("Response<{model}>"),
        CREATE_MANY_HANDLER => format!("ResponseWithMeta<PagingInfo, [{model}]>"),
        UPDATE_MANY_HANDLER => format!("ResponseWithMeta<PagingInfo, [{model}]>"),
        DELETE_MANY_HANDLER => format!("ResponseWithMeta<PagingInfo, [{model}]>"),
        COUNT_HANDLER => format!("Response<Int64>"),
        AGGREGATE_HANDLER => format!("Response<{model}>"),
        GROUP_BY_HANDLER => format!("Response<{model}>"),
        SIGN_IN_HANDLER => format!("ResponseWithMeta<TokenInfo, {model}>"),
        IDENTITY_HANDLER => format!("Response<{model}>"),
        _ => unreachable!()
    })
}
