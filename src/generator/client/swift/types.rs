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

pub(crate) fn field_type_to_swift_filter_type(field_type: &FieldType, optional: bool) -> Cow<str> {
    match field_type {
        FieldType::ObjectId => if optional { Cow::Borrowed("ObjectIdNullableFilter") } else { Cow::Borrowed("ObjectIdFilter") },
        FieldType::Bool => if optional { Cow::Borrowed("BoolNullableFilter") } else { Cow::Borrowed("BoolFilter") },
        FieldType::I32 => if optional { Cow::Borrowed("Int32NullableFilter") } else { Cow::Borrowed("Int32Filter") },
        FieldType::I64 => if optional { Cow::Borrowed("Int64NullableFilter") } else { Cow::Borrowed("Int64Filter") },
        FieldType::F32 => if optional { Cow::Borrowed("FloatNullableFilter") } else { Cow::Borrowed("FloatFilter") },
        FieldType::F64 => if optional { Cow::Borrowed("DoubleNullableFilter") } else { Cow::Borrowed("DoubleFilter") },
        FieldType::Decimal => if optional { Cow::Borrowed("DecimalNullableFilter") } else { Cow::Borrowed("DecimalFilter") },
        FieldType::String => if optional { Cow::Borrowed("StringNullableFilter") } else { Cow::Borrowed("StringFilter") },
        FieldType::Date => if optional { Cow::Borrowed("DateNullableFilter") } else { Cow::Borrowed("DateFilter") },
        FieldType::DateTime => if optional { Cow::Borrowed("DateTimeNullableFilter") } else { Cow::Borrowed("DateTimeFilter") },
        FieldType::Enum(enum_def) => Cow::Owned("Enum".to_owned() + if optional { "Nullable" } else { "" } + "Filter<" + enum_def.name() + ">"),
        FieldType::Vec(inner) => Cow::Owned("Array".to_owned() + if optional { "Nullable" } else { "" } + "Filter<" + field_type_to_swift_output_type(inner.field_type()).as_ref() + if inner.is_optional() { "?" } else { "" } + ">"),
        FieldType::HashMap(_) => unreachable!(),
        FieldType::BTreeMap(_) => unreachable!(),
        FieldType::Object(_) => unreachable!(),
    }
}

pub(crate) fn field_type_to_swift_create_type(field_type: &FieldType, optional: bool) -> Cow<str> {
    match field_type {
        FieldType::ObjectId => if optional { Cow::Borrowed("NullOr<String>") } else { Cow::Borrowed("String") },
        FieldType::Bool => if optional { Cow::Borrowed("NullOr<Bool>") } else { Cow::Borrowed("Bool") },
        FieldType::I32 => if optional { Cow::Borrowed("NullOr<Int32>") } else { Cow::Borrowed("Int32") },
        FieldType::I64 => if optional { Cow::Borrowed("NullOr<Int64>") } else { Cow::Borrowed("Int64") },
        FieldType::F32 => if optional { Cow::Borrowed("NullOr<Float>") } else { Cow::Borrowed("Float") },
        FieldType::F64 => if optional { Cow::Borrowed("NullOr<Double>") } else { Cow::Borrowed("Double") },
        FieldType::Decimal => if optional { Cow::Borrowed("NullOr<Decimal>") } else { Cow::Borrowed("Decimal") },
        FieldType::String => if optional { Cow::Borrowed("NullOr<String>") } else { Cow::Borrowed("String") },
        FieldType::Date => if optional { Cow::Borrowed("NullOr<String>") } else { Cow::Borrowed("String") },
        FieldType::DateTime => if optional { Cow::Borrowed("NullOr<Date>") } else { Cow::Borrowed("Date") },
        FieldType::Enum(enum_def) => if optional { Cow::Owned("NullOr<".to_owned() + enum_def.name() + ">") } else { Cow::Borrowed(enum_def.name()) },
        FieldType::Vec(inner) => Cow::Owned((if optional { "NullOr<[" } else { "[" }).to_owned() + field_type_to_swift_output_type(inner.field_type()).as_ref() + if inner.is_optional() { "?" } else { "" } + if optional { "]>" } else { ">" }),
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
