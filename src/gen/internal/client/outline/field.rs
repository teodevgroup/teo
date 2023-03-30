use std::borrow::Cow;
use crate::gen::internal::client::outline::field_kind::FieldKind;

/// # Field
///
/// Represents a field in a generated class.
///
/// An empty `docs` means no documentation is present.
///
/// An empty kind is always used inside enum type classes.
pub(in crate::gen) struct Field<'a> {
    pub(in crate::gen) name: &'a str,
    pub(in crate::gen) localized_name: Cow<'a, str>,
    pub(in crate::gen) docs: Cow<'a, str>,
    pub(in crate::gen) field_type: Cow<'a, str>,
    pub(in crate::gen) optional: bool,
    pub(in crate::gen) kind: FieldKind,
}
