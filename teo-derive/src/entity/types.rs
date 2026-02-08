use darling::{Error, FromDeriveInput, FromField, FromMeta, Result, ast::Data, util::Ignored};
use syn::{Attribute, Expr, Ident, Type, Visibility};

pub(in crate::entity) enum IndexColumnOrder {
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
pub(in crate::entity) struct IndexColumnDef {
    name: Ident,
    #[darling(default)]
    order: Option<IndexColumnOrder>
}

#[derive(Default, FromMeta)]
pub(in crate::entity) struct IndexDef {
    name: Option<String>,
    #[darling(multiple, rename = "column")]
    columns: Vec<IndexColumnDef>,
    #[darling(default)]
    unique: bool,
}

#[derive(FromField)]
#[darling(attributes(teo))]
pub(in crate::entity) struct FieldDef {
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
pub(in crate::entity) struct EntityDef {
    pub(in crate::entity) ident: Ident,
    pub(in crate::entity) attrs: Vec<Attribute>,
    pub(in crate::entity) table_name: Option<String>,
    #[darling(multiple, rename = "index")]
    pub(in crate::entity) indexes: Vec<IndexDef>,
    pub(in crate::entity) data: Data<Ignored, FieldDef>,
}

impl EntityDef {
    pub(in crate::entity) fn table_name(&self) -> String {
        self.table_name.clone().unwrap_or(self.ident.to_string())
    }
}
