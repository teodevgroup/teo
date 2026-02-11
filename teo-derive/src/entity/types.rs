use darling::{Error, FromDeriveInput, FromField, FromMeta, Result, ast::Data, util::Ignored};
#[cfg(feature = "mongo")]
use proc_macro2::TokenStream;
use syn::{Attribute, Expr, Ident, Lit, LitStr, Type, Visibility};

#[derive(Clone, Copy)]
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

#[derive(FromMeta, Clone)]
pub(in crate::entity) struct IndexColumnDef {
    name: Ident,
    #[darling(default)]
    order: Option<IndexColumnOrder>
}

#[derive(Default, FromMeta, Clone)]
pub(in crate::entity) struct IndexDef {
    name: Option<String>,
    #[darling(multiple, rename = "column")]
    columns: Vec<IndexColumnDef>,
    #[darling(default)]
    unique: bool,
}

#[derive(Clone)]
pub(in crate::entity) enum ColumnType {
    LitStr(LitStr),
    Expr(Expr),
}

impl FromMeta for ColumnType {
    fn from_expr(expr: &Expr) -> Result<Self> {
        Ok(match expr {
            Expr::Lit(lit) => match &lit.lit {
                Lit::Str(str) => Self::LitStr(str.clone()),
                _ => Self::Expr(expr.clone()),
            },
            _ => Self::Expr(expr.clone()),
        })
    }
}

#[cfg(feature = "mongo")]
#[derive(Default, FromMeta, Clone)]
pub(in crate::entity) struct MongoFieldDef {
    #[darling(default)]
    pub(in crate::entity) column_type: Option<ColumnType>,
}

#[cfg(feature = "mysql")]
#[derive(Default, FromMeta, Clone)]
pub(in crate::entity) struct MySQLFieldDef {
    #[darling(default)]
    pub(in crate::entity) column_type: Option<ColumnType>,
}

#[cfg(feature = "postgres")]
#[derive(Default, FromMeta, Clone)]
pub(in crate::entity) struct PostgresFieldDef {
    #[darling(default)]
    pub(in crate::entity) column_type: Option<ColumnType>,
}

#[cfg(feature = "sqlite")]
#[derive(Default, FromMeta, Clone)]
pub(in crate::entity) struct SQLiteFieldDef {
    #[darling(default)]
    pub(in crate::entity) column_type: Option<ColumnType>,
}

#[derive(FromField, Clone)]
#[darling(attributes(teo))]
pub(in crate::entity) struct FieldDef {
    pub(in crate::entity) ident: Option<Ident>,
    pub(in crate::entity) ty: Type,
    pub(in crate::entity) vis: Visibility,
    #[darling(default)]
    pub(in crate::entity) column_name: Option<String>,
    #[darling(default)]
    pub(in crate::entity) primary: bool,
    #[darling(default)]
    pub(in crate::entity) auto_increment: bool,
    #[darling(default)]
    pub(in crate::entity) unique: bool,
    #[darling(default)]
    pub(in crate::entity) index: bool,
    #[darling(default)]
    pub(in crate::entity) default: Option<Expr>,
    #[cfg(feature = "mongo")]
    #[darling(default)]
    pub(in crate::entity) mongo: Option<MongoFieldDef>,
    #[cfg(feature = "mysql")]
    #[darling(default)]
    pub(in crate::entity) mysql: Option<MySQLFieldDef>,
    #[cfg(feature = "postgres")]
    #[darling(default)]
    pub(in crate::entity) postgres: Option<PostgresFieldDef>,
    #[cfg(feature = "sqlite")]
    #[darling(default)]
    pub(in crate::entity) sqlite: Option<SQLiteFieldDef>,
}

