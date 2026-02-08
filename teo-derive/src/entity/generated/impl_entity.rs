use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;
use crate::entity::types::EntityDef;

pub(in super::super) fn generate_impl_entity(opts: EntityDef) -> Result<TokenStream> {
    let table_name = opts.table_name();
    let struct_ident = opts.ident;
    Ok(quote! {
        impl ::teo::types::Entity for #struct_ident {
            fn table_def() -> ::teo::migration::TableDef {
                ::teo::migration::TableDef {
                    name: #table_name,
                    columns: Vec::new(),
                    indexes: Vec::new()
                }
            }
        }
    })
}
