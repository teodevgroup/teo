use std::borrow::Cow;
use crate::core::action::{Action};
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::gen::internal::type_lookup::TypeLookup;

pub(crate) struct KotlinTypes { }

impl KotlinTypes {
    pub(crate) fn new() -> Self { Self { } }
}

impl TypeLookup for KotlinTypes {
    fn field_type_to_filter_type<'a>(&self, _field_type: &'a FieldType, _optional: bool) -> Cow<'a, str> {
        todo!()
    }

    fn field_type_to_filter_with_aggregates_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        todo!()
    }

    fn field_type_to_create_type<'a>(&self, _field_type: &'a FieldType, _optional: bool) -> Cow<'a, str> {
        todo!()
    }

    fn field_type_to_update_type<'a>(&self, _field_type: &'a FieldType, _optional: bool) -> Cow<'a, str> {
        todo!()
    }

    fn field_type_to_result_type<'a>(&self, field_type: &'a FieldType) -> Cow<'a, str> {
        match field_type {
            FieldType::ObjectId => Cow::Borrowed("String"),
            FieldType::Bool => Cow::Borrowed("Boolean"),
            FieldType::I32 => Cow::Borrowed("Int"),
            FieldType::I64 => Cow::Borrowed("Long"),
            FieldType::F32 => Cow::Borrowed("Float"),
            FieldType::F64 => Cow::Borrowed("Double"),
            FieldType::Decimal => Cow::Borrowed("Decimal"),
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

    fn generated_type_to_vec<'a>(&self, _generated_type: Cow<'a, str>) -> Cow<'a, str> {
        todo!()
    }

    fn generated_type_to_enumerate<'a>(&self, _generated_type: Cow<'a, str>) -> Cow<'a, str> {
        todo!()
    }

    fn generated_type_to_optional<'a>(&self, _generated_type: Cow<'a, str>) -> Cow<'a, str> {
        todo!()
    }

    fn generated_type_to_or_null<'a>(&self, _generated_type: Cow<'a, str>) -> Cow<'a, str> {
        todo!()
    }

    fn action_result_type<'a>(&self, _action: Action, _model_name: &'a str) -> Cow<'a, str> {
        todo!()
    }

    fn number_type(&self) -> &'static str {
        todo!()
    }

    fn bool_type(&self) -> &'static str {
        todo!()
    }
}
