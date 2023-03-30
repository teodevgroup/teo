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
    pub(super) name: &'a str,
    pub(super) localized_name: Cow<'a, str>,
    pub(super) docs: Cow<'a, str>,
    pub(super) field_type: Cow<'a, str>,
    pub(super) optional: bool,
    pub(super) kind: FieldKind,
}
