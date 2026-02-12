use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;
use crate::schema::types::SchemaDef;

pub(in crate::schema) fn generate_impl_schema(opts: SchemaDef) -> Result<TokenStream> {
    let struct_ident = opts.ident.clone();

    Ok(quote! {
        impl ::teo::types::Schema for #struct_ident {
            fn enum_defs() -> Vec<::teo::migration::EnumDef> {
                Vec::new()
            }
        }
    })
}
