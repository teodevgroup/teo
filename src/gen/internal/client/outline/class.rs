use std::borrow::Cow;
use std::cmp::Ordering;
use itertools::Itertools;
use crate::gen::internal::client::outline::field::Field;
use crate::gen::internal::client::outline::class_kind::ClassKind;

/// # Class
///
/// Represents a generated class. This is type or interface in TypeScript, sometimes struct in
/// Swift, class for other languages. This is interpreted by underlying client generators. A class
/// represents an input, output or internal type.
///
/// Use this in the template engine to generate correct classes or types for the client or old_server.
pub(in crate::gen) struct Class<'a> {
    pub(in crate::gen) model_name: &'a str,
    pub(in crate::gen) localized_name: Cow<'a, str>,
    pub(in crate::gen) name_suffix: Cow<'a, str>,
    pub(in crate::gen) docs: Cow<'a, str>,
    pub(in crate::gen) kind: ClassKind,
    pub(in crate::gen) fields: Vec<Field<'a>>,
}

impl<'a> Class<'a> {
    pub(in crate::gen) fn fields_optional_at_last(&'a self) -> Vec<&'a Field<'a>> {
        self.fields.iter().sorted_by(|a, _b| if a.optional { Ordering::Greater } else { Ordering::Less }).collect()
    }

    pub(in crate::gen) fn name(&'a self) -> String {
        self.model_name.to_owned() + self.name_suffix.as_ref()
    }

    pub(in crate::gen) fn joined_enum_variant_names(&'a self) -> String {
        self.fields.iter().map(|f| format!("\"{}\"", f.name)).join(" | ")
    }
}
