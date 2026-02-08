use darling::{Error, FromDeriveInput, FromField, FromMeta, Result, ast::Data, util::Ignored};
use syn::{Attribute, Expr, Ident, Type, Visibility};

pub(super) enum IndexColumnOrder {
    Asc,
    Desc,
}

impl Default for IndexColumnOrder {
    fn default() -> Self {
        Self::Asc
    }
}

impl FromMeta for IndexColumnOrder {
    fn from_string(value: &str) -> Result<Self> {
        Ok(match value {
            "asc" => Self::Asc,
            "desc" => Self::Desc,
            _ => Err(Error::unknown_value(value))?
        })
    }
}

#[derive(FromMeta)]
pub(super) struct IndexColumnDef {
    name: Ident,
    #[darling(default)]
    order: Option<IndexColumnOrder>
}

#[derive(Default, FromMeta)]
pub(super) struct IndexDef {
    name: Option<String>,
    #[darling(multiple, rename = "column")]
    columns: Vec<IndexColumnDef>,
    #[darling(default)]
    unique: bool,
}

#[derive(FromField)]
#[darling(attributes(teo))]
pub(super) struct FieldDef {
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
    auto_increment: bool,
    #[darling(default)]
    unique: bool,
    #[darling(default)]
    index: bool,
    #[darling(default)]
    default: Option<Expr>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(teo), forward_attrs(allow, doc, cfg), supports(struct_named))]
pub(super) struct EntityDef {
    ident: Ident,
    attrs: Vec<Attribute>,
    table_name: Option<String>,
    #[darling(multiple, rename = "index")]
    indexes: Vec<IndexDef>,
    data: Data<Ignored, FieldDef>,
}
