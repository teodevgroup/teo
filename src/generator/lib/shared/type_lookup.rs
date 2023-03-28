use std::borrow::Cow;
use crate::core::action::Action;
use crate::core::field::r#type::FieldType;

pub(crate) trait TypeLookup {
    fn field_type_to_filter_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str>;
    fn field_type_to_create_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str>;
    fn field_type_to_update_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str>;
    fn field_type_to_result_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str>;
    fn generated_type_to_vec<'a>(&self, generated_type: &'a str) -> Cow<'a, str>;
    fn action_result_type<'a>(&self, action: Action, model_name: &'a str) -> Cow<'a, str>;
}
