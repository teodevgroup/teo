use std::borrow::Cow;
use crate::gen::internal::server::outline::field_kind::FieldKind;

pub(in crate::gen) struct EnumVariant<'a> {
    pub(in crate::gen) name: &'a str,
    pub(in crate::gen) localized_name: &'a str,
    pub(in crate::gen) desc: &'a str,
}