use std::borrow::Cow;
use crate::core::action::{Action, AGGREGATE_HANDLER, COUNT_HANDLER, CREATE_HANDLER, CREATE_MANY_HANDLER, DELETE_HANDLER, DELETE_MANY_HANDLER, FIND_FIRST_HANDLER, FIND_MANY_HANDLER, FIND_UNIQUE_HANDLER, GROUP_BY_HANDLER, IDENTITY_HANDLER, SIGN_IN_HANDLER, UPDATE_HANDLER, UPDATE_MANY_HANDLER, UPSERT_HANDLER};
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::gen::internal::type_lookup::TypeLookup;

pub(crate) struct DartTypes { }

impl DartTypes {
    pub(crate) fn new() -> Self { Self { } }
}

impl TypeLookup for DartTypes {
    fn field_type_to_filter_type<'a>(&self, _field_type: &'a FieldType, _nullable: bool) -> Cow<'a, str> {
        Cow::Borrowed("dynamic")
    }

    fn field_type_to_filter_with_aggregates_type<'a>(&self, _field_type: &'a FieldType, _nullable: bool) -> Cow<'a, str> {
        Cow::Borrowed("dynamic")
    }

    fn field_type_to_create_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        if nullable {
            Cow::Borrowed("dynamic")
        } else {
            match field_type {
                FieldType::ObjectId => Cow::Borrowed("String"),
                FieldType::Bool => Cow::Borrowed("bool"),
                FieldType::I32 => Cow::Borrowed("int"),
                FieldType::I64 => Cow::Borrowed("int"),
                FieldType::F32 => Cow::Borrowed("double"),
                FieldType::F64 => Cow::Borrowed("double"),
                FieldType::Decimal => Cow::Borrowed("Decimal"),
                FieldType::String => Cow::Borrowed("String"),
                FieldType::Date => Cow::Borrowed("String"),
                FieldType::DateTime => Cow::Borrowed("DateTime"),
                FieldType::Enum(enum_def) => Cow::Borrowed(enum_def.name()),
                FieldType::Vec(inner) => Cow::Owned("List<".to_owned() + self.field_type_to_result_type(inner.field_type()).as_ref() + ">"),
                FieldType::HashMap(_) => unreachable!(),
                FieldType::BTreeMap(_) => unreachable!(),
                FieldType::Object(_) => unreachable!(),
            }
        }
    }

    fn field_type_to_update_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        if nullable {
            Cow::Borrowed("dynamic")
        } else {
            match field_type {
                FieldType::ObjectId => Cow::Borrowed("String"),
                FieldType::Bool => Cow::Borrowed("bool"),
                FieldType::I32 => Cow::Borrowed("dynamic"),
                FieldType::I64 => Cow::Borrowed("dynamic"),
                FieldType::F32 => Cow::Borrowed("dynamic"),
                FieldType::F64 => Cow::Borrowed("dynamic"),
                FieldType::Decimal => Cow::Borrowed("dynamic"),
                FieldType::String => Cow::Borrowed("String"),
                FieldType::Date => Cow::Borrowed("String"),
                FieldType::DateTime => Cow::Borrowed("DateTime"),
                FieldType::Enum(enum_def) => Cow::Borrowed(enum_def.name()),
                FieldType::Vec(_inner) => Cow::Borrowed("dynamic"),
                FieldType::HashMap(_) => unreachable!(),
                FieldType::BTreeMap(_) => unreachable!(),
                FieldType::Object(_) => unreachable!(),
            }
        }
    }

    fn field_type_to_result_type<'a>(&self, field_type: &'a FieldType) -> Cow<'a, str> {
        match field_type {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => Cow::Borrowed("String"),
            FieldType::String => Cow::Borrowed("String"),
            FieldType::Bool => Cow::Borrowed("bool"),
            FieldType::I32 => Cow::Borrowed("int"),
            FieldType::I64 => Cow::Borrowed("int"),
            FieldType::F32 => Cow::Borrowed("double"),
            FieldType::F64 => Cow::Borrowed("double"),
            FieldType::Decimal => Cow::Borrowed("decimal"),
            FieldType::Date => Cow::Borrowed("String"),
            FieldType::DateTime => Cow::Borrowed("DateTime"),
            FieldType::Enum(enum_def) => Cow::Owned(enum_def.name().to_string()),
            FieldType::Vec(inner) => Cow::Owned("List<".to_owned() + self.field_type_to_result_type(inner.field_type()).as_ref() + ">"),
            FieldType::HashMap(_) => panic!(),
            FieldType::BTreeMap(_) => panic!(),
            FieldType::Object(name) => Cow::Owned(name.to_string()),
        }
    }

    fn generated_type_to_vec<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned("List<".to_owned() + generated_type.as_ref() + ">")
    }

    fn generated_type_to_enumerate<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned("List<".to_owned() + generated_type.as_ref() + ">")
    }

    fn generated_type_to_optional<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned(generated_type.as_ref().to_owned() + "?")
    }

    fn generated_type_to_or_null<'a>(&self, _generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Borrowed("dynamic")
    }

    fn action_result_type<'a>(&self, action: Action, model_name: &'a str) -> Cow<'a, str> {
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
            COUNT_HANDLER => format!("Response<int>"),
            AGGREGATE_HANDLER => format!("Response<dynamic>"),
            GROUP_BY_HANDLER => format!("Response<List<Map<String, dynamic>>>"),
            SIGN_IN_HANDLER => format!("ResponseWithMeta<TokenInfo, {model_name}>"),
            IDENTITY_HANDLER => format!("Response<{model_name}>"),
            _ => unreachable!()
        })
    }

    fn number_type(&self) -> &'static str {
        "int"
    }

    fn bool_type(&self) -> &'static str {
        "bool"
    }
}
