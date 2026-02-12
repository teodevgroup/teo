use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Result, parse_macro_input};
use crate::schema::{generated::generate_impl_schema, types::SchemaDef};

pub(crate) fn perform(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(
        _perform(input)
            .unwrap_or_else(|err| err.to_compile_error()))
}

fn _perform(input: DeriveInput) -> Result<TokenStream2> {
    let opts = SchemaDef::from_derive_input(&input)?;
    let impl_schema = generate_impl_schema(opts)?;
    let output = quote! {
        #impl_schema
    };
    Ok(output)
}
