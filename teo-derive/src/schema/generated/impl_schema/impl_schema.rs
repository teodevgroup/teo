use proc_macro2::TokenStream;
use syn::Result;
use crate::schema::types::SchemaDef;

pub(in crate::schema) fn generate_impl_schema(opts: SchemaDef) -> Result<TokenStream> {
    panic!()
}
