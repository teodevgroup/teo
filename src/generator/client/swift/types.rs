use std::borrow::Cow;
use crate::core::action::{Action, AGGREGATE_HANDLER, COUNT_HANDLER, CREATE_HANDLER, CREATE_MANY_HANDLER, DELETE_HANDLER, DELETE_MANY_HANDLER, FIND_FIRST_HANDLER, FIND_MANY_HANDLER, FIND_UNIQUE_HANDLER, GROUP_BY_HANDLER, IDENTITY_HANDLER, SIGN_IN_HANDLER, UPDATE_HANDLER, UPDATE_MANY_HANDLER, UPSERT_HANDLER};
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::generator::lib::shared::type_lookup::TypeLookup;

pub(crate) struct SwiftTypes { }

impl SwiftTypes {
    pub(crate) fn new() -> Self { Self { } }
}

impl TypeLookup for SwiftTypes {
    fn number_type(&self) -> &'static str {
        "Int"
    }
    fn field_type_to_filter_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
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
            FieldType::Vec(inner) => Cow::Owned("Array".to_owned() + if optional { "Nullable" } else { "" } + "Filter<" + self.field_type_to_result_type(inner.field_type(), true).as_ref() + if inner.is_optional() { "?" } else { "" } + ">"),
            FieldType::HashMap(_) => unreachable!(),
            FieldType::BTreeMap(_) => unreachable!(),
            FieldType::Object(_) => unreachable!(),
        }
    }

    fn field_type_to_create_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
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
            FieldType::Vec(inner) => Cow::Owned((if optional { "NullOr<[" } else { "[" }).to_owned() + self.field_type_to_result_type(inner.field_type(), true).as_ref() + if inner.is_optional() { "?" } else { "" } + if optional { "]>" } else { ">" }),
            FieldType::HashMap(_) => unreachable!(),
            FieldType::BTreeMap(_) => unreachable!(),
            FieldType::Object(_) => unreachable!(),
        }
    }

    fn field_type_to_update_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
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
            FieldType::Vec(inner) => Cow::Owned((if optional { "NullOr<[" } else { "[" }).to_owned() + self.field_type_to_result_type(inner.field_type(), true).as_ref() + if inner.is_optional() { "?" } else { "" } + if optional { "]>" } else { ">" }),
            FieldType::HashMap(_) => unreachable!(),
            FieldType::BTreeMap(_) => unreachable!(),
            FieldType::Object(_) => unreachable!(),
        }
    }

    fn field_type_to_result_type<'a>(&self, field_type: &'a FieldType, _optional: bool) -> Cow<'a, str> {
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
            FieldType::Vec(inner) => Cow::Owned("[".to_owned() + self.field_type_to_result_type(inner.field_type(), true).as_ref() + if inner.is_optional() { "?" } else { "" } + "]"),
            FieldType::HashMap(_) => unreachable!(),
            FieldType::BTreeMap(_) => unreachable!(),
            FieldType::Object(_) => unreachable!(),
        }
    }

    fn generated_type_to_vec<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned("[".to_owned() + generated_type.as_ref() + "]")
    }

    fn generated_type_to_enumerate<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned("[".to_owned() + generated_type.as_ref() + "]")
    }

    fn action_result_type<'a>(&self, action: Action, model_name: &'a str) -> Cow<'a, str> {
        Cow::Owned(match action.to_u32() {
            FIND_UNIQUE_HANDLER => format!("Response<{model_name}>?"),
            FIND_FIRST_HANDLER => format!("Response<{model_name}>?"),
            FIND_MANY_HANDLER => format!("ResponseWithMeta<PagingInfo, [{model_name}]>"),
            CREATE_HANDLER => format!("Response<{model_name}>"),
            UPDATE_HANDLER => format!("Response<{model_name}>"),
            UPSERT_HANDLER => format!("Response<{model_name}>"),
            DELETE_HANDLER => format!("Response<{model_name}>"),
            CREATE_MANY_HANDLER => format!("ResponseWithMeta<PagingInfo, [{model_name}]>"),
            UPDATE_MANY_HANDLER => format!("ResponseWithMeta<PagingInfo, [{model_name}]>"),
            DELETE_MANY_HANDLER => format!("ResponseWithMeta<PagingInfo, [{model_name}]>"),
            COUNT_HANDLER => format!("Response<Int64>"),
            AGGREGATE_HANDLER => format!("Response<{model_name}>"),
            GROUP_BY_HANDLER => format!("Response<{model_name}>"),
            SIGN_IN_HANDLER => format!("ResponseWithMeta<TokenInfo, {model_name}>"),
            IDENTITY_HANDLER => format!("Response<{model_name}>"),
            _ => unreachable!()
        })
    }
}
