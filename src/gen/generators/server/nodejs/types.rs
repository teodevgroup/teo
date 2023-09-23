use std::borrow::Cow;
use crate::core::action::{Action};
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::gen::internal::server_type_lookup::ServerTypeLookup;
use crate::gen::internal::type_lookup::TypeLookup;

pub(crate) struct NodeJSTypes { }

impl NodeJSTypes {
    pub(crate) fn new() -> Self { Self { } }
}

impl TypeLookup for NodeJSTypes {
    fn field_type_to_filter_type<'a>(&self, _field_type: &'a FieldType, _nullable: bool) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn field_type_to_filter_with_aggregates_type<'a>(&self, _field_type: &'a FieldType, _nullable: bool) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn field_type_to_create_type<'a>(&self, _field_type: &'a FieldType, _nullable: bool) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn field_type_to_update_type<'a>(&self, _field_type: &'a FieldType, _nullable: bool) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn field_type_to_result_type<'a>(&self, field_type: &'a FieldType) -> Cow<'a, str> {
        let base = match field_type {
            FieldType::ObjectId => "string".to_string(),
            FieldType::String => "string".to_string(),
            FieldType::Date => "Date".to_string(),
            FieldType::DateTime => "Date".to_string(),
            FieldType::Bool => "boolean".to_string(),
            FieldType::I32 | FieldType::I64 | FieldType::F32 | FieldType::F64 => "number".to_string(),
            FieldType::Decimal => "Decimal".to_string(),
            FieldType::Vec(inner) => self.field_type_to_result_type(inner.field_type()).to_string() + "[]",
            FieldType::Object(name) => name.to_string(),
            FieldType::Enum(_) => field_type.unwrap_enum().name().to_string(),
            _ => unreachable!(),
        };
        Cow::Owned(base)
    }

    fn generated_type_to_vec<'a>(&self, _generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn generated_type_to_enumerate<'a>(&self, _generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn generated_type_to_optional<'a>(&self, _generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn generated_type_to_or_null<'a>(&self, _generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn action_result_type<'a>(&self, _action: Action, _model_name: &'a str) -> Cow<'a, str> {
        Cow::Borrowed("NotImplemented")
    }

    fn number_type(&self) -> &'static str {
        "number"
    }

    fn bool_type(&self) -> &'static str {
        "boolean"
    }
}

impl ServerTypeLookup for NodeJSTypes {
    fn input_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
        let base = match field_type {
            FieldType::ObjectId => "string".to_string(),
            FieldType::String => "string".to_string(),
            FieldType::Date => "Date".to_string(),
            FieldType::DateTime => "Date".to_string(),
            FieldType::Bool => "boolean".to_string(),
            FieldType::I32 | FieldType::I64 | FieldType::F32 | FieldType::F64 => "number".to_string(),
            FieldType::Decimal => "Decimal".to_string(),
            FieldType::Vec(inner) => self.field_type_to_result_type(inner.field_type()).to_string() + "[]",
            FieldType::Object(name) => name.to_string(),
            FieldType::Enum(_) => field_type.unwrap_enum().name().to_string(),
            _ => unreachable!(),
        };
        return if optional {
            Cow::Owned(base + " | null")
        } else {
            Cow::Owned(base)
        }
    }

    fn output_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
        self.input_type(field_type, optional)
    }

    fn wrap_in_vec<'a>(&self, original: &str) -> Cow<'a, str> {
        Cow::Owned(original.to_owned() + "[]")
    }

    fn wrap_in_optional<'a>(&self, original: &str) -> Cow<'a, str> {
        Cow::Owned(original.to_owned() + " | null")
    }
}