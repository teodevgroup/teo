use std::borrow::Cow;
use crate::gen::internal::client::outline::field::Field;
use crate::gen::internal::client::outline::class_kind::ClassKind;

/// # Class
///
/// Represents a generated class. This is type or interface in TypeScript, sometimes struct in
/// Swift, class for other languages. This is interpreted by underlying client generators. A class
/// represents an input, output or internal type.
///
/// Use this in the template engine to generate correct classes or types for the client or server.
pub(in crate::gen) struct Class<'a> {
    pub(in crate::gen) model_name: &'a str,
    pub(in crate::gen) localized_name: Cow<'a, str>,
    pub(in crate::gen) name_suffix: Cow<'a, str>,
    pub(in crate::gen) docs: Cow<'a, str>,
    pub(in crate::gen) kind: ClassKind,
    pub(in crate::gen) fields: Vec<Field<'a>>,
}
