use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;
use crate::entity::types::EntityDef;
#[cfg(feature = "mongo")]
use crate::entity::generated::impl_entity::mongo_table_def::gen_mongo_table_def;
#[cfg(feature = "mysql")]
use crate::entity::generated::impl_entity::mysql_table_def::gen_mysql_table_def;
#[cfg(feature = "postgres")]
use crate::entity::generated::impl_entity::postgres_table_def::gen_postgres_table_def;
#[cfg(feature = "sqlite")]
use crate::entity::generated::impl_entity::sqlite_table_def::gen_sqlite_table_def;

pub(in crate::entity) fn generate_impl_entity(opts: EntityDef) -> Result<TokenStream> {
    let struct_ident = opts.ident.clone();
    #[cfg(feature = "mongo")]
    let mongo_table_def = gen_mongo_table_def(opts.clone())?;
    #[cfg(not(feature = "mongo"))]
    let mongo_table_def = quote! { };
    #[cfg(feature = "mysql")]
    let mysql_table_def = gen_mysql_table_def(opts.clone())?;
    #[cfg(not(feature = "mysql"))]
    let mysql_table_def = quote! { };
    #[cfg(feature = "postgres")]
    let postgres_table_def = gen_postgres_table_def(opts.clone())?;
    #[cfg(not(feature = "postgres"))]
    let postgres_table_def = quote! { };
    #[cfg(feature = "sqlite")]
    let sqlite_table_def = gen_sqlite_table_def(opts.clone())?;
    #[cfg(not(feature = "sqlite"))]
    let sqlite_table_def = quote! { };
    Ok(quote! {
        impl ::teo::types::Entity for #struct_ident {
            #mongo_table_def
            #mysql_table_def
            #postgres_table_def
            #sqlite_table_def
        }
    })
}
