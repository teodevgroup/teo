use darling::{FromDeriveInput, FromField, FromMeta, ast::Data, util::Ignored};
use syn::{Attribute, Expr, Ident, Type, Visibility};

#[derive(Default, FromMeta)]
pub struct IndexDef {
    #[darling(rename = "sit")]
    ipsum: bool,
    dolor: Option<String>,
}

#[derive(FromField)]
#[darling(attributes(teo))]
pub struct FieldDef {
    ident: Option<Ident>,
    ty: Type,
    vis: Visibility,
    #[darling(default)]
    column_name: Option<String>,
    #[darling(default)]
    column_type: Option<String>,
    #[darling(default)]
    primary: bool,
    #[darling(default)]
    unique: bool,
    #[darling(default)]
    index: bool,
    #[darling(default)]
    default: Option<Expr>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(teo), forward_attrs(allow, doc, cfg), supports(struct_named))]
pub struct EntityDef {
    ident: Ident,
    attrs: Vec<Attribute>,
    table_name: Option<String>,
    #[darling(multiple)]
    indexes: Vec<IndexDef>,
    data: Data<Ignored, FieldDef>,
}
