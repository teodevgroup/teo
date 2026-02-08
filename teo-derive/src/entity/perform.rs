use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use darling::FromDeriveInput;
use quote::quote;
use syn::{Result, DeriveInput, parse_macro_input};
use crate::entity::types::EntityDef;

pub(crate) fn perform(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(
        _perform(input)
            .unwrap_or_else(|err| err.to_compile_error()))
}

fn _perform(input: DeriveInput) -> Result<TokenStream2> {
    let _opts = EntityDef::from_derive_input(&input)?;
    let output = quote! {

    };
    Ok(output)
}
