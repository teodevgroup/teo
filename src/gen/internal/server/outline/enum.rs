
use crate::gen::internal::server::outline::enum_variant::EnumVariant;

pub(in crate::gen) struct Enum<'a> {
    pub(in crate::gen) name: &'a str,
    pub(in crate::gen) variants: Vec<EnumVariant<'a>>,
    pub(in crate::gen) localized_name: &'a str,
    pub(in crate::gen) desc: &'a str,
}

