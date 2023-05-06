use std::borrow::Cow;
use crate::gen::internal::server::outline::field_kind::FieldKind;

pub(in crate::gen) struct ClassField<'a> {
    pub(in crate::gen) name: &'a str,
    pub(in crate::gen) kind: FieldKind,
    pub(in crate::gen) input_field_type: Cow<'a, str>,
    pub(in crate::gen) input_optional: bool,
    pub(in crate::gen) output_field_type: Cow<'a, str>,
    pub(in crate::gen) output_optional: bool,
    pub(in crate::gen) localized_name: Cow<'a, str>,
    pub(in crate::gen) desc: &'a str,
    pub(in crate::gen) getter: bool,
    pub(in crate::gen) setter: bool,
}