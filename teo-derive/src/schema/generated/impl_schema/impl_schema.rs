use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;
#[cfg(feature = "mongo")]
use crate::schema::generated::impl_schema::mongo_table_defs::generate_mongo_table_defs;
#[cfg(feature = "mysql")]
use crate::schema::generated::impl_schema::mysql_table_defs::generate_mysql_table_defs;
#[cfg(feature = "postgres")]
use crate::schema::generated::impl_schema::postgres_table_defs::generate_postgres_table_defs;
#[cfg(feature = "sqlite")]
use crate::schema::generated::impl_schema::sqlite_table_defs::generate_sqlite_table_defs;
use crate::schema::types::SchemaDef;

pub(in crate::schema) fn generate_impl_schema(opts: SchemaDef) -> Result<TokenStream> {
    let struct_ident = opts.ident.clone();
    #[cfg(feature = "mongo")]
    let mongo_table_defs = generate_mongo_table_defs(opts.clone())?;
    #[cfg(not(feature = "mongo"))]
    let mongo_table_defs = quote! { };
    #[cfg(feature = "mysql")]
    let mysql_table_defs = generate_mysql_table_defs(opts.clone())?;
    #[cfg(not(feature = "mysql"))]
    let mysql_table_defs = quote! { };
    #[cfg(feature = "postgres")]
    let postgres_table_defs = generate_postgres_table_defs(opts.clone())?;
    #[cfg(not(feature = "postgres"))]
    let postgres_table_defs = quote! { };
    #[cfg(feature = "sqlite")]
    let sqlite_table_defs = generate_sqlite_table_defs(opts.clone())?;
    #[cfg(not(feature = "sqlite"))]
    let sqlite_table_defs = quote! { };
    Ok(quote! {
        impl ::teo::types::Schema for #struct_ident {
            fn enum_defs() -> Vec<::teo::migration::EnumDef> {
                Vec::new()
            }
            #mongo_table_defs
            #mysql_table_defs
            #postgres_table_defs
            #sqlite_table_defs
        }
    })
}
