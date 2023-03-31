use std::borrow::Cow;
use crate::core::action::{Action, AGGREGATE_HANDLER, COUNT_HANDLER, CREATE_HANDLER, CREATE_MANY_HANDLER, DELETE_HANDLER, DELETE_MANY_HANDLER, FIND_FIRST_HANDLER, FIND_MANY_HANDLER, FIND_UNIQUE_HANDLER, GROUP_BY_HANDLER, IDENTITY_HANDLER, SIGN_IN_HANDLER, UPDATE_HANDLER, UPDATE_MANY_HANDLER, UPSERT_HANDLER};
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::gen::internal::type_lookup::TypeLookup;

pub(crate) struct KotlinTypes { }

impl KotlinTypes {
    pub(crate) fn new() -> Self { Self { } }
}

impl TypeLookup for KotlinTypes {
    fn field_type_to_filter_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
        todo!()
    }

    fn field_type_to_create_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
        todo!()
    }

    fn field_type_to_update_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
        todo!()
    }

    fn field_type_to_result_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
        todo!()
    }

    fn generated_type_to_vec<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        todo!()
    }

    fn generated_type_to_enumerate<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        todo!()
    }

    fn generated_type_to_optional<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        todo!()
    }

    fn generated_type_to_or_null<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        todo!()
    }

    fn action_result_type<'a>(&self, action: Action, model_name: &'a str) -> Cow<'a, str> {
        todo!()
    }

    fn number_type(&self) -> &'static str {
        todo!()
    }

    fn bool_type(&self) -> &'static str {
        todo!()
    }
}
