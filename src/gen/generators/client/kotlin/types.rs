use std::borrow::Cow;
use crate::core::action::{Action, AGGREGATE_HANDLER, COUNT_HANDLER, CREATE_HANDLER, CREATE_MANY_HANDLER, DELETE_HANDLER, DELETE_MANY_HANDLER, FIND_FIRST_HANDLER, FIND_MANY_HANDLER, FIND_UNIQUE_HANDLER, GROUP_BY_HANDLER, IDENTITY_HANDLER, SIGN_IN_HANDLER, UPDATE_HANDLER, UPDATE_MANY_HANDLER, UPSERT_HANDLER};
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::gen::internal::type_lookup::TypeLookup;

pub(crate) struct KotlinTypes { }

impl KotlinTypes {
    pub(crate) fn new() -> Self { Self { } }
}

impl TypeLookup for KotlinTypes {
    fn field_type_to_filter_type<'a>(&self, _field_type: &'a FieldType, _optional: bool) -> Cow<'a, str> {
        Cow::Borrowed("Any")
    }

    fn field_type_to_filter_with_aggregates_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        if nullable {
            match field_type {
                FieldType::ObjectId => Cow::Borrowed("ObjectIdNullableWithAggregatesFilter"),
                FieldType::Bool => Cow::Borrowed("BooleanNullableWithAggregatesFilter"),
                FieldType::I32 => Cow::Borrowed("IntNullableWithAggregatesFilter"),
                FieldType::I64 => Cow::Borrowed("LongNullableWithAggregatesFilter"),
                FieldType::F32 => Cow::Borrowed("FloatNullableWithAggregatesFilter"),
                FieldType::F64 => Cow::Borrowed("DoubleNullableWithAggregatesFilter"),
                FieldType::Decimal => Cow::Borrowed("BigDecimalNullableWithAggregatesFilter"),
                FieldType::String => Cow::Borrowed("StringNullableWithAggregatesFilter"),
                FieldType::Date => Cow::Borrowed("LocalDateNullableWithAggregatesFilter"),
                FieldType::DateTime => Cow::Borrowed("OffsetDateTimeNullableWithAggregatesFilter"),
                _ => unreachable!(),
            }
        } else {
            match field_type {
                FieldType::ObjectId => Cow::Borrowed("ObjectIdWithAggregatesFilter"),
                FieldType::Bool => Cow::Borrowed("BooleanWithAggregatesFilter"),
                FieldType::I32 => Cow::Borrowed("IntWithAggregatesFilter"),
                FieldType::I64 => Cow::Borrowed("LongWithAggregatesFilter"),
                FieldType::F32 => Cow::Borrowed("FloatWithAggregatesFilter"),
                FieldType::F64 => Cow::Borrowed("DoubleWithAggregatesFilter"),
                FieldType::Decimal => Cow::Borrowed("BigDecimalWithAggregatesFilter"),
                FieldType::String => Cow::Borrowed("StringWithAggregatesFilter"),
                FieldType::Date => Cow::Borrowed("LocalDateWithAggregatesFilter"),
                FieldType::DateTime => Cow::Borrowed("OffsetDateTimeWithAggregatesFilter"),
                _ => unreachable!(),
            }
        }
    }

    fn field_type_to_create_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        if nullable {
            Cow::Borrowed("Any")
        } else {
            self.field_type_to_result_type(field_type)
        }
    }

    fn field_type_to_update_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        if nullable {
            Cow::Borrowed("Any")
        } else {
            match field_type {
                FieldType::I32 | FieldType::I64 | FieldType::F32 | FieldType::F64 | FieldType::Decimal | FieldType::Vec(_) => Cow::Borrowed("Any"),
                _ => self.field_type_to_result_type(field_type),
            }
        }
    }

    fn field_type_to_result_type<'a>(&self, field_type: &'a FieldType) -> Cow<'a, str> {
        match field_type {
            FieldType::ObjectId => Cow::Borrowed("String"),
            FieldType::Bool => Cow::Borrowed("Boolean"),
            FieldType::I32 => Cow::Borrowed("Int"),
            FieldType::I64 => Cow::Borrowed("Long"),
            FieldType::F32 => Cow::Borrowed("Float"),
            FieldType::F64 => Cow::Borrowed("Double"),
            FieldType::Decimal => Cow::Borrowed("BigDecimal"),
            FieldType::String => Cow::Borrowed("String"),
            FieldType::Date => Cow::Borrowed("LocalDate"),
            FieldType::DateTime => Cow::Borrowed("OffsetDateTime"),
            FieldType::Enum(enum_def) => Cow::Borrowed(enum_def.name()),
            FieldType::Vec(inner) => Cow::Owned("List<".to_owned() + self.field_type_to_result_type(inner.field_type()).as_ref() + if inner.is_optional() { "?" } else { "" } + ">"),
            FieldType::HashMap(_) => unreachable!(),
            FieldType::BTreeMap(_) => unreachable!(),
            FieldType::Object(_) => unreachable!(),
        }
    }

    fn generated_type_to_vec<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned("List<".to_owned() + generated_type.as_ref() + ">")
    }

    fn generated_type_to_enumerate<'a>(&self, _generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned("List<".to_owned() + generated_type.as_ref() + ">")
    }

    fn generated_type_to_optional<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned(generated_type.as_ref().to_owned() + "?")
    }

    fn generated_type_to_or_null<'a>(&self, _generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Borrowed("Any")
    }

    fn action_result_type<'a>(&self, _action: Action, _model_name: &'a str) -> Cow<'a, str> {
        Cow::Owned(match action.to_u32() {
            FIND_UNIQUE_HANDLER => format!("Response<{model_name}>"),
            FIND_FIRST_HANDLER => format!("Response<{model_name}>"),
            FIND_MANY_HANDLER => format!("ResponseWithMeta<PagingInfo, List<{model_name}>>"),
            CREATE_HANDLER => format!("Response<{model_name}>"),
            UPDATE_HANDLER => format!("Response<{model_name}>"),
            UPSERT_HANDLER => format!("Response<{model_name}>"),
            DELETE_HANDLER => format!("Response<{model_name}>"),
            CREATE_MANY_HANDLER => format!("ResponseWithMeta<PagingInfo, List<{model_name}>>"),
            UPDATE_MANY_HANDLER => format!("ResponseWithMeta<PagingInfo, List<{model_name}>>"),
            DELETE_MANY_HANDLER => format!("ResponseWithMeta<PagingInfo, List<{model_name}>>"),
            COUNT_HANDLER => format!("Response<Int>"),
            AGGREGATE_HANDLER => format!("Response<Any>"),
            GROUP_BY_HANDLER => format!("Response<List<Map<String, Any>>>"),
            SIGN_IN_HANDLER => format!("ResponseWithMeta<TokenInfo, {model_name}>"),
            IDENTITY_HANDLER => format!("Response<{model_name}>"),
            _ => unreachable!()
        })
    }

    fn number_type(&self) -> &'static str {
        "Int"
    }

    fn bool_type(&self) -> &'static str {
        "Boolean"
    }
}
