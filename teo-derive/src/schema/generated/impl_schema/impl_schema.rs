use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;
#[cfg(feature = "mongo")]
use crate::schema::generated::impl_schema::mongo_table_defs::generate_mongo_table_defs;
use crate::schema::types::SchemaDef;

pub(in crate::schema) fn generate_impl_schema(opts: SchemaDef) -> Result<TokenStream> {
    let struct_ident = opts.ident.clone();
    #[cfg(feature = "mongo")]
    let mongo_tables_def = generate_mongo_table_defs(opts.clone())?;
    #[cfg(not(feature = "mongo"))]
    let mongo_table_def = quote! { };

    Ok(quote! {
        impl ::teo::types::Schema for #struct_ident {
            fn enum_defs() -> Vec<::teo::migration::EnumDef> {
                Vec::new()
            }
            #mongo_tables_def
        }
    })
}
