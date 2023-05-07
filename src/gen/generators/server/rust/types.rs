use std::borrow::Cow;
use crate::core::action::{Action, AGGREGATE_HANDLER, COUNT_HANDLER, CREATE_HANDLER, CREATE_MANY_HANDLER, DELETE_HANDLER, DELETE_MANY_HANDLER, FIND_FIRST_HANDLER, FIND_MANY_HANDLER, FIND_UNIQUE_HANDLER, GROUP_BY_HANDLER, IDENTITY_HANDLER, SIGN_IN_HANDLER, UPDATE_HANDLER, UPDATE_MANY_HANDLER, UPSERT_HANDLER};
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::gen::internal::server_type_lookup::ServerTypeLookup;
use crate::gen::internal::type_lookup::TypeLookup;

pub(crate) struct RustTypes { }

impl RustTypes {
    pub(crate) fn new() -> Self { Self { } }
}

impl TypeLookup for RustTypes {
    fn field_type_to_filter_type<'a>(&self, _field_type: &'a FieldType, _nullable: bool) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn field_type_to_filter_with_aggregates_type<'a>(&self, _field_type: &'a FieldType, _nullable: bool) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn field_type_to_create_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn field_type_to_update_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn field_type_to_result_type<'a>(&self, field_type: &'a FieldType) -> Cow<'a, str> {
        match field_type {
            FieldType::ObjectId => Cow::Borrowed("ObjectId"),
            FieldType::Bool => Cow::Borrowed("bool"),
            FieldType::I32 => Cow::Borrowed("i32"),
            FieldType::I64 => Cow::Borrowed("i64"),
            FieldType::F32 => Cow::Borrowed("f32"),
            FieldType::F64 => Cow::Borrowed("f64"),
            FieldType::Decimal => Cow::Borrowed("Decimal"),
            FieldType::String => Cow::Borrowed("String"),
            FieldType::Date => Cow::Borrowed("NaiveDate"),
            FieldType::DateTime => Cow::Borrowed("DateTime<Utc>"),
            FieldType::Enum(enum_def) => Cow::Borrowed(enum_def.name()),
            FieldType::Vec(inner) => Cow::Owned("Vec<".to_owned() + if inner.is_optional() { "Option<" } else { "" } + self.field_type_to_result_type(inner.field_type()).as_ref() + if inner.is_optional() { ">" } else { "" } + ">"),
            FieldType::HashMap(_) => unreachable!(),
            FieldType::BTreeMap(_) => unreachable!(),
            FieldType::Object(_) => unreachable!(),
        }
    }

    fn generated_type_to_vec<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn generated_type_to_enumerate<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn generated_type_to_optional<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn generated_type_to_or_null<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn action_result_type<'a>(&self, action: Action, model_name: &'a str) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn number_type(&self) -> &'static str {
        "i32"
    }

    fn bool_type(&self) -> &'static str {
        "bool"
    }
}

impl ServerTypeLookup for RustTypes {
    fn input_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
        let original = match field_type {
            FieldType::ObjectId => Cow::Borrowed("ObjectId"),
            FieldType::String => Cow::Borrowed("String"),
            FieldType::Bool => Cow::Borrowed("bool"),
            FieldType::I32 => Cow::Borrowed("i32"),
            FieldType::I64 => Cow::Borrowed("i64"),
            FieldType::F32 => Cow::Borrowed("f32"),
            FieldType::F64 => Cow::Borrowed("f64"),
            FieldType::Decimal => Cow::Borrowed("BigDecimal"),
            FieldType::Date => Cow::Borrowed("NaiveDate"),
            FieldType::DateTime => Cow::Borrowed("DateTime<Utc>"),
            FieldType::Enum(e) => Cow::Borrowed(e.name()),
            FieldType::Vec(v) => Cow::Owned("Vec<".to_owned() + self.input_type(v.field_type(), v.is_optional()).as_ref() + ">"),
            FieldType::HashMap(_) => unreachable!(),
            FieldType::BTreeMap(_) => unreachable!(),
            FieldType::Object(_) => unreachable!(),
        };
        if optional {
            Cow::Owned("Option<".to_owned() + original.as_ref() + ">")
        } else {
            original
        }
    }

    fn output_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
        self.input_type(field_type, optional)
    }

    fn wrap_in_vec<'a>(&self, original: &str) -> Cow<'a, str> {
        Cow::Owned("Vec<".to_owned() + original + ">")
    }

    fn wrap_in_optional<'a>(&self, original: &str) -> Cow<'a, str> {
        Cow::Owned("Option<".to_owned() + original + ">")
    }
}