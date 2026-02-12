use darling::{FromDeriveInput, FromMeta};
use syn::{Attribute, Ident, Path};

#[derive(Clone, FromMeta)]
pub(in crate::schema) struct SchemaEntityDef {
    path: Path
}

#[derive(FromDeriveInput, Clone)]
#[darling(attributes(teo), forward_attrs(allow, doc, cfg), supports(struct_unit))]
pub(in crate::schema) struct SchemaDef {
    pub(in crate::schema) ident: Ident,
    pub(in crate::schema) attrs: Vec<Attribute>,
    #[darling(multiple, rename = "entity")]
    pub(in crate::schema) entities: Vec<SchemaEntityDef>,
}
