use crate::gen::internal::server::outline::class_field::ClassField;

pub(in crate::gen) struct Class<'a> {
    pub(in crate::gen) name: &'a str,
    pub(in crate::gen) fields: Vec<ClassField<'a>>,
    pub(in crate::gen) localized_name: &'a str,
    pub(in crate::gen) desc: &'a str,
}
