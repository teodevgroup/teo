use std::borrow::Cow;
use crate::core::action::{Action, AGGREGATE_HANDLER, COUNT_HANDLER, CREATE_HANDLER, CREATE_MANY_HANDLER, DELETE_HANDLER, DELETE_MANY_HANDLER, FIND_FIRST_HANDLER, FIND_MANY_HANDLER, FIND_UNIQUE_HANDLER, GROUP_BY_HANDLER, IDENTITY_HANDLER, SIGN_IN_HANDLER, UPDATE_HANDLER, UPDATE_MANY_HANDLER, UPSERT_HANDLER};
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::gen::internal::type_lookup::TypeLookup;

pub(crate) struct SwiftTypes { }

impl SwiftTypes {
    pub(crate) fn new() -> Self { Self { } }
}

impl TypeLookup for SwiftTypes {
    fn field_type_to_filter_type<'a>(&self, _field_type: &'a FieldType, _nullable: bool) -> Cow<'a, str> {
        Cow::Borrowed("AnyEncodable")
    }

    fn field_type_to_filter_with_aggregates_type<'a>(&self, _field_type: &'a FieldType, _nullable: bool) -> Cow<'a, str> {
        Cow::Borrowed("AnyEncodable")
    }

    fn field_type_to_create_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        if nullable {
            Cow::Borrowed("AnyEncodable")
        } else {
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
                FieldType::Enum(_) => Cow::Borrowed(field_type.unwrap_enum().name()),
                FieldType::Vec(inner) => Cow::Owned(("[").to_owned() + self.field_type_to_result_type(inner.field_type()).as_ref() + ">"),
                FieldType::HashMap(_) => unreachable!(),
                FieldType::BTreeMap(_) => unreachable!(),
                FieldType::Object(_) => unreachable!(),
            }
        }

    }

    fn field_type_to_update_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        if nullable {
            Cow::Borrowed("AnyEncodable")
        } else {
            match field_type {
                FieldType::ObjectId => Cow::Borrowed("String"),
                FieldType::Bool => Cow::Borrowed("Bool"),
                FieldType::I32 => Cow::Borrowed("AnyEncodable"),
                FieldType::I64 => Cow::Borrowed("AnyEncodable"),
                FieldType::F32 => Cow::Borrowed("AnyEncodable"),
                FieldType::F64 => Cow::Borrowed("AnyEncodable"),
                FieldType::Decimal => Cow::Borrowed("AnyEncodable"),
                FieldType::String => Cow::Borrowed("String"),
                FieldType::Date => Cow::Borrowed("String"),
                FieldType::DateTime => Cow::Borrowed("Date"),
                FieldType::Enum(_) => Cow::Borrowed(field_type.unwrap_enum().name()),
                FieldType::Vec(_inner) => Cow::Borrowed("AnyEncodable"),
                FieldType::HashMap(_) => unreachable!(),
                FieldType::BTreeMap(_) => unreachable!(),
                FieldType::Object(_) => unreachable!(),
            }
        }
    }

    fn field_type_to_result_type<'a>(&self, field_type: &'a FieldType) -> Cow<'a, str> {
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
            FieldType::Enum(_) => Cow::Borrowed(field_type.unwrap_enum().name()),
            FieldType::Vec(inner) => Cow::Owned("[".to_owned() + self.field_type_to_result_type(inner.field_type()).as_ref() + if inner.is_optional() { "?" } else { "" } + "]"),
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

    fn generated_type_to_optional<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned(generated_type.as_ref().to_owned() + "?")
    }

    fn generated_type_to_or_null<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned("NullOr<".to_owned() + generated_type.as_ref() + ">")
    }

    fn action_result_type<'a>(&self, action: Action, model_name: &'a str) -> Cow<'a, str> {
        Cow::Owned(match action.to_u32() {
            FIND_UNIQUE_HANDLER => format!("Response<{model_name}>"),
            FIND_FIRST_HANDLER => format!("Response<{model_name}>"),
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

    fn number_type(&self) -> &'static str {
        "Int"
    }

    fn bool_type(&self) -> &'static str {
        "Bool"
    }
}
