use std::borrow::Cow;
use crate::core::field::r#type::FieldType;

pub(crate) trait ServerTypeLookup {
    fn input_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str>;
    fn output_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str>;
    fn wrap_in_vec<'a>(&self, original: &str) -> Cow<'a, str>;
    fn wrap_in_optional<'a>(&self, original: &str) -> Cow<'a, str>;
}
