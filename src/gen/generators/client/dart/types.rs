use std::borrow::Cow;
use crate::core::action::{Action};
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::gen::internal::type_lookup::TypeLookup;

pub(crate) struct DartTypes { }

impl DartTypes {
    pub(crate) fn new() -> Self { Self { } }
}

impl TypeLookup for DartTypes {
    fn field_type_to_filter_type<'a>(&self, _field_type: &'a FieldType, _optional: bool) -> Cow<'a, str> {
        todo!()
    }

    fn field_type_to_create_type<'a>(&self, _field_type: &'a FieldType, _optional: bool) -> Cow<'a, str> {
        todo!()
    }

    fn field_type_to_update_type<'a>(&self, _field_type: &'a FieldType, _optional: bool) -> Cow<'a, str> {
        todo!()
    }

    fn field_type_to_result_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
        let base: Cow<str> = match field_type {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => Cow::Borrowed("String"),
            FieldType::String => Cow::Borrowed("String"),
            FieldType::Bool => Cow::Borrowed("bool"),
            FieldType::I32 => Cow::Borrowed("int"),
            FieldType::I64 => Cow::Borrowed("long"),
            FieldType::F32 => Cow::Borrowed("float"),
            FieldType::F64 => Cow::Borrowed("double"),
            FieldType::Decimal => Cow::Borrowed("decimal"),
            FieldType::Date => Cow::Borrowed("DateOnly"),
            FieldType::DateTime => Cow::Borrowed("DateTime"),
            FieldType::Enum(enum_def) => Cow::Owned(enum_def.name().to_string()),
            FieldType::Vec(inner) => Cow::Owned(self.field_type_to_result_type(inner.field_type(), inner.is_optional()).as_ref().to_owned() + "[]"),
            FieldType::HashMap(_) => panic!(),
            FieldType::BTreeMap(_) => panic!(),
            FieldType::Object(name) => Cow::Owned(name.to_string()),
        };
        if optional {
            Cow::Owned(base.as_ref().to_owned() + "?")
        } else {
            base
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
