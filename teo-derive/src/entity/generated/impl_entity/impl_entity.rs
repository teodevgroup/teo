use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;
use crate::entity::types::EntityDef;

pub(crate) fn generate_impl_entity(opts: EntityDef) -> Result<TokenStream> {
    let struct_ident = opts.ident;
    Ok(quote! {
        impl ::teo::types::Entity for #struct_ident {

        }
    })
}