impl FieldDef {
    #[cfg(feature = "mongo")]
    pub(in crate::entity) fn mongo_column_type(&self) -> syn::Result<TokenStream> {
        if let Some(mongo) = &self.mongo && let Some(column_type) = &mongo.column_type {
            use quote::quote;
            match column_type {
                ColumnType::LitStr(lit_str) => {
                    use std::str::FromStr;
                    use teo_column_type::mongo::ColumnType;
                    use crate::entity::column_types::extended_column_type::ExtendedColumnType;
                    let column_type = ColumnType::from_str(&lit_str.value()).map_err(|_| {
                        syn::Error::new(lit_str.span(), "teo(mongo): invalid column type.")
                    })?;
                    Ok(column_type.to_token_stream())
                },
                ColumnType::Expr(expr) => Ok(quote! { #expr }),
            }
        } else {
            use teo_column_type::mongo::ColumnType;
            use crate::entity::column_types::extended_column_type::ExtendedColumnType;
            ColumnType::default_column_type(&self.ty)
        }
    }

    #[cfg(feature = "mysql")]
    pub(in crate::entity) fn mysql_column_type(&self) -> syn::Result<TokenStream> {
        if let Some(mysql) = &self.mysql && let Some(column_type) = &mysql.column_type {
            use quote::quote;
            match column_type {
                ColumnType::LitStr(lit_str) => {
                    use std::str::FromStr;
                    use teo_column_type::mysql::ColumnType;
                    use crate::entity::column_types::extended_column_type::ExtendedColumnType;
                    let column_type = ColumnType::from_str(&lit_str.value()).map_err(|_| {
                        syn::Error::new(lit_str.span(), "teo(mysql): invalid column type.")
                    })?;
                    Ok(column_type.to_token_stream())
                },
                ColumnType::Expr(expr) => Ok(quote! { #expr }),
            }
        } else {
            use teo_column_type::mysql::ColumnType;
            use crate::entity::column_types::extended_column_type::ExtendedColumnType;
            ColumnType::default_column_type(&self.ty)
        }
    }

    #[cfg(feature = "postgres")]
    pub(in crate::entity) fn postgres_column_type(&self) -> syn::Result<TokenStream> {
        if let Some(postgres) = &self.postgres && let Some(column_type) = &postgres.column_type {
            use quote::quote;
            match column_type {
                ColumnType::LitStr(lit_str) => {
                    use std::str::FromStr;
                    use teo_column_type::postgres::ColumnType;
                    use crate::entity::column_types::extended_column_type::ExtendedColumnType;
                    let column_type = ColumnType::from_str(&lit_str.value()).map_err(|_| {
                        syn::Error::new(lit_str.span(), "teo(postgres): invalid column type.")
                    })?;
                    Ok(column_type.to_token_stream())
                },
                ColumnType::Expr(expr) => Ok(quote! { #expr }),
            }
        } else {
            use teo_column_type::postgres::ColumnType;
            use crate::entity::column_types::extended_column_type::ExtendedColumnType;
            ColumnType::default_column_type(&self.ty)
        }
    }

    #[cfg(feature = "sqlite")]
    pub(in crate::entity) fn sqlite_column_type(&self) -> syn::Result<TokenStream> {
        if let Some(sqlite) = &self.sqlite && let Some(column_type) = &sqlite.column_type {
            use quote::quote;
            match column_type {
                ColumnType::LitStr(lit_str) => {
                    use std::str::FromStr;
                    use teo_column_type::sqlite::ColumnType;
                    use crate::entity::column_types::extended_column_type::ExtendedColumnType;
                    let column_type = ColumnType::from_str(&lit_str.value()).map_err(|_| {
                        syn::Error::new(lit_str.span(), "teo(sqlite): invalid column type.")
                    })?;
                    Ok(column_type.to_token_stream())
                },
                ColumnType::Expr(expr) => Ok(quote! { #expr }),
            }
        } else {
            use teo_column_type::sqlite::ColumnType;
            use crate::entity::column_types::extended_column_type::ExtendedColumnType;
            ColumnType::default_column_type(&self.ty)
        }
    }
}

#[derive(FromDeriveInput, Clone)]
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
