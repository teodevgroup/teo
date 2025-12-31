use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Error, ItemStruct, parse};

pub(crate) fn perform(input: TokenStream) -> TokenStream {
    TokenStream::from(
        _perform(input)
            .unwrap_or_else(|err| err.to_compile_error()))
}

fn _perform(input: TokenStream) -> Result<TokenStream2, Error> {
    let Ok(input) = parse::<ItemStruct>(input) else {
        Err(Error::new(
            Span::call_site(),
            "`Entity` can only be applied to struct.",
        ))?
    };
    let new_struct = quote! {
        struct NewStruct { }
    };
    Ok(quote! { #new_struct })
}
